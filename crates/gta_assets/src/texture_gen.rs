use image::{ImageBuffer, Rgba};
use std::fs;
use std::path::Path;

pub struct TextureGenerator;

impl TextureGenerator {
    /// Generates authentic Portland weathered red brick wall texture (256x256)
    pub fn generate_brick_texture<P: AsRef<Path>>(path: P) {
        let width = 256;
        let height = 256;
        let mut img = ImageBuffer::new(width, height);

        for y in 0..height {
            let row_idx = y / 16;
            let offset = if row_idx % 2 == 1 { 16 } else { 0 };
            let is_mortar_y = y % 16 == 0 || y % 16 == 1;

            for x in 0..width {
                let shifted_x = (x + offset) % width;
                let is_mortar_x = shifted_x % 32 == 0 || shifted_x % 32 == 1;

                if is_mortar_y || is_mortar_x {
                    // Mortar grey
                    img.put_pixel(x, y, Rgba([95, 90, 85, 255]));
                } else {
                    // Gritty brick variation
                    let noise = ((x * 13 + y * 37) % 25) as u8;
                    let r = 145u8.saturating_sub(noise);
                    let g = 60u8.saturating_sub(noise / 2);
                    let b = 45u8.saturating_sub(noise / 2);
                    img.put_pixel(x, y, Rgba([r, g, b, 255]));
                }
            }
        }

        let _ = fs::create_dir_all(path.as_ref().parent().unwrap());
        let _ = img.save(path);
    }

    /// Generates Liberty City asphalt road with yellow center dashed line (256x256)
    pub fn generate_road_texture<P: AsRef<Path>>(path: P) {
        let width = 256;
        let height = 256;
        let mut img = ImageBuffer::new(width, height);

        for y in 0..height {
            let is_dash = (y % 64) < 36;
            for x in 0..width {
                let is_center = x >= 124 && x <= 132;
                let is_curb = x < 8 || x > 248;

                if is_curb {
                    img.put_pixel(x, y, Rgba([120, 120, 125, 255]));
                } else if is_center && is_dash {
                    // Weathered yellow road marking
                    img.put_pixel(x, y, Rgba([230, 195, 40, 255]));
                } else {
                    // Dark gritty asphalt with noise
                    let noise = ((x * 19 + y * 31) % 18) as u8;
                    let c = 38u8.saturating_add(noise);
                    img.put_pixel(x, y, Rgba([c, c, c + 2, 255]));
                }
            }
        }

        let _ = fs::create_dir_all(path.as_ref().parent().unwrap());
        let _ = img.save(path);
    }

    /// Generates realistic human Mafia skin & suit texture atlas (256x256)
    pub fn generate_mafia_suit_texture<P: AsRef<Path>>(path: P) {
        let width = 256;
        let height = 256;
        let mut img = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                if y < 64 {
                    // Face / Skin area (natural peach skin tones with subtle shading)
                    let shade = ((x + y) % 12) as u8;
                    img.put_pixel(x, y, Rgba([215 - shade, 165 - shade, 135 - shade, 255]));
                } else if y < 140 {
                    // Pinstripe charcoal black suit jacket
                    let is_pinstripe = x % 8 == 0;
                    if is_pinstripe {
                        img.put_pixel(x, y, Rgba([70, 70, 75, 255]));
                    } else {
                        img.put_pixel(x, y, Rgba([25, 25, 28, 255]));
                    }
                } else if y < 170 {
                    // Crimson red silk tie & white collar area
                    if x > 110 && x < 145 {
                        img.put_pixel(x, y, Rgba([185, 20, 25, 255]));
                    } else {
                        img.put_pixel(x, y, Rgba([225, 225, 230, 255]));
                    }
                } else {
                    // Charcoal trousers & shoes
                    let noise = ((x * 7 + y * 13) % 10) as u8;
                    img.put_pixel(x, y, Rgba([20 + noise, 20 + noise, 22 + noise, 255]));
                }
            }
        }

        let _ = fs::create_dir_all(path.as_ref().parent().unwrap());
        let _ = img.save(path);
    }

    /// Generates realistic NYC / Liberty Taxi vehicle livery with checkerboard doors (256x256)
    pub fn generate_taxi_texture<P: AsRef<Path>>(path: P) {
        let width = 256;
        let height = 256;
        let mut img = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                if y >= 115 && y <= 140 {
                    // Checkerboard taxi stripe
                    let check = ((x / 16) + (y / 12)) % 2 == 0;
                    if check {
                        img.put_pixel(x, y, Rgba([20, 20, 20, 255]));
                    } else {
                        img.put_pixel(x, y, Rgba([245, 245, 245, 255]));
                    }
                } else if y < 45 {
                    // Tinted car glass / windshield
                    img.put_pixel(x, y, Rgba([45, 60, 75, 230]));
                } else if y > 210 {
                    // Rubber tires & chrome hubcaps
                    let is_hub = (x % 64) > 20 && (x % 64) < 44;
                    if is_hub {
                        img.put_pixel(x, y, Rgba([190, 195, 200, 255]));
                    } else {
                        img.put_pixel(x, y, Rgba([25, 25, 25, 255]));
                    }
                } else {
                    // Glossy Taxi Yellow
                    let metallic = ((x + y) % 8) as u8;
                    img.put_pixel(x, y, Rgba([245 - metallic, 190 - metallic, 20, 255]));
                }
            }
        }

        let _ = fs::create_dir_all(path.as_ref().parent().unwrap());
        let _ = img.save(path);
    }
}
