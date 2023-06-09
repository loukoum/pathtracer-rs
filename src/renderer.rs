use crate::camera::Camera;
use crate::camera::Ray;
use crate::film::Film;
use crate::scene::Scene;
use crate::tools;
use crate::tools::Sampler;
use crate::vector::Vector3;

pub struct RenderSettings {
    pub image_width: u32,
    pub image_height: u32,
    pub num_of_samples: u32,
}

pub fn render_scene(scene: &Scene, camera: &Camera, render_settings: &RenderSettings) -> Film {
    println!(
        "Starting rendering image [{}, {}], with {} samples",
        render_settings.image_width, render_settings.image_height, render_settings.num_of_samples
    );
    let mut film = Film::new(render_settings.image_width, render_settings.image_height);
    let mut sampler = Sampler::default();

    let total_samples =
        render_settings.image_width * render_settings.image_height * render_settings.num_of_samples;
    for x in 0..render_settings.image_width {
        for y in 0..render_settings.image_height {
            for _ in 0..render_settings.num_of_samples {
                let sample = sampler.get_sample_2d();
                let film_x = (sample.s + x as f32) / render_settings.image_width as f32;
                let film_y = (sample.t + y as f32) / render_settings.image_height as f32;
                let ray = camera.generate_ray(film_x, film_y);
                let radiance = trace_ray(&ray, scene, &mut sampler);
                film.add_sample(x, y, &radiance);
            }
        }
        let samples_done = x * render_settings.image_height * render_settings.num_of_samples;
        print!("\rProgress: {} samples left.", total_samples - samples_done);
    }

    println!("\nDone!");
    film
}

fn trace_ray(camera_ray: &Ray, scene: &Scene, sampler: &mut Sampler) -> Vector3 {
    const MAX_DEPTH: u32 = 8;
    let mut ray = *camera_ray;
    let mut throughput = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    for _ in 0..MAX_DEPTH {
        let intersection = scene.trace(&ray);
        if intersection.shape_intersection.t < 0.0 {
            return &throughput * &scene.sky;
        }

        let emission = intersection.material.get_emission();
        if !emission.is_zero() {
            return &throughput * &emission;
        }

        let material_sample = intersection.material.sample_material(
            &(-&ray.direction),
            &intersection.shape_intersection,
            sampler,
        );
        let wi_dot_n = f32::abs(
            material_sample
                .sample_direction
                .dot(&intersection.shape_intersection.surface_normal),
        );

        if tools::equal_error(material_sample.pdf, 0.0)
            || material_sample.sample_direction.is_zero()
        {
            break;
        }

        let new_throughput = &material_sample.brdf * (wi_dot_n / material_sample.pdf);
        throughput *= &new_throughput;

        let intersection_point =
            &ray.origin + &(&ray.direction * intersection.shape_intersection.t);

        ray.origin = &intersection_point + &(&material_sample.sample_direction * 0.001);
        ray.direction = material_sample.sample_direction;
    }

    Vector3::zero_vector()
}
