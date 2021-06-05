use crate::Vector4D;
use crate::Matrix4x4;
use crate::Intersectable;
use crate::Intersection;
use crate::Intersections;
use crate::World;
use crate::Shape;
use crate::Color;
use crate::lighting;
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
        let mut p =  self.origin + t * self.direction;
        p
    }


    pub fn intersect<S: Intersectable>(&self, shape: &S) -> Vec<Intersection> {
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
        match *intersection.obj {
            Shape::Sphere(sph) => {
                normalv = sph.normal_at(p);
                obj = Shape::Sphere(sph);
            }
        }

        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        ShadeComputation {
            t: intersection.t,
            obj: Box::new(obj),
            point: p,
            eyev: eyev,
            normalv: normalv,
            inside: inside
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
}

pub fn shade_hit(world: &World, sc: &ShadeComputation) -> Color {
    let Shape::Sphere(s) = *sc.obj;
    lighting(s.material, world.light_source, sc.point, sc.eyev, sc.normalv)
}

pub fn color_at(world: &World, ray: Ray) -> Color {
    let xs = ray.intersect_world(world);
    if xs.len() == 0 {
        Color::black()
    } else {
        let intersect = &xs[0];
        let sc = ray.prepare_computations(intersect);
        shade_hit(world, &sc)
    }
}
