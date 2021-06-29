use std::ptr;

use crate::Vector4D;
use crate::Matrix4x4;
use crate::Intersectable;
use crate::Intersection;
use crate::Intersections;
use crate::World;
use crate::Shape;
use crate::Color;
use crate::lighting;
use crate::hit;
use crate::reflect;
use crate::refractive_indices;

use utils::*;
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector4D,
    pub direction: Vector4D
}

impl Ray {
    pub fn new(origin: Vector4D, direction: Vector4D) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> Vector4D {
        self.origin
    }

    pub fn dir(&self) -> Vector4D {
        self.direction
    }

    pub fn at_t(&self, t: f64) -> Vector4D {
        let p =  self.origin + t * self.direction;
        p
    }


    pub fn intersect<S: Intersectable>(&self, shape: &S) -> Intersections {
        shape.intersect(self)
    }

    pub fn intersect_world(&self, world: &World) -> Intersections {
        world.intersect(self)
    }

    // Given an intersection at an object for the ray, compute various things needed for rendering
    // the object, 
    // the normal at the point of intersection
    // the point of intersection,
    // the point slightly above the point of intersection
    // (used for shadow and reflection caclulation),
    // whether the intersection is inside the object
    // the reflection vector of the ray
    pub fn prepare_computations(&self, intersection: &Intersection, xs: &Intersections) -> ShadeComputation {
        let p = self.at_t(intersection.t);
        let eyev = -self.dir();
        let mut normalv;
        let obj;
        let inside;
        match &*intersection.obj {
            Shape::Sphere(sph) => {
                normalv = sph.normal_at(p);
                obj = Shape::Sphere(sph.clone());
            },
            Shape::TestShape(t) => {
                normalv = t.normal_at(p);
                obj = Shape::TestShape(t.clone());
            }
            Shape::Plane(plane) => {
                normalv = plane.normal_at(p);
                obj = Shape::Plane(plane.clone());
            },
            Shape::Cube(cube) => {
                normalv = cube.normal_at(p);
                obj = Shape::Cube(cube.clone());
            },
            Shape::Cylinder(o) => {
                normalv = o.normal_at(p);
                obj = Shape::Cylinder(o.clone());
            },
            Shape::Cone(o) => {
                normalv = o.normal_at(p);
                obj = Shape::Cone(o.clone());
            },
        }

        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        // =================================
        // compute the n1 n2 refraction values
        let mut refract_n1 = refractive_indices::VACUUM;
        let mut refract_n2 = refractive_indices::VACUUM;
        let hit : &Intersection = intersection;
        let mut containers: Vec<Box<Shape>> = vec![];
        for  i in xs.iter() {
            let i_obj = &i.obj; 
            let hit_obj = &hit.obj;
            if f64_eq(hit.t, i.t) {
                if containers.len() == 0 {
                    refract_n1 = refractive_indices::VACUUM; 
                } else {
                    refract_n1 = containers.last().unwrap().get_material().refractive_index; 
                }
            }

            let index = containers.iter().position(|other| (*other).eq(&*i.obj));

            match index {
                Some(index) => {
                   containers.remove(index);
                },
                None => {
                    containers.push(Box::new((*i.obj).clone()));
                }
            }
            if f64_eq(hit.t, i.t) {
                if containers.len() == 0 {
                    refract_n2 = refractive_indices::VACUUM;
                } else {
                    refract_n2 = containers.last().unwrap().get_material().refractive_index;
                }
                break;
            }

        }
        // Used during shadow computation, so that shadow ray doesn't intersect the point of
        // intersection at object, we compute a point above the normal
        let over_point = p + utils::SHADOW_EPSILON*normalv; // direciton of normal above point of intersection
        let under_point = p - utils::SHADOW_EPSILON*normalv; // direction away from normal below point of intersection
        let sc = ShadeComputation {
            t: intersection.t,
            obj: Box::new(obj),
            point: p,
            eyev: eyev,
            normalv: normalv,
            inside: inside,
            over_point: over_point,
            under_point: under_point,
            reflectv: reflect(self.dir(), normalv),
            n1: refract_n1,
            n2: refract_n2, 
            do_debug: None,
        };
        sc
        
    }

    pub fn transform(&self, m: &Matrix4x4) -> Ray {
        Ray {
            origin: m.mul_vector4d(&self.origin),
            direction: m.mul_vector4d(&self.direction)
        }
    }
}

#[derive(Debug)]
pub struct ShadeComputation {
    pub t: f64,
    pub obj: Box<Shape>,
    pub point: Vector4D,
    pub eyev: Vector4D,
    pub normalv: Vector4D,
    pub inside: bool,
    pub over_point: Vector4D,
    pub under_point: Vector4D,
    pub reflectv: Vector4D,
    pub n1: f64, // Refraction index 1
    pub n2: f64, // Refraction index 2
    // Debug
    pub do_debug : Option<bool>
}
pub static mut global_do_debug: Option<bool> = None;

pub fn shade_hit(world: &World, sc: &ShadeComputation, reflect_rays_remaining: usize) -> Color {
    unsafe {
    if let Some(_) = sc.do_debug {
        global_do_debug = Some(true);
    }
    }
    let surface =  lighting(sc.obj.get_material(), 
                            &*sc.obj, 
                            world.light_source,
                            sc.over_point, sc.eyev, sc.normalv, world.is_shadowed(sc.over_point));
    let reflected = world.reflected_color(sc, reflect_rays_remaining);
    let refracted = world.refracted_color(sc, reflect_rays_remaining);
    if let Some(_) = sc.do_debug {
    println!("shade_hit: {} surface color: {:?} reflected color: {:?}\nrefracted color: {:?}",
             reflect_rays_remaining, surface, reflected, refracted);
    }

    //DEBUG , clear debug 
    unsafe {
    global_do_debug = None;
    }
    let mut m  = sc.obj.get_material();
    if m.reflective > 0.0 && m.transparency > 0.0 {
        let reflectance = schlick(&sc);
        return surface + reflected*reflectance + (1.0  - reflectance)*refracted;
    } else { 
        let total_color = surface + reflected + refracted;
        return total_color; 
    }
}

pub fn color_at(world: &World, ray: Ray, remaining: usize) -> Color {
    let xs = ray.intersect_world(world);
    let mut do_debug = false;
    // Debug
    if ray.origin().x == 0.0 && ray.origin().y == 5.0 && ray.origin().z == 0.0 && 
        f64_eq(ray.dir().x, -0.48060956)  && f64_eq(ray.dir().y, -0.8408486) && f64_eq(ray.dir().z, 0.2489737) {
            println!("bad ray: {:?}\nxs{:?}", ray, xs);
            do_debug = true;
    }
    if let Some(hit) = hit(&xs) {
        let mut sc = ray.prepare_computations(&hit, &xs);
        if do_debug {
            sc.do_debug = Some(true);
            println!("sc:{:?}", sc);
        }
        //DEBUG
        let color = shade_hit(world, &sc, remaining);
        if ray.origin().x == 0.0 && ray.origin().y == 5.0 && ray.origin().z == 0.0 && 
            f64_eq(ray.dir().x, -0.48060956)  && f64_eq(ray.dir().y, -0.8408486) && f64_eq(ray.dir().z, 0.2489737) {
                println!("bad ray: {:?}\n color: {:?}", ray, color);
        }
        color
    } else {
            if ray.origin().x == 0.0 && ray.origin().y == 5.0 && ray.origin().z == 0.0 && 
                f64_eq(ray.dir().x, -0.48060956)  && f64_eq(ray.dir().y, -0.8408486) && f64_eq(ray.dir().z, 0.2489737) {
                println!("bad ray: {:?}\n color black", ray);
            }
        Color::BLACK
    }
}

pub fn schlick(sc: &ShadeComputation) -> f64 {
    let mut cos_eye_normal = sc.eyev.dot(sc.normalv);
    if sc.n1 > sc.n2 {
        let n = sc.n1 / sc.n2;
        let sin2_t = n.powi(2) * (1.0 - cos_eye_normal.powi(2));
        if sin2_t > 1.0 {
            return 1.0
        }
        let cos_t = (1.0 - sin2_t.powi(2)).sqrt();
        cos_eye_normal = cos_t;
    }
    let r0 = ((sc.n1 - sc.n2)/(sc.n1+sc.n2)).powi(2);
    return r0 + (1.0 - r0)*(1.0 - cos_eye_normal).powi(5);
}
