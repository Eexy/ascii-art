use std::{
    cmp,
    env::{self, Args},
    path::PathBuf,
};

use image::{DynamicImage, ImageDecoder, ImageReader, imageops::FilterType};

const VALID_FILE_EXTENSION: [&str; 3] = ["jpeg", "jpg", "png"];

fn get_file_path(mut args: Args) -> Result<PathBuf, String> {
    args.next();
    let raw_path = args
        .next()
        .ok_or_else(|| "You need to provide a path to the image".to_string())?;

    let current_dir =
        env::current_dir().map_err(|err| format!("unable to get current directory path: {err}"))?;

    let path = current_dir.join(PathBuf::from(raw_path));
    let is_valid_file_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| VALID_FILE_EXTENSION.contains(&ext.to_lowercase().as_str()));

    if !is_valid_file_extension {
        return Err(format!(
            "Invalid file extension. file must be {}",
            VALID_FILE_EXTENSION.join(", "),
        ));
    }

    Ok(path)
}

fn load_image_and_fix_exif(path: PathBuf) -> Result<DynamicImage, String> {
    let mut decoder = ImageReader::open(path)
        .map_err(|err| format!("unable to load image: {err}"))?
        .into_decoder()
        .map_err(|err| format!("unable to creader decoder {err}"))?;

    let exif_orientation = decoder
        .orientation()
        .map_err(|err| format!("unable to extract orientation from image: {err}"))?;
    let mut image = DynamicImage::from_decoder(decoder)
        .map_err(|err| format!("unable to create image from decoded: {err}"))?;
    image.apply_orientation(exif_orientation);

    Ok(image)
}

const IMAGE_ASCII_WIDTH: u32 = 120;
const CHAR_ASPECT_RATION: f64 = 0.45;

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "height is always positive and far below u32:MAX"
)]
fn resize_image(image: &DynamicImage) -> DynamicImage {
    let image_height = image.height();
    let image_width = image.width();
    let aspect_ratio = f64::from(image_height) / f64::from(image_width);

    let computed_height =
        (f64::from(IMAGE_ASCII_WIDTH) * aspect_ratio * CHAR_ASPECT_RATION).round() as u32;
    let target_height = cmp::max(1, computed_height);

    image.resize(IMAGE_ASCII_WIDTH, target_height, FilterType::Triangle)
}

const ASCII_CHARS: [char; 95] = [
    ' ', '`', '.', '-', '\'', '_', ',', ':', '~', '*', ';', '!', '"', '^', '/', '\\', '+', '=',
    'r', '>', '<', '|', '7', 'l', '(', ')', '?', 'L', 't', 'i', 'T', 'x', 'Y', 'v', 'c', 'z', 'J',
    '1', 'j', 's', 'f', '}', '[', ']', '{', 'I', 'u', 'n', 'y', 'F', '#', 'V', 'o', '4', 'C', '2',
    'k', 'X', 'e', 'h', '3', 'Z', 'a', 'P', 'A', '5', 'w', 'U', 'S', 'K', 'b', 'd', 'E', '6', 'H',
    'p', 'q', '9', 'G', 'm', '%', 'O', 'D', '8', '0', '&', 'R', 'N', 'g', 'M', 'W', '$', 'B', 'Q',
    '@',
];

const GAMMA_CORRECTION: f32 = 0.5;

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "ratio is in [0, 1] and ASCII_CHARS has 95 entries, so idx is in 0..=94"
)]
fn pixel_to_char(pixel_grayscale_value: u8) -> char {
    // let ratio = pixel_grayscale_value as f32 / 255.0;
    let gamma_corrected_pixel =
        ((f32::from(pixel_grayscale_value) / 255.0).powf(1.0 / GAMMA_CORRECTION) * 255.0).round();
    // let idx = (ratio * (ASCII_CHARS.len() as u8 - 1) as f32).round() as usize;
    let idx = ((gamma_corrected_pixel / 255.0) * (ASCII_CHARS.len() - 1) as f32).round() as usize;

    ASCII_CHARS[idx]
}

/*
 * TODO
 * separer le projet en different module
 * corriger les warnings
 * completer les tache todoist
 */
fn main() -> Result<(), String> {
    let path = get_file_path(env::args())?;
    println!("{}", path.display());
    let image = load_image_and_fix_exif(path)?;
    let resized_image = resize_image(&image);
    let grayscaled_image = resized_image.to_luma8();
    let mut ascii_art = String::new();

    for row in grayscaled_image.rows() {
        for pixel in row {
            ascii_art.push(pixel_to_char(pixel.0[0]));
        }
        ascii_art.push('\n');
    }

    println!("{}", ascii_art);

    Ok(())
}
