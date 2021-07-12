use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render_with_reflection};
use types::refractive_indices;

#[test]
fn test_new_group() {
    let g = Group::new(0);
    assert_eq!(g.is_empty(), true);
}

#[test]
fn test_testshape_no_parent() {
    let t = TestShape::new();
    assert_eq!(t.get_parent().is_none(), true);
}

#[test]
fn test_group_add() {
    let mut g = Group::new(700);
    let t = TestShape::new();
    g.add_child(Shape::TestShape(t)); 
    let parent = g.children[0].get_parent().unwrap();
    assert_eq!(parent.id, 700);
}

#[test]
fn test_group_intersect() {
    let mut g = Group::new(600);
    let s1 = Sphere::new();
    let mut s2 = Sphere::new();
    s2.set_transform(Matrix4x4::translation(0.0, 0.0, -3.0));

    let mut s3 = Sphere::new();
    s3.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));

    g.add_child(Shape::Sphere(s1.clone()));
    g.add_child(Shape::Sphere(s2.clone()));
    g.add_child(Shape::Sphere(s3.clone()));
    
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&ray);
    println!("xs: {:?}", xs);
    assert_eq!(xs.len(), 4);
    assert!(xs[0].obj.eq(&Shape::Sphere(s2.clone())));
    assert!(xs[1].obj.eq(&Shape::Sphere(s2.clone())));
    assert!(xs[2].obj.eq(&Shape::Sphere(s1.clone())));
    assert!(xs[3].obj.eq(&Shape::Sphere(s1.clone())));
}


#[test]
fn test_group_transformed_instersect() {
    let mut g = Group::new(500);
    g.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    g.add_child(Shape::Sphere(s.clone()));
    let ray = Ray::new(Vector4D::new_point(10.0, 0.0, -10.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&ray);
    assert_eq!(xs.len(), 2);
}


#[test]
fn test_group_world_to_object_space() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    let mut g2 = Group::new(400);
    g2.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    g2.add_child(Shape::Sphere(s));
    let mut g1 = Group::new(401);
    g1.set_transform(Matrix4x4::rotate_y(PI/2.0));
    g1.add_child(Shape::Group(g2));

    match g1.children[0] {
        Shape::Group(ref g) => {
            let point = g.children[0].world_to_object(Vector4D::new_point(-2.0, 0.0, -10.0));
            assert_vector4d_eq!(point, Vector4D::new_point(0.0, 0.0, -1.0));
        }
        _ => {}
    }
}

#[test]
fn test_normal_to_world() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    let mut g2 = Group::new(200);
    g2.set_transform(Matrix4x4::scaling(1.0, 2.0, 3.0));
    g2.add_child(Shape::Sphere(s));
    let mut g1 = Group::new(201);
    g1.set_transform(Matrix4x4::rotate_y(PI/2.0));
    g1.add_child(Shape::Group(g2));

    match g1.children[0] {
        Shape::Group(ref g) => {
            let normal = g.children[0].normal_to_world(Vector4D::new_vector(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));
            assert_vector4d_eq!(normal, Vector4D::new_vector(0.2857, 0.4286, -0.8571));
        }
        _ => {}
    }
}

#[test]
fn test_normal_child_object() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    let mut g2 = Group::new(300);
    g2.set_transform(Matrix4x4::scaling(1.0, 2.0, 3.0));
    g2.add_child(Shape::Sphere(s));
    let mut g1 = Group::new(301);
    g1.set_transform(Matrix4x4::rotate_y(PI/2.0));
    g1.add_child(Shape::Group(g2));

    match g1.children[0] {
        Shape::Group(ref g) => {
            let normal = g.children[0].normal_at(Vector4D::new_point(1.7321, 1.11547, -5.5774));
            assert_vector4d_eq!(normal, Vector4D::new_vector(0.28747, 0.41654, -0.862466));
        }
        _ => {}
    }
}



fn hexagon_corner() -> Shape {
    let mut corner = Sphere::new();
    let mut obj_mat: Material = Default::default();
    obj_mat.color = Color::new(1.0, 0.8431, 0.0);
    obj_mat.reflective = 0.0;
    obj_mat.refractive_index = refractive_indices::GLASS;
    obj_mat.transparency = 0.0;
    corner.set_material(obj_mat);
    corner.set_transform(MatrixChainer::new()
                         .then(Matrix4x4::scaling(0.25, 0.25, 0.25))
                         .then(Matrix4x4::translation(0.0, 0.0, -1.0))
                         .finish());
    Shape::Sphere(corner)
}

fn hexagon_edge() -> Shape {
    let mut edge = Cylinder::new();
    let mut obj_mat: Material = Default::default();
    obj_mat.color = Color::new(1.0, 0.8431, 0.0);
    obj_mat.reflective = 0.0;
    obj_mat.refractive_index = refractive_indices::GLASS;
    obj_mat.transparency = 0.0;
    edge.set_material(obj_mat);
    edge.minimum = 0.0;
    edge.maximum = 1.0;
    edge.set_transform(MatrixChainer::new()
                       .then(Matrix4x4::scaling(0.25, 1.0, 0.25))
                       .then(Matrix4x4::rotate_z(-PI/2.0))
                       .then(Matrix4x4::rotate_y(-PI/6.0))
                       .then(Matrix4x4::translation(0.0, 0.0, -1.0))
                       .finish());
    Shape::Cylinder(edge)
}

fn hexagon_side(id: usize) -> Group {
    let mut side = Group::new(id);
    side.add_child(hexagon_corner());
    side.add_child(hexagon_edge());
    side 
}

fn hexagon(mut id: usize) -> Shape {
    let mut hexagon = Group::new(id);
    id += 2000;
    for i in 0..6 {
        let mut side = hexagon_side(id);
        side.set_transform(Matrix4x4::rotate_y(i as f64 * PI/3.0));
        hexagon.add_child(Shape::Group(side));
        id += i;
    }

    Shape::Group(hexagon)
}

#[test]
//#[ignore="render"]
fn test_render_gold_hexagon() {
    let mut world: World = Default::default();
    world.objects.clear();

    let mut obj = hexagon(5000); 
    obj.set_transform(MatrixChainer::new()
                        .then(Matrix4x4::rotate_x(PI/6.0))
                       .then(Matrix4x4::translation(0.2, -2.0, 0.3))
                       .then(Matrix4x4::scaling(1.7, 1.7, 1.7))
                       .finish());
    world.objects.push(obj);

    let mut wall = Plane::new();
    wall.set_transform(MatrixChainer::new()
                       .then(Matrix4x4::translation(0.0, -6.0, 0.0))
                       .finish());
    let mut wall_mat: Material = Default::default();
    wall_mat.color = Color::new(1.0, 0.8431, 0.0);
    wall_mat.specular = 0.8;
    wall_mat.transparency = 0.0;
    wall_mat.refractive_index = 1.83;
    wall_mat.pattern = Some(Box::new(Pattern::CheckeredPattern(CheckeredPattern::new(Color::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)))));
    wall_mat.no_cast_shadow = true;
    wall.set_material(wall_mat);
    world.objects.push(Shape::Plane(wall));

    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    const WIDTH_PX: usize = 800;
    const HEIGHT_PX: usize = 600;
    world.light_source = LightSource::new(Color::new(1.0, 1.0, 1.0), 
                                          Vector4D::new_point(-10.0, 10.0, -25.0));

    let mut c = Camera::new(WIDTH_PX, HEIGHT_PX, PI/3.0);
    let from = Vector4D::new_point(0.0, 5.0, 0.0);
    let to = Vector4D::new_point(0.0, -1.0, 0.0);
    let up = Vector4D::new_vector(0.0, 0.0, 1.0);
    c.transform = view_transformation(from, to, up); 
    render_with_reflection(&c, &world, &mut canvas);

    canvas.write_ppm("ch14_gold_hexagon.ppm").unwrap();
}
