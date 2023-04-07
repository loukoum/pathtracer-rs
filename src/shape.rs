use crate::vector::Vector3;
use crate::camera::Ray;
use crate::tools as tools;

pub struct  ShapeIntersection {
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

pub struct Plane {
	pub position: Vector3,
	pub normal: Vector3,
	pub width: f32,
	pub height: f32,
}

impl ShapeIntersection {
	pub fn new() -> ShapeIntersection {
		ShapeIntersection { t: -1.0, surface_normal: Vector3 { x: 0.0, y: 0.0, z: 0.0 } }
	}
}

impl Shape for Sphere {
	fn intersect(&self, ray: &Ray) -> ShapeIntersection {
		let mut intersection = ShapeIntersection::new();
		let translated_origin = &ray.origin - &self.position; // Move sphere origin to 0, 0, 0 for easier calculations
		let a = 1.0; // a = (dx * dx + dy * dy + dz * dz) where d is ray.direction. ray.direction is normalized => a = 1.0 since dir.dot(dir) = len^2
		let b = 2.0 * translated_origin.dot(&ray.direction); // 2.0 * (dx * ox + dy * oy + dz * oz) where d is ray.direction and o is the translated_origin
		let c = translated_origin.dot(&translated_origin) - self.radius * self.radius; // (ox * ox + oy * oy + oz * oz) where o is the translated_origin
		let disc = b * b - 4.0 * a * c;
		if disc < 0.0 {
			return intersection;
		}

		let sqrt_disc = f32::sqrt(disc);
		let t0 = (-b + sqrt_disc) / (2.0 * a);
		let t1 = (-b - sqrt_disc) / (2.0 * a);

		if t1 >= 0.0 { // both t0 and t1 are greater than 0.0
			intersection.t = f32::min(t0, t1);
		}
		else if t0 >= 0.0 { // else check if t0 is greater than 0.0
			intersection.t = t0;
		}
		// both ts are negative -> ray in front of the sphere

		if intersection.t >= 0.0 {
			let intersection_point = &ray.origin + &(&ray.direction * intersection.t);
			intersection.surface_normal = &(&intersection_point - &self.position) / self.radius; // normalize
		}

		return intersection;
	}
}

impl Shape for Plane {
	fn intersect(&self, ray: &Ray) -> ShapeIntersection {
		let mut intersection = ShapeIntersection::new();

		let denom = self.normal.dot(&ray.direction);
		if tools::is_positive_error(f32::abs(denom)) {
			let pos_or = &self.position - &ray.origin;
			let t = self.normal.dot(&pos_or) / denom;
			if t >= 0.0 {
				let plane_x_dist = f32::abs(ray.origin.x + ray.direction.x * t - self.position.x);
				let plane_y_dist = f32::abs(ray.origin.y + ray.direction.y * t - self.position.y);
				if plane_x_dist <= self.width && plane_y_dist <= self.height {
					intersection.t = t;
					intersection.surface_normal = self.normal;
					debug_assert!(tools::equal_error(intersection.surface_normal.length(), 1.0)); // surface normal should be normalized
				}
			}
		}

		return intersection;
	}
}

#[cfg(test)]
mod shape_tests {
	use super::Shape;
	use super::Sphere;
	use super::Plane;
	use crate::vector::Vector3;
	use crate::camera::Ray;
	use crate::film::Film;
	use crate::camera::Camera;
	use crate::tools as tools;

	#[test]
	fn main_test() {
		let sphere = Sphere { position: Vector3 { x: 13.0, y: -69.0, z: 4.2 }, radius: 3.3 };
		let ray = Ray { origin: Vector3 { x: 13.0, y: -69.0, z: -1.0 }, direction: Vector3 { x: 0.0, y: 0.0, z: 1.0 }};
		let sphere_intersection = sphere.intersect(&ray);
		assert!(sphere_intersection.t > 0.0);

		let plane = Plane { position: Vector3 { x: 13.0, y: -69.0, z: 4.2 }
			, normal: Vector3 { x: 0.0, y: 0.0, z: 1.0 }
			, width: 5.0
			, height: 5.0
		};
		let plane_intersection = plane.intersect(&ray);
		assert!(plane_intersection.t > 0.0);
	}

	#[test]
	fn camera_ray_sphere_intersections_test() {
		let width = 800;
		let height = 600;
		let camera_position = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
		let camera_direction = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
		let camera_up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		let sphere_position = Vector3 { x: 0.0, y: 0.0, z: 10.0 };
		let camera = Camera::new(std::f32::consts::PI, (width as f32) / (height as f32), &camera_position, &camera_direction, &camera_up);
		let sphere = Sphere { position: sphere_position, radius: 5.0 };
		let mut film = Film::new(width, height);
		for x in 0..width {
			let film_x = (x as f32 + 0.5) / (width as f32);
			for y in 0..height {
				let film_y = (y as f32 + 0.5) / (height as f32);
				let ray = camera.generate_ray(film_x, film_y);
				let intersection = sphere.intersect(&ray);
				if intersection.t >= 0.0 {
					assert!(tools::equal_error(intersection.surface_normal.length(), 1.0));
					assert!(intersection.surface_normal.z <= 0.0);
					let sample_radiance = Vector3 { x: 1.0 / intersection.t + (x as f32 / 1000.0), y: 0.05, z: 1.0 / intersection.t };
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
		let camera_position = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
		let camera_direction = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
		let camera_up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		let plane_position = Vector3 { x: 0.0, y: 0.0, z: 10.0 };
		let plane_normal = (Vector3 { x: 0.0, y: 1.0, z: 1.0 }).unit();
		let camera = Camera::new(std::f32::consts::PI, (width as f32) / (height as f32), &camera_position, &camera_direction, &camera_up);
		let plane = Plane { position: plane_position
			, normal: plane_normal
			, width: 4.0
			, height: 3.0 };

		let mut film = Film::new(width, height);
		for x in 0..width {
			let film_x = (x as f32 + 0.5) / (width as f32);
			for y in 0..height {
				let film_y = (y as f32 + 0.5) / (height as f32);
				let ray = camera.generate_ray(film_x, film_y);
				let intersection = plane.intersect(&ray);
				if intersection.t >= 0.0 {
					let sample_radiance = Vector3 { x: 1.0 / intersection.t + (x as f32 / 1000.0), y: 0.05, z: 1.0 / intersection.t };
					film.add_sample(x, y, &sample_radiance);
				}
			}
		}
		
		film.save_image("camera_ray_plane_intersections_test.png");
	}
}
