use crate::Color;
use crate::Vector4D;
use crate::reflect;
use crate::Material;

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
                light: LightSource, point: Vector4D, eyev: Vector4D, normalv: Vector4D) -> Color {

    let effective_color = material.color * light.intensity;
    let lightv = (light.position - point).normalized();
    println!("lightv: {:?}", lightv);
    let ambient = effective_color * material.ambient;

    let mut light_dot_normal = lightv.dot(normalv);
    println!("light_dot_normal: {:?}", light_dot_normal);
    
    let mut diffuse : Color = Default::default(); 
    let mut specular : Color = Default::default(); 
    if light_dot_normal < 0.0 {
        diffuse = Color::BLACK();
        specular = Color::BLACK();
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        println!("diffuse: {:?}", diffuse);
        let reflectv = reflect(-lightv, normalv);
        println!("reflectv = {:?}", reflectv);
        let reflect_dot_eye = reflectv.dot(eyev);
        println!("reflect_dot_eye: {:?}", reflect_dot_eye);
        if reflect_dot_eye <= 0.0 {
            specular = Color::BLACK();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular

}
