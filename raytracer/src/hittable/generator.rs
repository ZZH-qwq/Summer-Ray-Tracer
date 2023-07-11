// 生成器

use crate::bvh_node::BVHNode;
use crate::hittable::aarect::*;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::instance::*;
use crate::hittable::moving_sphere::MovingSphere;
use crate::hittable::rectbox::RectBox;
use crate::hittable::sphere::Sphere;
use crate::material::*;
use crate::texture::*;
use crate::vec3::{Color, Vec3};
use rand::Rng;

// 生成随机场景
pub fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(CheckerTexture::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    ));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffus
                    let albedo = SolidColor::new(Color::random() * Color::random());
                    // 添加了运动的球体
                    let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center1,
                        0.0,
                        1.0,
                        0.2,
                        Lambertian::new(albedo),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = 0.5 * Color::one() + 0.5 * Color::random();
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(CheckerTexture::new(
            SolidColor::new(Color::new(0.2, 0.3, 0.1)),
            SolidColor::new(Color::new(0.9, 0.9, 0.9)),
        )),
    )));
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(CheckerTexture::new(
            SolidColor::new(Color::new(0.2, 0.3, 0.1)),
            SolidColor::new(Color::new(0.9, 0.9, 0.9)),
        )),
    )));
    objects
}

// 柏林噪声
pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(NoiseTexture::new(4.0)),
    )));
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(NoiseTexture::new(4.0)),
    )));
    objects
}

// 地球贴图
pub fn earth() -> HittableList {
    let mut objects = HittableList::new();
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Lambertian::new(ImageTexture::new(
            "raytracer/src/texture/img/earthmap.jpg".to_string(),
        )),
    )));

    objects
}

// 发光材质
pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(NoiseTexture::new(4.0)),
    )));
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(NoiseTexture::new(4.0)),
    )));
    objects.add(Box::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))),
    )));
    objects
}

// 康奈尔盒子
pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Box::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    objects.add(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    objects.add(Box::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let box1 = RectBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white,
    );
    let box2 = RectBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    );

    objects.add(Box::new(Translate::new(
        Box::new(RotateY::new(Box::new(box1), 15.0)),
        Vec3::new(265.0, 0.0, 295.0),
    )));
    objects.add(Box::new(Translate::new(
        Box::new(RotateY::new(Box::new(box2), -18.0)),
        Vec3::new(130.0, 0.0, 65.0),
    )));

    objects
}

// 带有烟块的康奈尔盒子
pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)));

    objects.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Box::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    objects.add(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    objects.add(Box::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let box1 = RectBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white,
    );
    let box2 = RectBox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    );

    objects.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(ConstantMedium::new(
                Box::new(box1),
                0.01,
                Isotropic::new(SolidColor::new(Color::zero())),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    )));
    objects.add(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(ConstantMedium::new(
                Box::new(box2),
                0.01,
                Isotropic::new(SolidColor::new(Color::one())),
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    )));

    objects
}

// 包含所有特性的复杂场景
pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(SolidColor::new(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    let mut rng = rand::thread_rng();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Box::new(RectBox::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground,
            )))
        }
    }
    let mut objects = HittableList {
        objects: vec![BVHNode::create(boxes1, 0.0, 1.0)],
    };

    let light = DiffuseLight::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Box::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center0 = Vec3::new(400.0, 400.0, 200.0);
    let center1 = center0 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(SolidColor::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Box::new(MovingSphere::new(
        center0,
        center1,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));
    objects.add(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    objects.add(Box::new(boundary));
    objects.add(Box::new(ConstantMedium::new(
        Box::new(boundary),
        0.2,
        Isotropic::new(SolidColor::new(Color::new(0.2, 0.4, 0.9))),
    )));
    objects.add(Box::new(ConstantMedium::new(
        Box::new(Sphere::new(Vec3::zero(), 5000.0, Dielectric::new(1.5))),
        0.0001,
        Isotropic::new(SolidColor::new(Color::one())),
    )));

    objects.add(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::new(ImageTexture::new(
            "raytracer/src/texture/img/earthmap.jpg".to_string(),
        )),
    )));
    objects.add(Box::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(NoiseTexture::new(0.1)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Box::new(Sphere::new(Vec3::random() * 165.0, 10.0, white)))
    }

    objects.add(Box::new(Translate::new(
        Box::new(RotateY::new(BVHNode::create(boxes2, 0.0, 1.0), 10.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}
