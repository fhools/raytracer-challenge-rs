use crate::LightSource;
use crate::Sphere;
use crate::Matrix4x4;
use crate::Shape;
use crate::Intersections;
use crate::Intersection;
use crate::Intersectable;
use crate::Material;
use crate::Color;
use crate::Ray;
use crate::Vector4D;
use crate::hit;
use crate::positive_hits;
use crate::color_at;
use crate::ShadeComputation;
use utils::*;
use crate::global_do_debug;

pub struct World {
    pub light_source: LightSource,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new() -> World {
        World {
            light_source: LightSource::new(Color::new(1.0, 1.0, 1.0), Vector4D::new_point(-10.0, 10.0, -10.0)),
            objects: vec![]
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut vs : Intersections = vec![];
        for o in self.objects.iter() {
            match o {
                Shape::Sphere(s) => {
                    vs.extend(ray.intersect(s));
                },
                Shape::TestShape(t) => {
                    vs.extend(ray.intersect(t));
                },
                Shape::Plane(p) => {
                    vs.extend(ray.intersect(p));
                },
                Shape::Cube(c) => {
                    vs.extend(ray.intersect(c));
                },
                Shape::Cylinder(c) => {
                    vs.extend(ray.intersect(c));
                },
                Shape::Cone(c) => {
                    vs.extend(ray.intersect(c));
                },
            }

        }
        vs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        vs.dedup_by(|a, b| f64_eq(a.t, b.t));
        // NOTE: Lets not filter here, we can use the hit() function to locate the first non
        // negative hit.  
         //vs = vs.iter().filter(|a| a.t >= 0.0).cloned().collect::<Vec<Intersection>>();
        vs
    }

    pub fn is_shadowed_new(&self, point: Vector4D) -> bool {
        let mut to_light_vec = self.light_source.position - point;
        let distance_to_light = to_light_vec.norm();
        to_light_vec.normalize(); 
        let ray_to_light = Ray::new(point, to_light_vec);
        let xs = ray_to_light.intersect_world(self);
        let xs = positive_hits(&xs);
        for a_xs in xs.iter() {
            if !a_xs.obj.get_material().no_cast_shadow {
               if a_xs.t < distance_to_light {
                   return true;
               }
            }
        }
        return false
    }

    pub fn is_shadowed(&self, point: Vector4D) -> bool {
        let mut to_light_vec = self.light_source.position - point;
        let distance_to_light = to_light_vec.norm();
        to_light_vec.normalize(); 
        let ray_to_light = Ray::new(point, to_light_vec);
        let xs = ray_to_light.intersect_world(self);
        if let Some(a_xs) = hit(&xs) {
            unsafe {
            if let Some(true) = global_do_debug {
                println!("bad shadow: xs:{:?}, ray_to_light: {:?}", xs, ray_to_light);
            }
            }
            a_xs.t < distance_to_light 
        } else  {
            false
        }
    }

    pub fn is_shadowed_disabled(&self, point: Vector4D) -> bool {
        false
    }

    pub fn reflected_color(&self, shade_computation: &ShadeComputation, reflect_rays_remaining: usize) -> Color {
        if reflect_rays_remaining == 0 {
            return Color::BLACK;
        }

        if shade_computation.obj.get_material().reflective == 0.0 {
            return Color::BLACK;
        }
        let reflect_ray = Ray::new(shade_computation.over_point, shade_computation.reflectv);
        let color = color_at(self, reflect_ray, reflect_rays_remaining - 1);
        return color * shade_computation.obj.get_material().reflective;
    }

    pub fn refracted_color(&self, shade_computation: &ShadeComputation, reflectrays_remaining: usize) -> Color {
        // Compute snell's law 
        // sin(theta_i) / sin(theta_t) = n1/n2 
        
        //println!("refracted color eyev: {:?}\npoint: {:?}\nunder_point: {:?}\nn1: {:?},\nn2: {:?},\nobj: {:?}",
        //         shade_computation.eyev,
        //         shade_computation.point,
        //         shade_computation.under_point,
        //         shade_computation.n1,
        //         shade_computation.n2,
        //         *shade_computation.obj);
        let n1_n2_ratio = shade_computation.n1 / shade_computation.n2;
        let cos_theta_i  = shade_computation.eyev.dot( shade_computation.normalv);
        let sin2_theta_t = n1_n2_ratio.powi(2) * (1.0 - cos_theta_i.powi(2));


        if reflectrays_remaining == 0 || sin2_theta_t > 1.0 || shade_computation.obj.get_material().transparency == 0.0 {
            return Color::BLACK;
        }

        let cos_t = (1.0 - sin2_theta_t).sqrt();
         
        let refracted_ray_dir = (n1_n2_ratio * cos_theta_i - cos_t) * shade_computation.normalv - 
            n1_n2_ratio * shade_computation.eyev;
        let refracted_ray = Ray::new(shade_computation.under_point, refracted_ray_dir);
        let refracted_color_at = color_at(self, refracted_ray, reflectrays_remaining - 1);
        let refracted_color =  refracted_color_at * 
            shade_computation.obj.get_material().transparency;
        refracted_color 
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new();
        let mut s1 = Sphere::new();
        let mut mat_s1 = Material::new(Color::new(0.8, 1.0, 0.6));
        mat_s1.diffuse = 0.7;
        mat_s1.specular = 0.2;
        s1.set_material(mat_s1);

        w.objects.push(Shape::Sphere(s1));
        let mut s2 = Sphere::new();
        //s2.set_material(mat_s1);
        s2.set_transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        w.objects.push(Shape::Sphere(s2));
        w
    }
}
