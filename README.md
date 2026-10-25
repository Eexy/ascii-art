# ASCII ART

The purpose of this project is to create a CLI program that can convert an image to ASCII art

## What's ASCII Art 

ASCII Art is a way of recreating images with ascii characters

## How ASCII Art program work

An ASCII Art program follow a specific pipeline

```
Load the image -> fix Exif rotation (JPEG only) -> Resize based on the width -> Grayscale  -> Map pixel to char (and gamma correction) -> Render
```

### 1. Loading the image

Loading the image is without the surprise the first step of the pipeline. It's a two steps operation

- Take the path from the user (we will accept the path as argument of the program)
- Load the image in the program with crate `image`

#### 1.1 Using crate `image`

To load the image we will use the `image` crate

This crate provide two main type

- `ImageReader` : this allow use to load a file or bytes, detects the format and decodes it into pixels
- `ImageBuffer` : this holds the decoded pixels in memory in a 2D grid that we can read and modify

### 2. Correcting the EXIF rotation

The EXIF rotation  is a metadata tag on an JPEG image that indicates how the camera was positioned when taking the shot and how the image should be rotated or flipped when displayed

The EXIF rotation tag is a numeric value that has 8 possible values

| value | transformation needed to display correctly |
|---|-----|
| 1 | None (normal) |
| 2 | Flip horizontally |
| 3 | Rotate 180 deg|
|4| Flip vertically |
|5| Transpose (flip horizontal + rotate 90 deg CCW)  |
|6| Rotate 90 deg CW|
|7| Transverse (flip horizontal + rotate 90 deg CW) |
|8| Rotate 90 deg CCW (270 deg CW) |

**CW** : Clockwise

**CCW** : Counter clockwise

It's important to handle it if we want our image to be displayed correctly

### 3. Resizing the image

ASCII characters are generally 2 times taller than they are wide (it varies by font but it's usually ~0.4-0.5 width /height ratio)

Because of that we need to resize the image to avoid getting a stretched result

### 4. Grayscale

ASCII art work based of the luminosity of an area. Based of a luminosity we will set a character to represent it. To do that we need to get the grayscale version of the image we can do it manually with the following formula

```
gray = 0.299*R + 0.587*G + 0.114*B
```

Where `R` `G` and `B` are the value of of red, green and blue of each pixel

This formula is called the **perceptual luminance**. We can see that before each value there is a different weight. This is because the human eye is more sensitive to green that red or blue

In our case we will directly use the `luma8` method provided by the `image` crate


### 5. Map pixel to char

Once we get have associated each pixel to its grayscale value we can transpose it to an ASCII char to render it

Before rendering it we need to define a `palette`. The palette is a collection of char that will be use to render the luminosity of each pixel. Bigger is the palette the more we can represent different level of luminosity 
and go into details

We will use this palette

```
palette = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,"^'.  " 
```

Once we have defined the palette we also need to define the order to handle light / dark theme

In a light mode bigger character represent  pixels that have the lest luminosity while in dark mode it's the opposite. This is due to the fact that each character represent the "ink" of the the pixel.
So in light mode the ink is dark because we have white background so we need more ink and in dark mode it's the opposite

Now that we have the palette we need to map our grayscale value to a specific character to do that we get take the grayscale value divided by 255 and multiply by the length of our palette to get the associated character index

The division by 255 nomalize the pixel value that is between [0, 255] into a ratio which is between [0, 1]. The more the ratio is close to 1 the more the pixel is bright


#### 5.1 Gamma correction (optional)


Before associating a pixel brightness level to a specific ASCII char we can apply a gamma correction. The gamma correction help fix dark, muddy or washed-out detail. In our case in an ASCII art dark and middle tones are often
crushed and they finished by being displayed with the same characters and it becomes difficult to read the image

This gamma correction also come from the fact that the human eye dont perceive the light linearly. We are more sensible from light variation in dark tone that variation form light tone

Here is the ranges we used to select the value for the gamma correction

- gamma < 1 (0.5 ex): lightens dark mid-tones and brings out hidden details in shadows
- gamma = 1 : we change nothing
- gama > 1 (generally 2.0-2.5) : darkens mid-tones and increases contrast, which helps clean up overexposed or bright images

In ASCII art we generally use a gamma correction that is between 0.4 and 0.5


#### 5.2 Use gamma correction to optimize space

Because the human eye is less sensible to light variable in light tone there is no  need to store the light variation linearly. For those cases we can can apply a gamma correction to compress the image


### 6. Render

Now that we have associated each pixel to a char we can simply render it. For a simple render in the terminal we just need to store each pixel into a string and render the string at the end
