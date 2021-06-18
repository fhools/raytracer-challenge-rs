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

    pub fn prepare_computations(&self, intersection: &Intersection) -> ShadeComputation {
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

        // Used during shadow computation, so that shadow ray doesn't intersect the point of
        // intersection at object, we compute a point above the normal
        let over_point = p + utils::EPSILON*normalv;
        ShadeComputation {
            t: intersection.t,
            obj: Box::new(obj),
            point: p,
            eyev: eyev,
            normalv: normalv,
            inside: inside,
            over_point: over_point,
            reflectv: reflect(self.dir(), normalv)
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
    pub reflectv: Vector4D,
}

pub fn shade_hit(world: &World, sc: &ShadeComputation, remaining: usize) -> Color {
    let shape : &dyn Intersectable;
    match *sc.obj {
        Shape::Sphere(ref s) => {
            shape = s;
        },
        Shape::TestShape(ref t) => {
            shape = t;
        },
        Shape::Plane(ref p) => {
            shape =p;
        }
    }
    let surface =  lighting(shape.get_material(), shape, world.light_source, sc.over_point, sc.eyev, sc.normalv, world.is_shadowed(sc.over_point));
    let reflected = world.reflected_color(sc, remaining);
    return surface + reflected;
}

pub fn color_at(world: &World, ray: Ray, remaining: usize) -> Color {
    let xs = ray.intersect_world(world);
    if let Some(xs) = hit(&xs) {
        let sc = ray.prepare_computations(&xs);
        shade_hit(world, &sc, remaining)
    } else {
        Color::BLACK
    }
}
