use crate::tools;
use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    position: Vector3,
    up: Vector3,
    look: Vector3,
    right: Vector3,
    width: f32,
    aspect_ratio: f32,
}

pub struct OrthographicCamera {
    corner: Vector3,
    look_dir: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(
        fov: f32,
        aspect_ratio: f32,
        position: Vector3,
        look_dir: Vector3,
        up_dir: Vector3,
    ) -> Camera {
        let width = f32::tan(fov * 0.5) * 2.0;
        Camera {
            position,
            up: up_dir,
            look: look_dir,
            right: up_dir.cross(&look_dir),
            width,
            aspect_ratio,
        }
    }

    #[inline(always)]
    pub fn generate_ray(&self, film_x: f32, film_y: f32) -> Ray {
        debug_assert!((0.0..=1.0).contains(&film_x) && (0.0..=1.0).contains(&film_y));
        let film_position = Vector3 {
            x: self.width * film_x - 1.0,
            y: (self.width * film_y - 1.0) / self.aspect_ratio,
            z: 1.0,
        };
        let film_position_camera =
            Vector3::to_basis(&film_position, &self.look, &self.right, &self.up);
        Ray {
            origin: self.position,
            direction: film_position_camera.unit(),
        }
    }
}

impl OrthographicCamera {
    pub fn new(
        scale: f32,
        aspect_ratio: f32,
        position: Vector3,
        look_dir: Vector3,
        up: Vector3,
    ) -> OrthographicCamera {
        debug_assert!(tools::equal_error(look_dir.length(), 1.0));
        let horizontal = &(up.cross(&look_dir)) * scale;
        let vertical = &(up.unit()) * (scale / aspect_ratio);
        OrthographicCamera {
            corner: &position - &(&(&horizontal * 0.5) + &(&vertical * 0.5)),
            look_dir,
            horizontal,
            vertical,
        }
    }

    #[inline(always)]
    pub fn generate_ray(&self, film_x: f32, film_y: f32) -> Ray {
        Ray {
            origin: &self.corner + &(&(&self.horizontal * film_x) + &(&self.vertical * film_y)),
            direction: self.look_dir,
        }
    }
}

#[cfg(test)]
mod camera_tests {
    use super::Camera;
    use crate::vector::Vector3;

    #[test]
    fn main_test() {
        let cam = Camera::new(
            std::f32::consts::PI / 2.0,
            1.0,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        println!("Camera = {:?}", cam);

        let center_ray = cam.generate_ray(0.5, 0.5);
        println!("Ray = {:?}", center_ray);
        assert!(
            center_ray.direction
                == Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0
                }
        );
    }

    #[test]
    fn direction_test() {
        let width = 800;
        let height = 800;
        let camera_position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let camera_direction = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let camera_up = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let camera = Camera::new(
            std::f32::consts::PI / 2.0,
            (width as f32) / (height as f32),
            camera_position,
            camera_direction,
            camera_up,
        );

        let mut image_buffer = image::ImageBuffer::new(width, height);
        for (x, y, image_pixel) in image_buffer.enumerate_pixels_mut() {
            let film_x = (x as f32 + 0.5) / (width as f32);
            let film_y = (y as f32 + 0.5) / (height as f32);
            let ray = camera.generate_ray(film_x, film_y);
            let x = (ray.direction.x + 1.0) * 0.5;
            let y = (ray.direction.y + 1.0) * 0.5;
            let z = (ray.direction.z + 1.0) * 0.5;
            *image_pixel = image::Rgb([(x * 255.0) as u8, (y * 255.0) as u8, (z * 255.0) as u8]);
        }

        image_buffer.save("camera_direction.ppm").unwrap();
    }
}
