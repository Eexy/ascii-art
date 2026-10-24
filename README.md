# ASCII ART

The purpose of this project is to create a CLI program that can convert an image to ASCII art

## What's ASCII Art 

ASCII Art is a way of recreating images with ascii characters

## How ASCII Art program work

An ASCII Art program follow a specific pipeline

```
Load the image -> fix Exif rotation (JPEG only) -> Resize based on the width -> 
```

### 1. Loading the image

Loading the image is without the surprise the first step of the pipeline. It's a two step operation

- Take the path from the user (we will accept the path as argument of the program)
- Load the image
