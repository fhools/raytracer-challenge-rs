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
            }
        }

        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        let mut refract_n1 = refractive_indices::VACUUM;
        let mut refract_n2 = refractive_indices::VACUUM;
        let hit : &Intersection = intersection;
        let mut containers: Vec<Box<Shape>> = vec![];
        for  i in xs.iter() {
            if f64_eq(i.t, hit.t) {
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
            if f64_eq(i.t, hit.t) {
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
        let over_point = p + utils::EPSILON*normalv;
        let under_point = p - utils::EPSILON*normalv;
        ShadeComputation {
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
        }
    }

    pub fn transform(&self, m: &Matrix4x4) -> Ray {
        Ray {
            origin: m.mul_vector4d(&self.origin),
            direction: m.mul_vector4d(&self.direction)
        }
    }
}

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
}

pub fn shade_hit(world: &World, sc: &ShadeComputation, reflect_rays_remaining: usize) -> Color {
    let shape : &dyn Intersectable;
    match *sc.obj {
        Shape::Sphere(ref s) => {
            shape = s;
        },
        Shape::TestShape(ref t) => {
            shape = t;
        },
        Shape::Plane(ref p) => {
            shape = p;
        }
    }
    let surface =  lighting(shape.get_material(), shape, world.light_source, sc.over_point, sc.eyev, sc.normalv, world.is_shadowed(sc.over_point));
    let reflected = world.reflected_color(sc, reflect_rays_remaining);
    let refracted = world.refracted_color(sc, reflect_rays_remaining);

    let mut m  = sc.obj.get_material();
    if m.reflexivity > 0.0 && m.transparency > 0.0 {
        let reflectance = schlick(&sc);
        return surface + reflected*reflectance + (1.0  - reflectance)*refracted;
    } else { 
        return surface + reflected + refracted;
    }
}

pub fn color_at(world: &World, ray: Ray, remaining: usize) -> Color {
    let xs = ray.intersect_world(world);
    if let Some(hit) = hit(&xs) {
        let sc = ray.prepare_computations(&hit, &xs);
        shade_hit(world, &sc, remaining)
    } else {
        Color::BLACK
    }
}

pub fn schlick(sc: &ShadeComputation) -> f64 {
    let mut cos_eye_normal = sc.eyev.dot(sc.normalv);
    if sc.n1  > sc.n2 {
        let n = sc.n1 / sc.n2;
        let sin2_t = n.powf(2.0) * (1.0 - cos_eye_normal.powf(2.0));
        if sin2_t > 1.0 {
            return 1.0
        }
        let cos_t = (1.0 - sin2_t.powf(2.0)).sqrt();
        cos_eye_normal = cos_t;
    }
    let r0 = ((sc.n1 - sc.n2)/(sc.n1+sc.n2)).powf(2.0);
    return r0 + (1.0 - r0)*(1.0 - cos_eye_normal).powf(5.0);
}
