use crate::ray_tracing::lighting::PointLight;
use crate::core::matrix::Matrix4x4;
use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::objects::sphere::Sphere;
use crate::ray_tracing::lighting::*;
use crate::core::comp::Comp;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use std::rc::Rc;

pub struct Scene {
    light_sources: Vec<PointLight>,
    objects: Vec<Rc<Sphere>>,
}

impl Scene {
    //Creates a new Scene
    pub fn new() -> Scene {
        Scene {
            light_sources: vec![],
            objects: vec![],
        }
    }

    //Pushes a light source to the Scene
    pub fn add_light(&mut self, light: PointLight) {
        self.light_sources.push(light); 
    }

    //Pushes a light source to the Scene
    pub fn clear_lights(&mut self) {
        self.light_sources.clear(); 
    }

    //Gets the Scene's light source
    pub fn get_light_sources(&self) -> &Vec<PointLight> {
       &self.light_sources
    }

    //Pushes an object to the scene
    pub fn add_object(&mut self, object: Rc<Sphere>) {
        self.objects.push(object);
    }

    //Gets the Scene's objects
    pub fn get_objects(&self) -> &Vec<Rc<Sphere>> {
        &self.objects
    }

    //Clears the objects in a scene
    pub fn clear_objects(&mut self) {
        self.objects.clear(); 
    }

    //Creates a default Scene
    pub fn default() -> Scene {
        let mut scene = Scene {
            light_sources: vec![PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0))],
            objects: vec![],
        };
        let mut sphere1_raw = Sphere::new_raw(); 
        &sphere1_raw.mut_material_ref().set_color(Color::new(0.8, 1.0, 0.6));
        &sphere1_raw.mut_material_ref().set_diffuse(0.7);
        &sphere1_raw.mut_material_ref().set_specular(0.2);
        let sphere1 = Rc::new(sphere1_raw);
        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        let sphere2 = Rc::new(sphere2_raw);
        &scene.add_object(sphere1); 
        &scene.add_object(sphere2); 
        scene
    }

    //Lights a pixel in the scene
    pub fn scene_lighting(scene: &Scene, comps: Comp) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        println!("point: {:?}, eye vector: {:?}, normal vector: {:?}", &comps.point, &comps.e_vec, &comps.n_vec);
        for light in scene.get_light_sources() {
            color = color + lighting(comps.object.get_material(), &light, &comps.point, &comps.e_vec, &comps.n_vec);
        }
        color
    }

    //Computes the color at a given point
    pub fn compute_color(ray: Ray, scene: &Scene) -> Option<Color> {
        let intersections = Ray::intersect_scene(scene, &ray);
        let hit = Intersection::hit(&intersections);
        if hit != None {
            let comps = Comp::compute_vars(hit.unwrap(), &ray);
            let color = Scene::scene_lighting(&scene, comps);
            Some(color)
        }
        else {
            None
        }
    }
}
