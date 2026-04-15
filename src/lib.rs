use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use image::{
    DynamicImage,
    GenericImageView,
    ImageError,
    imageops::{FilterType, resize},
    ImageReader
};


#[derive(thiserror::Error, Debug)]
pub enum TilingError {
    #[error("Unsupported source image: {0}")]
    UnsupportedSourceImage(String),
    #[error("Unexpected error")]
    UnexpectedError,
    #[error("Unsupported source image: {0}")]
    ImageError(#[from] ImageError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

pub type MultiresResult<T, E = TilingError> = Result<T, E>;

/// A tile creator, this struct and associated functions implement the Multires tiler
#[derive(Debug)]
pub struct TileCreator {
    /// path of destination directory where tiles will be stored
    dest_path: PathBuf,
    /// source image
    image: DynamicImage,
    /// size of individual tiles in pixels
    tile_size: u32,
    /// horizontal size in pixels
    x_size: u32,
    /// vertical size in pixels
    y_size: u32,
    /// total number of levels of tiles
    levels: u32,
    /// select png image format
    png: bool,
    /// Prefix of tile filename
    prefix: String,
}


impl TileCreator {
    pub fn new_from_image_path(
        image_path: PathBuf, dest_path: PathBuf, tile_size: u32, png: bool
    ) -> MultiresResult<Self> {
        let file = File::open(image_path.clone())?;
        let reader = BufReader::new(file);
        let mut image_reader = ImageReader::new(reader).with_guessed_format()?;
        image_reader.no_limits();
        let im = image_reader.decode()?;
        let (x_size, y_size) = im.dimensions();

        let prefix = Path::new(image_path.file_name().unwrap()).file_stem().unwrap().to_str().unwrap().to_string();

        let tile_size = tile_size.min(x_size).min(y_size);
        let size = x_size.min(y_size);
        let mut levels: u32 = (size as f64 / tile_size as f64).log2().ceil() as u32 + 1;
        if (size as f64 / 2u32.pow(levels - 2) as f64).round() as u32 == tile_size {
            levels -= 1  // Handle edge case
        }

        Ok(Self {
            dest_path,
            image: im,
            tile_size,
            x_size,
            y_size,
            levels,
            png,
            prefix,
        })
    }

    /// Create Multires tiles
    pub fn create_tiles(&mut self) -> MultiresResult<()> {
        let mut x_size = self.x_size;
        let mut y_size = self.y_size;
        for level in (1..=self.levels).rev() {
            let p = self.dest_path.join(level.to_string());
            std::fs::create_dir_all(&p)?;

            let x_tiles = (x_size as f64 / self.tile_size as f64).ceil() as u32;
            let y_tiles = (y_size as f64 / self.tile_size as f64).ceil() as u32;
            if level < self.levels {
                self.image = image::DynamicImage::ImageRgba8(
                    resize(&self.image, x_size, y_size, FilterType::Triangle)
                );
            }
            for i in 0..y_tiles {
                for j in 0..x_tiles {
                    let left = j * self.tile_size;
                    let upper = i * self.tile_size;
                    let width = if left + self.tile_size >= self.x_size {
                        self.x_size - left
                    } else {
                        self.tile_size
                    };
                    let height = if upper + self.tile_size >= self.y_size {
                        self.y_size - upper
                    } else {
                        self.tile_size
                    };
                    let tile_image = self.image.crop_imm(left, upper, width, height);
                    let extension = if self.png { "png" } else { "jpg" };

                    let tile_path = self.dest_path.join(level.to_string()).join(format!("{}{}_{}.{extension}", self.prefix, i, j));
                    tile_image.save(tile_path)?;
                }
            }

            x_size = x_size / 2;
            y_size = y_size / 2;
        }
        Ok(())
    }
}
