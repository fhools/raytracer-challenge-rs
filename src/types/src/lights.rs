use crate::Color;
use crate::Vector4D;
use crate::reflect;
use crate::Material;
use crate::Intersectable;
use crate::Pattern;
use crate::Patternable;
#[derive(Copy,Clone, Debug)]
pub struct LightSource {
    pub intensity: Color,
    pub position: Vector4D
}


impl LightSource {
    pub fn new(intensity: Color, position: Vector4D) -> LightSource {
        LightSource {
            intensity,
            position
        }
    }
}

pub fn lighting(material: Material, 
                object: &dyn Intersectable,
                light: LightSource, 
                point: Vector4D, eyev: Vector4D, normalv: Vector4D, shadow: bool) -> Color {

    
    let effective_color = match material.pattern {
        None => { material.color },
        Some(pattern) =>  {  match *pattern {
            Pattern::StripePattern(pattern) => {
                pattern.pattern_at_object(object, point) 
            },
            Pattern::RingPattern(pattern) => {
                pattern.pattern_at_object(object, point)
            },
            Pattern::GradientPattern(pattern) => {
                pattern.pattern_at_object(object, point)
            },
            Pattern::CheckeredPattern(pattern) => {
                pattern.pattern_at_object(object, point)
            }
        }
        }
    } * light.intensity;
    let lightv = (light.position - point).normalized();
    let ambient = effective_color * material.ambient;

    let light_dot_normal = lightv.dot(normalv);
    
    let diffuse; 
    let specular;
    if light_dot_normal < 0.0 || shadow {
        diffuse = Color::BLACK;
        specular = Color::BLACK;
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            specular = Color::BLACK;
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular

}
