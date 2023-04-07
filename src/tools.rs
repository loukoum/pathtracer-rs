use rand::{rngs::ThreadRng, distributions::uniform::{UniformFloat, UniformSampler}};

const ERROR: f32 = 0.0001;

pub fn equal_error(f0: f32, f1: f32) -> bool {
    f32::abs(f0 - f1) < ERROR
}

pub fn greater_error(f0: f32, f1: f32) -> bool {
	f0 > f1 + ERROR
}

pub fn less_error(f0: f32, f1: f32) -> bool {
	f0 < f1 - ERROR
}

pub fn is_positive_error(f0: f32) -> bool {
	f0 > ERROR
}

pub fn is_negative_error(f0: f32) -> bool {
	f0 < -ERROR
}

pub struct Sampler {
	rgen: ThreadRng,
	distribution: UniformFloat<f32>,
}

pub struct Sample2D {
	pub s: f32,
	pub t: f32,
}

impl Sampler {
	pub fn new() -> Sampler {
		Sampler { rgen: rand::thread_rng(), distribution: UniformFloat::new(0.0, 1.0) }
	}

	#[inline(always)]
	pub fn get_sample(&mut self) -> f32 {
		self.distribution.sample(&mut self.rgen)
	}

	#[inline(always)]
	pub fn get_sample_2d(&mut self) -> Sample2D {
		Sample2D { s: self.get_sample(), t: self.get_sample() }
	}
}
