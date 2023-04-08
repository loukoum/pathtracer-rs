use crate::material::Material;
use crate::material::NoMaterial;
use crate::shape::Shape;
use crate::shape::ShapeIntersection;
use crate::vector::Vector3;
use crate::camera::Ray;

#[derive(Copy, Clone)]
pub struct Entity<'a> {
	pub material: &'a dyn Material,
	pub shape: &'a dyn Shape,
}

pub struct Scene<'a> {
	pub sky: Vector3,
	entities: Vec<Entity<'a>>,
}

pub struct EntityIntersection<'a> {
	pub shape_intersection: ShapeIntersection,
	pub material: &'a dyn Material,
}

impl <'a> EntityIntersection<'a> {
	pub fn new() -> EntityIntersection<'a> {
		EntityIntersection { shape_intersection: ShapeIntersection::new(), material: &NoMaterial }
	}
}

impl <'a> Scene<'a> {
	pub fn new(sky: Vector3) -> Scene<'a> {
		Scene { entities: Vec::new(), sky: sky }
	}

	pub fn add_entity(&mut self, entity: Entity<'a>) {
		self.entities.push(entity.clone());
	}

	pub fn trace(&self, ray: &Ray) -> EntityIntersection {
		let mut t = f32::MAX;
		let mut entity_intersection = EntityIntersection::new();

		for entity in self.entities.iter() {
			let intersection = 	entity.shape.intersect(ray);
			if intersection.t >= 0.0 && intersection.t < t {
				t = intersection.t;
				entity_intersection.shape_intersection = intersection;
				entity_intersection.material = entity.material;
			}
		}

		return entity_intersection;
	}
}
