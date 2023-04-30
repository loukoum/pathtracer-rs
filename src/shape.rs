use crate::camera::Ray;
use crate::tools;
use crate::vector::Vector3;

pub struct ShapeIntersection {
    pub t: f32, // negative t means no intersection
    pub surface_normal: Vector3,
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> ShapeIntersection;
}

pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
}

#[derive(Debug)]
pub struct Plane {
    pub position: Vector3,
    pub normal: Vector3,
    pub up: Vector3,
    pub half_width: f32,
    pub half_height: f32,
    right: Vector3,
}

impl Default for ShapeIntersection {
    fn default() -> ShapeIntersection {
        ShapeIntersection {
            t: -1.0,
            surface_normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> ShapeIntersection {
        let mut intersection = ShapeIntersection::default();
        let translated_origin = &ray.origin - &self.position; // Move sphere origin to 0, 0, 0 for easier calculations
        let a = 1.0; // a = (dx * dx + dy * dy + dz * dz) where d is ray.direction. ray.direction is normalized => a = 1.0 since dir.dot(dir) = len^2
        let b = 2.0 * translated_origin.dot(&ray.direction); // 2.0 * (dx * ox + dy * oy + dz * oz) where d is ray.direction and o is the translated_origin
        let c = translated_origin.dot(&translated_origin) - self.radius * self.radius; // (ox * ox + oy * oy + oz * oz) where o is the translated_origin
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return intersection;
        }

        let sqrt_disc = f32::sqrt(disc);
        let t0 = (-b + sqrt_disc) * 0.5; // should be div(/) 2 * a but a = 1
        let t1 = (-b - sqrt_disc) * 0.5;

        if t1 >= 0.0 {
            // both t0 and t1 are greater than 0.0
            intersection.t = f32::min(t0, t1);
        } else if t0 >= 0.0 {
            // else check if t0 is greater than 0.0
            intersection.t = t0;
        }
        // both ts are negative -> ray in front of the sphere

        if intersection.t >= 0.0 {
            let intersection_point = &ray.origin + &(&ray.direction * intersection.t);
            intersection.surface_normal = (&intersection_point - &self.position).unit();
        }

        intersection
    }
}

impl Plane {
    pub fn new(position: Vector3, normal: Vector3, up: Vector3, width: f32, height: f32) -> Plane {
        debug_assert!(tools::equal_error(normal.length(), 1.0));
        debug_assert!(tools::equal_error(up.length(), 1.0));
        debug_assert!(tools::equal_error(up.cross(&normal).length(), 1.0));
        Plane {
            position,
            normal,
            up,
            right: up.cross(&normal),
            half_width: width * 0.5,
            half_height: height * 0.5,
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> ShapeIntersection {
        let mut intersection = ShapeIntersection::default();

        let denom = self.normal.dot(&ray.direction);
        if f32::abs(denom) > 0.0 {
            let pos_origin = &self.position - &ray.origin;
            let t = self.normal.dot(&pos_origin) / denom;
            if t >= 0.0 {
                let intersection_point_origin =
                    &(&ray.origin + &(&ray.direction * t)) - &self.position;

                let plane_basis_ip = &Vector3::to_basis(
                    &intersection_point_origin,
                    &self.normal,
                    &self.right,
                    &self.up,
                );

                if plane_basis_ip.x.abs() <= self.half_width
                    && plane_basis_ip.y.abs() <= self.half_height
                {
                    intersection.t = t;
                    intersection.surface_normal = self.normal;
                    debug_assert!(tools::equal_error(
                        intersection.surface_normal.length(),
                        1.0
                    )); // surface normal should be normalized
                }
            }
        }

        intersection
    }
}

#[cfg(test)]
mod shape_tests {
    use super::Plane;
    use super::Shape;
    use super::Sphere;
    use crate::camera::Camera;
    use crate::camera::Ray;
    use crate::film::Film;
    use crate::tools;
    use crate::vector::Vector3;

    #[test]
    fn main_test() {
        let sphere = Sphere {
            position: Vector3 {
                x: 13.0,
                y: -69.0,
                z: 4.2,
            },
            radius: 3.3,
        };
        let ray = Ray {
            origin: Vector3 {
                x: 13.0,
                y: -69.0,
                z: -1.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere_intersection = sphere.intersect(&ray);
        assert!(sphere_intersection.t > 0.0);

        let plane = Plane::new(
            Vector3 {
                x: 13.0,
                y: -69.0,
                z: 4.2,
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
            1.0,
            1.0,
        );
        let plane_intersection = plane.intersect(&ray);
        assert!(plane_intersection.t > 0.0);
    }

    #[test]
    fn camera_ray_sphere_intersections_test() {
        let width = 800;
        let height = 600;
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
        let sphere_position = Vector3 {
            x: 5.0,
            y: 0.0,
            z: 10.0,
        };
        let camera = Camera::new(
            std::f32::consts::PI / 2.0,
            (width as f32) / (height as f32),
            camera_position,
            camera_direction,
            camera_up,
        );
        let sphere = Sphere {
            position: sphere_position,
            radius: 3.0,
        };
        let mut film = Film::new(width, height);
        for x in 0..width {
            let film_x = (x as f32 + 0.5) / (width as f32);
            for y in 0..height {
                let film_y = (y as f32 + 0.5) / (height as f32);
                let ray = camera.generate_ray(film_x, film_y);
                let intersection = sphere.intersect(&ray);
                if intersection.t >= 0.0 {
                    assert!(tools::equal_error(
                        intersection.surface_normal.length(),
                        1.0
                    ));
                    let sample_radiance = Vector3 {
                        x: 1.0 / intersection.t + (x as f32 / 1000.0),
                        y: 0.05,
                        z: 1.0 / intersection.t,
                    };
                    film.add_sample(x, y, &sample_radiance);
                }
            }
        }

        film.save_image("camera_ray_sphere_intersections_test.png");
    }

    #[test]
    fn camera_ray_plane_intersections_test() {
        let width = 800;
        let height = 600;
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
        let plane_position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        };
        let plane_normal = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let plane_up = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
        .unit();

        let camera = Camera::new(
            std::f32::consts::PI / 2.0,
            (width as f32) / (height as f32),
            camera_position,
            camera_direction,
            camera_up,
        );

        let plane = Plane::new(plane_position, plane_normal, plane_up, 4.0, 4.0);

        let mut film = Film::new(width, height);
        for x in 0..width {
            let film_x = (x as f32 + 0.5) / (width as f32);
            for y in 0..height {
                let film_y = (y as f32 + 0.5) / (height as f32);
                let ray = camera.generate_ray(film_x, film_y);
                let intersection = plane.intersect(&ray);
                if intersection.t > 0.0 {
                    let sample_radiance = Vector3 {
                        x: 1.0 / intersection.t + (x as f32 / 1000.0),
                        y: 0.1,
                        z: 1.0 / intersection.t,
                    };
                    film.add_sample(x, y, &sample_radiance);
                }
            }
        }

        film.save_image("camera_ray_plane_intersections_test.png");
    }
}
