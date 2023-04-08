use camera::Camera;
use material::{DiffuseMaterial, EmissiveMaterial, ReflectiveMaterial, TransparentMaterial};
use renderer::RenderSettings;
use scene::{Entity, Scene};
use shape::{Plane, Sphere};
use vector::Vector3;

pub mod camera;
pub mod film;
pub mod material;
pub mod renderer;
pub mod scene;
pub mod shape;
pub mod tools;
pub mod vector;

fn main() {
    let floor_normal = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }
    .unit();
    let floor = Plane {
        position: Vector3 {
            x: 0.0,
            y: -2.0,
            z: 0.0,
        },
        normal: floor_normal,
        width: 10.0,
        height: 10.0,
    };

    let light_sphere = Sphere {
        position: Vector3 {
            x: 0.5,
            y: 0.0,
            z: 3.0,
        },
        radius: 0.1,
    };
    let mirror_sphere = Sphere {
        position: Vector3 {
            x: 0.0,
            y: 0.4,
            z: 5.0,
        },
        radius: 0.3,
    };

    let light = EmissiveMaterial::new(
        &Vector3 {
            x: 0.5,
            y: 0.1,
            z: 0.1,
        },
        7.0,
    );
    let white_diffuse = DiffuseMaterial {
        color: Vector3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    };
    let mirror = ReflectiveMaterial {
        color: Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    };
    let _glass = TransparentMaterial {
        color: Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        ior: 1.5,
    };

    let mut scene = Scene::new(Vector3 {
        x: 0.1,
        y: 0.1,
        z: 0.4,
    });
    scene.add_entity(Entity {
        material: &light,
        shape: &light_sphere,
    });
    scene.add_entity(Entity {
        material: &mirror,
        shape: &mirror_sphere,
    });
    scene.add_entity(Entity {
        material: &white_diffuse,
        shape: &floor,
    });

    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;

    let camera = Camera::new(
        std::f32::consts::PI / 2.0,
        aspect_ratio,
        &Vector3 {
            x: 0.0,
            y: 0.0,
            z: 2.0,
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

    let render_settings = RenderSettings {
        image_width: width,
        image_height: height,
        num_of_samples: 64,
    };
    let film = renderer::render_scene(&scene, &camera, &render_settings);
    film.save_image("example.png");
}
