//

use crate::bvh_node::BVHNode;
use crate::hittable::aarect::*;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::instance::{RotateY, Translate};
use crate::hittable::sphere::Sphere;
use crate::hittable_list::HittableList;
use crate::material::*;
use crate::obj_file::load;
use crate::rectbox::RectBox;
use crate::texture::*;
use crate::vec3::{Color, Vec3};
use rand::Rng;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub struct Block {
    pub height: f64,
    pub id: u8,
    pub occupied: bool,
    pub decoration: u8,
}

impl Block {
    pub fn new() -> Self {
        Self {
            height: 0.0,
            id: 0,
            occupied: false,
            decoration: 0,
        }
    }

    pub fn create(boxes_per_side: usize, offset: f64) -> Vec<Vec<Block>> {
        let mut rng = rand::thread_rng();
        let noise = NoiseTexture::new(0.03);
        let mut map: Vec<Vec<Block>> = vec![vec![Block::new(); boxes_per_side]; boxes_per_side];
        for (i, it) in map.iter_mut().enumerate().take(boxes_per_side) {
            for (j, block) in it.iter_mut().enumerate().take(boxes_per_side) {
                let w = 1.0;
                let x0 = (i as f64 - 25.0) * w;
                let z0 = (j as f64 - 25.0) * w;
                let h = (noise.value(x0, -10.0, Vec3::new(x0, -10.0, z0)).x
                    + (x0 * 3.0 + z0 + 3.0).max(1.0).ln() * 0.2)
                    * (x0 + z0 + 0.5).max(0.3).ln()
                    * 2.5
                    + offset;
                if (0.0..4.0).contains(&h) {
                    // beach
                    block.id = if rng.gen_range(2.5..4.0) > h { 2 } else { 1 };
                    block.height = (h / 2.0).floor();
                } else if h >= 4.0 {
                    // normal
                    block.id = 1;
                    block.height = ((h - 4.0) / (j.max(i) as f64 / i as f64)).floor() + 2.0;
                    let select = rng.gen::<f64>();
                    if !block.occupied
                        && select
                            > (1.0 - ((i as f64 * 1.25).max(j as f64) - j as f64) / 500.0)
                                .max(0.995)
                    {
                        // have tree
                        block.decoration = 1;
                        block.occupied = true;
                    } else if select > 0.9 {
                        // have flower
                        block.decoration = 2;
                    }
                } else {
                    // sea
                    block.height = ((-h / 2.0 + 1.0).ln() * -3.0).floor();
                    let select = rng.gen::<f64>();
                    if !block.occupied && select > (1.03 + h / 30.0).max(0.92) {
                        // have coral
                        block.decoration = 3;
                        block.occupied = true;
                    }
                }
            }
        }
        map
    }

    pub fn tree(x: f64, y: f64, z: f64, list: &mut HittableList) {
        let log = Lambertian::new(SolidColor::new(Color::new(0.3, 0.23, 0.14)));
        let leaf = Lambertian::new(SolidColor::new(Color::new(0.25, 0.35, 0.11)));

        list.add(Box::new(XYRect::new(x, x + 1.0, y, y + 3.0, z, log)));
        list.add(Box::new(YZRect::new(y, y + 3.0, z, z + 1.0, x, log)));

        list.add(Box::new(XZRect::new(
            x - 1.0,
            x + 2.0,
            z - 1.0,
            z + 2.0,
            y + 6.0,
            leaf,
        )));
        list.add(Box::new(XZRect::new(
            x - 2.0,
            x + 3.0,
            z - 1.0,
            z + 2.0,
            y + 5.0,
            leaf,
        )));
        list.add(Box::new(XZRect::new(
            x - 1.0,
            x + 2.0,
            z - 2.0,
            z + 3.0,
            y + 5.0,
            leaf,
        )));

        list.add(Box::new(YZRect::new(
            y + 3.0,
            y + 5.0,
            z - 2.0,
            z + 3.0,
            x - 1.0,
            leaf,
        )));
        list.add(Box::new(YZRect::new(
            y + 4.9,
            y + 6.0,
            z - 1.0,
            z + 2.0,
            x - 1.0,
            leaf,
        )));
        list.add(Box::new(YZRect::new(
            y + 3.0,
            y + 5.0,
            z - 1.0,
            z + 2.0,
            x - 2.0,
            leaf,
        )));

        list.add(Box::new(XYRect::new(
            x - 2.0,
            x + 3.0,
            y + 3.0,
            y + 5.0,
            z - 1.0,
            leaf,
        )));
        list.add(Box::new(XYRect::new(
            x - 1.0,
            x + 2.0,
            y + 4.9,
            y + 6.0,
            z - 1.0,
            leaf,
        )));
        list.add(Box::new(XYRect::new(
            x - 1.0,
            x + 2.0,
            y + 3.0,
            y + 5.0,
            z - 2.0,
            leaf,
        )));
    }

    pub fn flower(x: f64, y: f64, z: f64, list: &mut HittableList) {
        let c = (Color::random() + Color::one() * 0.5) * 0.66;
        let mat = Lambertian::new(SolidColor::new(c));
        let mut rng = rand::thread_rng();
        let offset = rng.gen::<f64>() * 0.5;
        list.add(Box::new(XZRect::new(
            x + offset,
            x + offset + 0.3,
            z + offset,
            z + offset + 0.3,
            y + rng.gen::<f64>() / 4.0,
            mat,
        )));
        list.add(Box::new(XZRect::new(
            x + 1.0 - offset - 0.35,
            x + 1.0 - offset,
            z + offset + 0.25,
            z + offset + 0.6,
            y + rng.gen::<f64>() / 5.0 + 0.05,
            mat,
        )));
        list.add(Box::new(XZRect::new(
            x + 1.0 - offset,
            x + 1.0 - offset + 0.3,
            z + 1.0 - offset - 0.3,
            z + 1.0 - offset,
            y + rng.gen::<f64>() / 3.0,
            mat,
        )));
    }

    pub fn coral(x: f64, y: f64, z: f64, list: &mut HittableList) {
        let c = (Color::random() + Color::one() * 0.5) * 1.0;
        let mat = DiffuseLight::new(SolidColor::new(c));
        let mut rng = rand::thread_rng();
        let offset = rng.gen::<f64>() * 0.8;
        list.add(Box::new(XZRect::new(
            x + offset,
            x + offset + 0.6,
            z + offset,
            z + offset + 0.6,
            y + rng.gen::<f64>() / 4.0,
            mat,
        )));
        list.add(Box::new(XZRect::new(
            x + 1.0 - offset - 0.55,
            x + 1.0 - offset,
            z + offset - 0.25,
            z + offset + 0.3,
            y + rng.gen::<f64>() / 5.0 + 0.05,
            mat,
        )));
        list.add(Box::new(XZRect::new(
            x + 1.0 - offset,
            x + 1.0 - offset + 0.6,
            z + 1.0 - offset - 0.6,
            z + 1.0 - offset,
            y + rng.gen::<f64>() / 3.0,
            mat,
        )));
        let offset = rng.gen::<f64>() * 0.5;
        list.add(Box::new(XZRect::new(
            x + offset,
            x + offset + 0.3,
            z + offset,
            z + offset + 0.3,
            y + rng.gen::<f64>() / 4.0 + 0.5,
            mat,
        )));
        list.add(Box::new(XZRect::new(
            x + 1.0 - offset - 0.35,
            x + 1.0 - offset,
            z + offset + 0.25,
            z + offset + 0.6,
            y + rng.gen::<f64>() / 5.0 + 0.55,
            mat,
        )));
        list.add(Box::new(XZRect::new(
            x + 1.0 - offset,
            x + 1.0 - offset + 0.3,
            z + 1.0 - offset - 0.3,
            z + 1.0 - offset,
            y + rng.gen::<f64>() / 3.0 + 0.5,
            mat,
        )));
        list.add(Box::new(RectBox::new(
            Vec3::new(x + 0.2, y + 0.1, z + 0.2),
            Vec3::new(x + 0.8, y + 0.7, z + 0.8),
            ColoredDielectric::new(1.5, 0.2, c),
        )));
    }

    // 像素世界生成
    pub fn the_world() -> HittableList {
        let mut boxes1 = HittableList::new();
        let mud = Lambertian::new(SolidColor::new(Color::new(0.36, 0.25, 0.16)));
        let grass = Lambertian::new(SolidColor::new(Color::new(0.50, 0.72, 0.36)));
        let sand = Lambertian::new(SolidColor::new(Color::new(0.87, 0.84, 0.67)));

        // let tree = Lambertian::new(SolidColor::new(Color::zero()));
        // let flower = Lambertian::new(SolidColor::new(Color::new(1.0, 0.0, 0.0)));
        // let coral = Lambertian::new(SolidColor::new(Color::new(0.0, 0.0, 1.0)));

        let boxes_per_side = 250;
        let map = Block::create(boxes_per_side, -6.0);
        // Self::flower(8.0, 14.0, 8.0, &mut boxes1);
        // boxes1.add(Box::new(RectBox::new(
        //     Vec3::new(8.0, 13.0, 8.0),
        //     Vec3::new(9.0, 14.0, 9.0),
        //     mud,
        // )));

        for (i, it) in map.into_iter().enumerate() {
            for (j, block) in it.into_iter().enumerate() {
                if (0.3..3.5).contains(&(j as f32 / i as f32)) && (j as i32 + i as i32) > 40 {
                    let w = 1.0;
                    let x0 = i as f64 * w;
                    let z0 = j as f64 * w;
                    let y0 = block.height - 1.0;
                    let x1 = x0 + w;
                    let z1 = z0 + w;
                    let y1 = block.height;

                    let block_mat = match block.id {
                        1 => mud,
                        2 => sand,
                        _ => sand,
                    };
                    boxes1.add(Box::new(XZRect::new(x0, x1, z0, z1, y1, block_mat)));
                    boxes1.add(Box::new(YZRect::new(y0 - 0.1, y1, z0, z1, x0, block_mat)));
                    boxes1.add(Box::new(XYRect::new(x0, x1, y0 - 0.1, y1, z0, block_mat)));

                    match block.id {
                        1 => {
                            if !block.occupied {
                                boxes1.add(Box::new(XZRect::new(
                                    x0 - 0.001,
                                    x1 + 0.001,
                                    z0 - 0.001,
                                    z1 + 0.001,
                                    y1 + 0.001,
                                    grass,
                                )));
                                boxes1.add(Box::new(YZRect::new(
                                    y1 - 0.2,
                                    y1 + 0.001,
                                    z0 - 0.001,
                                    z1 + 0.001,
                                    x0 - 0.001,
                                    grass,
                                )));
                                boxes1.add(Box::new(XYRect::new(
                                    x0 - 0.001,
                                    x1 + 0.001,
                                    y1 - 0.2,
                                    y1 + 0.001,
                                    z0 - 0.001,
                                    grass,
                                )));
                            }
                            match block.decoration {
                                1 => {
                                    Self::tree(x0, y1, z0, &mut boxes1);
                                }
                                2 => {
                                    Self::flower(x0, y1, z0, &mut boxes1);
                                }
                                _ => (),
                            }
                        }
                        2 => {}
                        _ => {
                            if block.decoration == 3 {
                                Self::coral(x0, y1, z0, &mut boxes1);
                            }
                        }
                    }
                }
            }
        }
        let mut objects = HittableList {
            objects: vec![BVHNode::create(boxes1, 0.0, 1.0)],
        };

        let log = Lambertian::new(SolidColor::new(Color::new(0.3, 0.23, 0.14)));
        let obj = load("raytracer/src/obj/cottage_obj.obj".to_string(), log, 0.4);
        objects.add(Box::new(Translate::new(
            Box::new(RotateY::new(Box::new(HittableList { objects: obj }), 180.0)),
            Vec3::new(32.0, 1.0, 56.0),
        )));

        objects.add(Box::new(RectBox::new(
            Vec3::new(28.0, 0.0, 42.0),
            Vec3::new(29.0, 0.6, 43.0),
            DiffuseLight::new(SolidColor::new(Color::new(10.0, 8.0, 5.0))),
        )));
        objects.add(Box::new(RectBox::new(
            Vec3::new(27.8, 0.0, 41.8),
            Vec3::new(29.2, 0.25, 43.2),
            ColoredDielectric::new(1.5, 0.05, Color::new(0.1, 0.1, 0.1)),
        )));

        let surface = XZRect::new(
            0.0,
            boxes_per_side as f64,
            0.0,
            boxes_per_side as f64,
            -0.1,
            // Dielectric::new(1.5, 0.0),
            ColoredDielectric::new(1.5, 0.0, Color::new(0.83, 0.91, 0.97)),
        );
        objects.add(Box::new(surface));
        let sea = RectBox::new(
            Vec3::new(0.0, -5.0, 0.0),
            Vec3::new(boxes_per_side as f64, -0.1, boxes_per_side as f64),
            Dielectric::new(1.5, 0.0),
        );
        objects.add(Box::new(ConstantMedium::new(
            Box::new(sea),
            0.02,
            Isotropic::new(SolidColor::new(Color::new(0.83, 0.91, 0.97))),
        )));
        // let sky = RectBox::new(
        //     Vec3::new(0.0, -0.1, 0.0),
        //     Vec3::new(boxes_per_side as f64, 500.0, boxes_per_side as f64),
        //     Dielectric::new(1.5, 0.0),
        // );
        // objects.add(Box::new(ConstantMedium::new(
        //     Box::new(sky),
        //     0.002,
        //     Isotropic::new(SolidColor::new(Color::one())),
        // )));

        objects.add(Box::new(Sphere::new(
            Vec3::new(-10.0, -30.0, 20.0),
            5.0,
            DiffuseLight::new(SolidColor::new(Color::new(10.0, 10.0, 10.0))),
        )));

        objects
    }
}
