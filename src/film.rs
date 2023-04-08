use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct FilmSample {
	pub num_of_samples: u32,
	pub accumulated_radiance: Vector3,
}

pub struct Film {
	width: u32,
	height: u32,
	pixels: Box<[FilmSample]>
}

pub fn to_srgb(linear_color: &Vector3) -> Vector3 {
	Vector3 { x: if linear_color.x <= 0.0031  { linear_color.x * 12.92 } else {1.055 * f32::powf(linear_color.x, 0.4166) - 0.055 }
		, y: if linear_color.y <= 0.0031  { linear_color.y * 12.92 } else {1.055 * f32::powf(linear_color.y, 0.4166) - 0.055 }
		, z: if linear_color.z <= 0.0031  { linear_color.z * 12.92 } else {1.055 * f32::powf(linear_color.z, 0.4166) - 0.055 }
	}
}

impl Film {
	pub fn new(width: u32, height: u32) -> Film {
		Film {
			width: width
			, height: height
			, pixels: vec![FilmSample { num_of_samples: 0, accumulated_radiance: Vector3 { x: 0.0, y: 0.0, z: 0.0 } }; (width * height) as usize].into_boxed_slice()
		}
	}

	#[inline(always)]
	fn index(&self, x: u32, y : u32) -> usize {
		debug_assert!(x < self.width && y < self.height);
		(x * self.height + y) as usize
	}

	#[inline(always)]
	pub fn add_sample(&mut self, x: u32, y: u32, radiance: &Vector3) {
		let pixel_index = self.index(x, y);
		self.pixels[pixel_index].accumulated_radiance += radiance;
		self.pixels[pixel_index].num_of_samples += 1;
	}

	pub fn save_image(&self, location: &'static str) {
		let mut image_buffer = image::ImageBuffer::new(self.width, self.height);

		for (x, y, image_pixel) in image_buffer.enumerate_pixels_mut() {
			let film_pixel = &self.pixels[self.index(x, self.height - y - 1)]; // the film is flipped in the camera so we need to revert it
			let linear_color = &film_pixel.accumulated_radiance / (film_pixel.num_of_samples as f32);
			let mut srgb = to_srgb(&linear_color);
			srgb.x = f32::clamp(srgb.x, 0.0, 1.0);
			srgb.y = f32::clamp(srgb.y, 0.0, 1.0);
			srgb.z = f32::clamp(srgb.z, 0.0, 1.0);
			*image_pixel = image::Rgb([(srgb.x * 255.0) as u8, (srgb.y * 255.0) as u8, (srgb.z * 255.0) as u8]);
		}

		image_buffer.save(location).unwrap();
	}
}

#[cfg(test)]
mod film_tests {
	use super::Film;
	use crate::vector::Vector3;
	use rand::Rng;

    #[test]
    fn main_test() {
		let mut film = Film::new(512, 512);
		let mut rgen = rand::thread_rng();

		for x in 0..512 {
			for y in 0..512 {
				for _ in 0..10 {
					let radiance = Vector3 { x: rgen.gen(), y: rgen.gen(), z: rgen.gen() };
					film.add_sample(x, y, &radiance);
				}
			}
		}

		film.save_image("film_tests.png");
	}
}
