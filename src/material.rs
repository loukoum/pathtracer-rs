use crate::vector::Vector3;
use crate::shape::ShapeIntersection;
use crate::tools as tools;
use crate::tools::Sampler;

#[derive(Debug)]
pub struct MaterialSample {
	pub brdf: Vector3,
	pub sample_direction: Vector3,
	pub pdf: f32,
}

pub trait Material {
	fn sample_material(&self, wo: &Vector3, intersection: &ShapeIntersection, sampler: &mut Sampler) -> MaterialSample;
	fn get_emission(&self) -> Vector3 {
		Vector3::zero_vector()
	}
}

pub struct NoMaterial;
impl Material for NoMaterial {
	fn sample_material(&self, _wo: &Vector3, _intersection: &ShapeIntersection, _sampler: &mut Sampler) -> MaterialSample {
		MaterialSample::invalid_sample()
	}
}

pub struct EmissiveMaterial {
	pub emission: Vector3
}

pub struct DiffuseMaterial {
	pub color: Vector3,
}

pub struct ReflectiveMaterial {
	pub color: Vector3,
}

pub struct TransparentMaterial {
	pub color: Vector3,
	pub ior: f32, // index of refraction
}

impl MaterialSample {
	pub fn invalid_sample() -> MaterialSample {
		MaterialSample { brdf: Vector3::zero_vector(), sample_direction: Vector3::zero_vector(), pdf: 0.0 }
	}
}

impl EmissiveMaterial {
	pub fn new(color: &Vector3, intensity: f32) -> EmissiveMaterial {
		EmissiveMaterial { emission: color * intensity }
	}
}

impl Material for EmissiveMaterial {
	fn sample_material(&self, _wo: &Vector3, _intersection: &ShapeIntersection, _sampler: &mut Sampler) -> MaterialSample {
		MaterialSample::invalid_sample()
	}

	fn get_emission(&self) -> Vector3 {
		self.emission
	}
}

impl Material for DiffuseMaterial {
	// sample hemisphere uniformly
	fn sample_material(&self, wo: &Vector3, intersection: &ShapeIntersection, sampler: &mut Sampler) -> MaterialSample {
		if tools::is_negative_error(wo.dot(&intersection.surface_normal)) {
			return MaterialSample::invalid_sample();
		}

		const ONE_OVER_PI: f32 = 1.0 / std::f32::consts::PI;
		let sample_2d = sampler.get_sample_2d();
		let cos_theta = 1.0 - 2.0 * sample_2d.t;
		let phi = sample_2d.s * (2.0 * std::f32::consts::PI);
		let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
		let sample_dir = Vector3 { x: sin_theta * f32::cos(phi)
			, y: sin_theta * f32::sin(phi)
			, z: f32::abs(cos_theta)
		};

		MaterialSample { brdf: &self.color * ONE_OVER_PI, sample_direction: sample_dir, pdf: 0.5 * ONE_OVER_PI }
	}
}

#[inline(always)]
fn reflect(wo: &Vector3, n: &Vector3, wo_dot_n: f32) -> Vector3 {
	&(n * (2.0 * wo_dot_n)) - wo
}

#[inline(always)]
fn refract(wo: &Vector3, n: &Vector3, one_over_eta: f32, wo_dot_n: f32) -> Vector3 {
	let sin2_theta_eta2 = one_over_eta * one_over_eta * (1.0 - wo_dot_n * wo_dot_n);
	if !tools::less_error(sin2_theta_eta2, 1.0) {
		return Vector3::zero_vector();
	}

	let cos_theta_t = f32::sqrt(1.0 - sin2_theta_eta2);
	return &(&(-wo) * one_over_eta) + &(n * (one_over_eta * wo_dot_n - cos_theta_t));
}

#[inline(always)]
fn fresnel_schlik(r0: &Vector3, cos_theta: f32) -> Vector3 {
	const ONES_VEC: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
	let oct = 1.0 - cos_theta;
	return r0 + &(&(&ONES_VEC - r0) * (oct * oct * oct * oct * oct));
}

#[inline(always)]
fn calculate_fresnel(eta: f32, cos_theta: f32) -> f32 {
	let g_sqrt = eta * eta + cos_theta * cos_theta - 1.0;
	if !tools::is_positive_error(g_sqrt) {
		return 1.0;
	}

	let g = f32::sqrt(g_sqrt);
	let mut first = (g - cos_theta) / (g + cos_theta);
	first *= 0.5 * first;

	let denom = (g - cos_theta) * cos_theta + 1.0;
	if tools::equal_error(denom, 0.0) {
		return 1.0;
	}

	let mut sec = ((g + cos_theta) * cos_theta - 1.0) / denom;
	sec *= sec;
	return first * (1.0 + sec);
}

impl Material for ReflectiveMaterial {
	fn sample_material(&self, wo: &Vector3, intersection: &ShapeIntersection, _sampler: &mut Sampler) -> MaterialSample {
		let wo_dot_n = wo.dot(&intersection.surface_normal);
		if tools::is_negative_error(wo_dot_n) {
			return MaterialSample::invalid_sample();
		}

		MaterialSample { brdf: fresnel_schlik(&self.color, wo_dot_n)
			, sample_direction: reflect(wo, &intersection.surface_normal, wo_dot_n)
			, pdf: 1.0
		}
	}
}

impl Material for TransparentMaterial {
	fn sample_material(&self, wo: &Vector3, intersection: &ShapeIntersection, sampler: &mut Sampler) -> MaterialSample {
		let mut wo_dot_n = wo.dot(&intersection.surface_normal);
		if tools::equal_error(wo_dot_n, 0.0) {
			return MaterialSample::invalid_sample();
		}

		let eta = if wo_dot_n < 0.0 { self.ior } else { 1.0 / self.ior };
		let mut n = intersection.surface_normal;
		if wo_dot_n < 0.0 {
			wo_dot_n = -wo_dot_n;
			n = -&n;
		}

		let fresnel = calculate_fresnel(eta, wo_dot_n);
		if sampler.get_sample() < fresnel {
			return MaterialSample { brdf: &self.color * (fresnel / wo_dot_n)
				, sample_direction: reflect(wo, &n, wo_dot_n)
				, pdf: fresnel
			}
		}

		let wi = refract(wo, &n, 1.0 / eta, wo_dot_n);
		let wi_dot_n = f32::abs(wi.dot(&intersection.surface_normal));
		if tools::equal_error(wi_dot_n, 0.0) {
			return MaterialSample::invalid_sample();
		}

		MaterialSample { brdf: &self.color * (fresnel * eta * eta / wi_dot_n)
			, sample_direction: wi
			, pdf: 1.0 - fresnel
		}
	}
}

#[cfg(test)]
mod material_tests {
	use super::EmissiveMaterial;
	use crate::material::Material;
	use crate::vector::Vector3;
	use crate::camera::Camera;
	use crate::shape::{Shape, Sphere};
	use crate::film::Film;
	use crate::tools as tools;
	use crate::tools::Sampler;

	#[test]
	fn emission_material_test() {
		let width = 800;
		let height = 600;
		let camera_position = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
		let camera_direction = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
		let camera_up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		let sphere_position = Vector3 { x: 0.0, y: 0.0, z: 10.0 };
		let camera = Camera::new(std::f32::consts::PI, (width as f32) / (height as f32), &camera_position, &camera_direction, &camera_up);
		let sphere = Sphere { position: sphere_position, radius: 5.0 };
		let mut film = Film::new(width, height);
		let material_color = Vector3 { x: 0.8, y: 0.1, z: 0.8 };
		let material = EmissiveMaterial::new(&material_color, 1.0);
		let mut sampler = Sampler::new();
		for x in 0..width {
			let film_x = (x as f32 + 0.5) / (width as f32);
			for y in 0..height {
				let film_y = (y as f32 + 0.5) / (height as f32);
				let ray = camera.generate_ray(film_x, film_y);
				let intersection = sphere.intersect(&ray);
				if intersection.t >= 0.0 {
					assert!(tools::equal_error(intersection.surface_normal.length(), 1.0));
					assert!(intersection.surface_normal.z <= 0.0);
					let sample_radiance = material.get_emission();
					film.add_sample(x, y, &sample_radiance);
				}
			}
		}
		
		film.save_image("emission_material_test.png");
	}
}
