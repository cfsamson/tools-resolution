# Resolution

Simple tool that prints the resolution based on exif data found on image files.

Most programs doesn't calculate the resolution and loading the images into a
special program just to see how many megapixels the image file has is just
overkill.

Once installed this simply reads any files in the current folder and list the
file name and calculates the number of megapixels of the fil.

Only tested on Jpeg files so far.

## Installation

Clone the repo and run:

```
cargo install --path .
```

## Usage

Naviage to the folder with images and simply write:

```
resultion
```