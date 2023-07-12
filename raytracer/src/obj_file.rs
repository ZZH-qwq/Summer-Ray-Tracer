// .obj 文件导入

use crate::bvh_node::BVHNode;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::triangle::Triangle;
use crate::hittable::*;
use crate::material::Material;
use crate::vec3::Vec3;
use std::string::String;
use tobj::{load_obj, LoadOptions};

pub fn load<M: 'static + Material + Clone + Copy>(
    file_name: String,
    material: M,
) -> Vec<Box<dyn Hittable>> {
    let obj = load_obj(
        file_name,
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    assert!(obj.is_ok());

    let (models, _) = obj.expect("Failed to load OBJ file");

    let mut objects = vec![];

    for m in models.iter() {
        let indices = &m.mesh.indices;
        let positions = &m.mesh.positions;
        let mut faces = HittableList::new();
        let mut points = vec![];

        for p in (0..positions.len()).step_by(3) {
            points.push(Vec3::new(positions[p], positions[p + 1], positions[p + 2]));
        }

        for f in (0..indices.len()).step_by(3) {
            faces.add(Box::new(Triangle::new(
                points[indices[f] as usize],
                points[indices[f + 1] as usize],
                points[indices[f + 2] as usize],
                material,
            )))
        }

        objects.push(BVHNode::create(faces, 0.0, 1.0));

        // Normals and texture coordinates are also loaded, but not printed in this example
        // println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        // assert!(mesh.positions.len() % 3 == 0);
        // for v in 0..mesh.positions.len() / 3 {
        //     println!(
        //         "    v[{}] = ({}, {}, {})",
        //         v,
        //         mesh.positions[3 * v],
        //         mesh.positions[3 * v + 1],
        //         mesh.positions[3 * v + 2]
        //     );
        // }
    }

    objects
}
