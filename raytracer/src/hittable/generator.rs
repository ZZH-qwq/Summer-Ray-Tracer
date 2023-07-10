// 生成器

use crate::hittable::aarect::*;
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
