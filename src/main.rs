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
    let back_wall = Plane::new(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 3.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        8.0,
        10.0,
    );

    let light_plane = Plane::new(
        Vector3 {
            x: 0.0,
            y: 4.95,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        1.3,
        1.3,
    );

    let ceiling = Plane::new(
        Vector3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        8.0,
        6.0,
    );

    let floor = Plane::new(
        Vector3 {
            x: 0.0,
            y: -5.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        8.0,
        6.0,
    );

    let right_wall = Plane::new(
        Vector3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        8.0,
        10.0,
    );

    let left_wall = Plane::new(
        Vector3 {
            x: -4.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        8.0,
        10.0,
    );

    let mirror_sphere = Sphere {
        position: Vector3 {
            x: -1.75,
            y: -2.5,
            z: 2.0,
        },
        radius: 1.35,
    };

    let glass_sphere = Sphere {
        position: Vector3 {
            x: 2.0,
            y: -2.25,
            z: 0.5,
        },
        radius: 1.5,
    };

    let light = EmissiveMaterial::new(
        &Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        27.777,
    );

    let white_diffuse = DiffuseMaterial {
        color: Vector3 {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
    };
    let green_diffuse = DiffuseMaterial {
        color: Vector3 {
            x: 0.1,
            y: 0.8,
            z: 0.1,
        },
    };
    let red_diffuse = DiffuseMaterial {
        color: Vector3 {
            x: 0.8,
            y: 0.1,
            z: 0.1,
        },
    };
    let mirror = ReflectiveMaterial {
        color: Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    };
    let glass = TransparentMaterial {
        color: Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        ior: 1.75,
    };

    let mut scene = Scene::new(Vector3 {
        x: 0.05,
        y: 0.05,
        z: 0.1,
    });

    scene.add_entity(Entity {
        material: &white_diffuse,
        shape: &back_wall,
    });
    scene.add_entity(Entity {
        material: &white_diffuse,
        shape: &floor,
    });
    scene.add_entity(Entity {
        material: &red_diffuse,
        shape: &left_wall,
    });
    scene.add_entity(Entity {
        material: &green_diffuse,
        shape: &right_wall,
    });
    scene.add_entity(Entity {
        material: &light,
        shape: &light_plane,
    });
    scene.add_entity(Entity {
        material: &white_diffuse,
        shape: &ceiling,
    });
    scene.add_entity(Entity {
        material: &mirror,
        shape: &mirror_sphere,
    });
    scene.add_entity(Entity {
        material: &glass,
        shape: &glass_sphere,
    });

    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;

    let camera = Camera::new(
        std::f32::consts::PI / 2.0,
        aspect_ratio,
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -7.0,
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

    let render_settings = RenderSettings {
        image_width: width,
        image_height: height,
        num_of_samples: 2048,
    };
    let film = renderer::render_scene(&scene, &camera, &render_settings);
    film.save_image("example.png");
}
