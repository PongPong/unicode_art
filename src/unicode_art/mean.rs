use image::{DynamicImage, GenericImageView};

pub trait Mean {
    fn mean(&self, sx: u32, ex: u32, sy: u32, ey: u32) -> u8;
}

impl Mean for DynamicImage {
    fn mean(&self, sx: u32, ex: u32, sy: u32, ey: u32) -> u8 {
        let sub_image = self.view(sx, sy, ex - sx, ey - sy);
        let sub_image = sub_image.to_image();

        let len = sub_image.pixels().len();
        debug_assert_ne!(len, 0);
        let sum = sub_image.pixels().fold(0u32, |mut sum, &pixel| {
            let image::Rgba(data): image::Rgba<u8> = pixel;
            sum += data[0] as u32 + data[1] as u32 + data[2] as u32;
            sum
        });
        (sum / 3 / len as u32) as u8
    }
}
