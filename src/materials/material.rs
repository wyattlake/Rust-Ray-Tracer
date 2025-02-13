use crate::core::color::Color;
use crate::misc::utils::clamp_float;
use crate::materials::patterns::*;

//A Material holds a bunch of properties for an object
//Lighting properties are based on the Phong Reflection Model
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub environment_lighting: f32,
    pub casts_shadows: bool,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Material {
    //Creates a new Material and clamps all the values
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        reflectivity: f32,
        transparency: f32,
        refractive_index: f32,
        environment_lighting: f32,
        casts_shadows: bool,
        pattern: Option<Box<dyn Pattern>>,
    ) -> Material {
        Material {
            color,
            ambient: clamp_float(ambient, 0.0, 1.0),
            diffuse: clamp_float(diffuse, 0.0, 1.0),
            specular: clamp_float(specular, 0.0, 1.0),
            shininess: clamp_float(shininess, 1.0, 200.0),
            reflectivity,
            transparency,
            refractive_index,
            environment_lighting,
            casts_shadows,
            pattern,
        }
    }

    //Sets a material
    pub fn set(
        &mut self,
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        reflectivity: f32,
        pattern: Option<Box<dyn Pattern>>,
    ) {
        self.color = color;
        self.ambient = clamp_float(ambient, 0.0, 1.0);
        self.diffuse = clamp_float(diffuse, 0.0, 1.0);
        self.specular = clamp_float(specular, 0.0, 1.0);
        self.shininess = clamp_float(shininess, 1.0, 200.0);
        self.reflectivity = reflectivity;
        self.pattern = pattern;
    }

    //Creates a material with default values
    pub fn default() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            environment_lighting: 0.0,
            casts_shadows: true,
            pattern: None,
        }
    }
}


