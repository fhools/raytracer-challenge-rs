use crate::LightSource;
use crate::Sphere;
use crate::Matrix4x4;
use crate::Shape;
use crate::Intersection;
use crate::Intersections;
use crate::Intersectable;
use crate::Material;
use crate::Color;
use crate::Ray;
use crate::Vector4D;
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
                Shape::Sphere(sph) => {
                    vs.extend(ray.intersect(sph));
                }
            }
        }
        vs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        vs = vs.iter().filter(|a| a.t >= 0.0).cloned().collect::<Vec<Intersection>>();
        vs
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
