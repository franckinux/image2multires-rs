The function of this program is to convert a single image into tiles in multires
file scheme. In the context of panoramic images production, images are cube
faces and are square-shaped.

This tool is not limited to square-shaped images, and can be used on
rectangle-shaped images as well.

This work has been inspired by Pannellum
(https://github.com/mpetroff/pannellum) and dzi (https://github.com/n-k/dzi)
projects.

I made the choice of a tool that makes a unique operation. So it does not
iterate over the 6 faces of the cube, nor does it generate a configuration file
for any panorama viewer. For these purposes, you can use a script of your own.
An example shell script `cube2tiles.sh` is provided in the `scripts` directory.

I didn't find any specification of the multires tile format used by Pannellum
viewer. The Python script `generate.py` in the test directory serves this
propose. This is a modified version of the original script from Pannellum: the
convertion from equirectangular to cube faces has been removed.

As a memo for myself, in the hope that it will be useful to others, an
equirectangular image can the converted into cube faces using ffmpeg:

    ffmpeg -i pano.tif -vf \
    "v360=equirect:c6x1:w=24000:h=4000:interp=lanczos,untile=6x1" \
    faces/face_%d.png

where $h$ is the face size and $w = 6 \times h$. The convertion is really fast !

# Usage

```
Generate multires tiles from an image

Usage: image2multires [OPTIONS] [image]

Arguments:
  [image]

Options:
  -p, --png                    Set tile image format to png instead of default jpg
  -s, --tilesize <tile-size>   Set tile image size [default: 512]
  -d, --directory <directory>  Set output directory of tile image files [default: output]
  -h, --help                   Print help
  -V, --version                Print version
```

# My workflow for generating a parorama

This is a contextualized explanation on when this tool is used. The steps are
the following:

1. Stitch the images to produce an equirectangular image using the image
   stitcher of your choice (hugin, ...);
2. Convert the equirectangular image into 6 cube faces;
3. Delete the original equirectangular image;
4. Do some changes on the down image to have a nadir without foreign objects,
   artifacts or holes (gimp + nona);
5. Generate tiles from the cube faces. **This is where this program operates**;
6. Upload the tiles to a web server using the protocol of your choice (ftp,
   ...);
7. Write and upload a html page to execute the Pannellum panorama viewer.
