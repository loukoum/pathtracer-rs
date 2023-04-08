use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    position: Vector3,
    left_edge: Vector3,
    horizontal_parallel: Vector3,
    vertical_parallel: Vector3,
}

impl Camera {
    pub fn new(
        fov: f32,
        aspect_ratio: f32,
        position: &Vector3,
        look_dir: &Vector3,
        up_dir: &Vector3,
    ) -> Camera {
        let left = look_dir.cross(up_dir).unit();
        let half_width = f32::tanh(fov / 2.0);

        Camera {
            position: *position,
            left_edge: look_dir + &(&left * half_width),
            horizontal_parallel: &left * (half_width * -2.0),
            vertical_parallel: up_dir * ((2.0 * half_width) / aspect_ratio),
        }
    }

    #[inline(always)]
    pub fn generate_ray(&self, film_x: f32, film_y: f32) -> Ray {
        debug_assert!(film_x >= 0.0 && film_x <= 1.0 && film_y >= 0.0 && film_y <= 1.0);
        Ray {
            origin: self.position,
            direction: (&(&self.left_edge + &(&self.horizontal_parallel * film_x))
                + &(&self.vertical_parallel * (film_y - 0.5)))
                .unit(),
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
            &Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            &Vector3 {
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
}
