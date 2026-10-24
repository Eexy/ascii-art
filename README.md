# ASCII ART

The purpose of this project is to create a CLI program that can convert an image to ASCII art

## What's ASCII Art 

ASCII Art is a way of recreating images with ascii characters

## How ASCII Art program work

An ASCII Art program follow a specific pipeline

```
Load the image -> fix Exif rotation (JPEG only) -> Resize based on the width -> Grayscale
```

### 1. Loading the image

Loading the image is without the surprise the first step of the pipeline. It's a two steps operation

- Take the path from the user (we will accept the path as argument of the program)
- Load the image in the program with crate `image`

#### 1. Using crate `image`

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

In our case we will directly use the `grayscale` method provided by the `image` crate
