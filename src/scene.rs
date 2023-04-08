use crate::camera::Ray;
use crate::material::Material;
use crate::material::NoMaterial;
use crate::shape::Shape;
use crate::shape::ShapeIntersection;
use crate::vector::Vector3;

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

impl<'a> Default for EntityIntersection<'a> {
    fn default() -> EntityIntersection<'a> {
        EntityIntersection {
            shape_intersection: ShapeIntersection::default(),
            material: &NoMaterial,
        }
    }
}

impl<'a> Scene<'a> {
    pub fn new(sky: Vector3) -> Scene<'a> {
        Scene {
            entities: Vec::new(),
            sky,
        }
    }

    pub fn add_entity(&mut self, entity: Entity<'a>) {
        self.entities.push(entity);
    }

    pub fn trace(&self, ray: &Ray) -> EntityIntersection {
        let mut t = f32::MAX;
        let mut entity_intersection = EntityIntersection::default();

        for entity in self.entities.iter() {
            let intersection = entity.shape.intersect(ray);
            if intersection.t >= 0.0 && intersection.t < t {
                t = intersection.t;
                entity_intersection.shape_intersection = intersection;
                entity_intersection.material = entity.material;
            }
        }

        entity_intersection
    }
}
