use core::ops as ops;
use crate::tools as tools;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
	pub fn zero_vector() -> Vector3 {
		Vector3 { x: 0.0, y: 0.0, z: 0.0 }
	}

	#[inline(always)]
	pub fn dot(&self, other: &Vector3) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	#[inline(always)]
	pub fn length(&self) -> f32 {
		assert!(!self.is_zero());
		f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
	}

	#[inline(always)]
	pub fn cross(&self, other: &Vector3) -> Vector3 {
		Vector3 { x: self.y * other.z - self.z * other.y,
				  y: self.z * other.x - self.x * other.z,
				  z: self.x * other.y - self.y * other.x }
	}

	#[inline(always)]
	pub fn unit(&self) -> Vector3 {
		let inv_length = 1.0 / self.length();
		return self * inv_length;
	}

	#[inline(always)]
	pub fn normalize(&mut self) {
		let inv_lenth = 1.0 / self.length();
		self.x *= inv_lenth;
		self.y *= inv_lenth;
		self.z *= inv_lenth;
	}

	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		tools::equal_error(self.x, 0.0) && tools::equal_error(self.y, 0.0) && tools::equal_error(self.z, 0.0)
	}
}

impl PartialEq for Vector3 {
	#[inline(always)]
	fn eq(&self, other: &Vector3) -> bool {
		tools::equal_error(self.x, other.x)
			&& tools::equal_error(self.y, other.y)
			&& tools::equal_error(self.z, other.z)
	}
}

impl ops::Add<&Vector3> for &Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn add(self, rhs: &Vector3) -> Vector3 {
		Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}

impl ops::AddAssign<&Vector3> for Vector3 {
	#[inline(always)]
	fn add_assign(&mut self, rhs: &Vector3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl ops::Sub<&Vector3> for &Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn sub(self, rhs: &Vector3) -> Vector3 {
		Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}

impl ops::SubAssign<&Vector3> for Vector3 {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: &Vector3) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}

impl ops::Mul<f32> for &Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Vector3 {
		Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}

impl ops::MulAssign<f32> for Vector3 {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}

impl ops::Mul<&Vector3> for &Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, rhs: &Vector3) -> Vector3 {
		Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
	}
}

impl ops::MulAssign<&Vector3> for Vector3 {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: &Vector3) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}

impl ops::Div<f32> for &Vector3 {
	type Output =  Vector3;

	#[inline(always)]
	fn div(self, rhs: f32) -> Vector3 {
		Vector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
	}
}

impl ops::Neg for &Vector3{
	type Output = Vector3;

	fn neg(self) -> Vector3 {
		Vector3 { x: -self.x, y: -self.y, z: -self.z }
	}
}

#[cfg(test)]
mod vector_tests {
    use crate::tools as tools;
    use crate::vector::Vector3;

    #[test]
    fn main_test() {
        let mut v0 = Vector3 { x: 13.0, y: 21.5, z: -2.3 };
        let v1 = Vector3 { x: -1.0, y: -2.1, z: 1.1 };

		assert!(tools::equal_error(v0.dot(&v1), -60.68));

		let sum = &v1 + &v0;
        assert!(tools::equal_error(sum.x, 12.0));
        assert!(tools::equal_error(sum.y, 19.4));
        assert!(tools::equal_error(sum.z, -1.2));

		v0 += &v1;
        assert!(tools::equal_error(v0.x, 12.0));
        assert!(tools::equal_error(v0.y, 19.4));
        assert!(tools::equal_error(v0.z, -1.2));

		v0 = Vector3 {x: 0.0, y: 0.0, z: 0.0 };
		v0 = &v1 - &v0;
		assert!(&v0 == &v1);

		v0 = v1;
		v0 -= &v1;
		assert!(tools::equal_error(v0.x, 0.0));
		assert!(tools::equal_error(v0.y, 0.0));
		assert!(tools::equal_error(v0.z, 0.0));

		let a = Vector3 { x: 3.0, y: -3.0, z: 1.0 };
		let b = Vector3 { x: 4.0, y: 9.0, z: 2.0 };
		assert!(&a != &b);

		let a_x_b = a.cross(&b);
        assert!(tools::equal_error(a_x_b.x, -15.0));
        assert!(tools::equal_error(a_x_b.y, -2.0));
        assert!(tools::equal_error(a_x_b.z, 39.0));

		let z_axis = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
		let y_axis = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		let neg_x_axis = z_axis.cross(&y_axis);
		assert!(neg_x_axis == Vector3{ x: -1.0, y: 0.0, z: 0.0 });

		let a_mul_b = &a * &b;
        assert!(tools::equal_error(a_mul_b.x, 12.0));
        assert!(tools::equal_error(a_mul_b.y, -27.0));
        assert!(tools::equal_error(a_mul_b.z, 2.0));

		let c = b;
		assert!(&c == &b);

		let mut vec = Vector3 { x: 3.0, y: 1.0, z: 2.0 };
		let unit_vec = vec.unit();
		assert!(tools::equal_error(unit_vec.x, 0.8017));
		assert!(tools::equal_error(unit_vec.y, 0.2672));
		assert!(tools::equal_error(unit_vec.z, 0.5345));

		vec.normalize();
		assert!(vec == unit_vec);
    }
}
