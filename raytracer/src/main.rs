mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use console::style;
use hittable::*;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::{fs::File, process::exit};
use vec3::{Color, Vec3};

// 接受一个光线做为参数 然后计算这条光线所产生的颜色
fn ray_color(ray: Ray, world: &dyn Hittable, depth: i32) -> Color {
    // 加入了漫反射材质
    // 限制递归层数
    if depth <= 0 {
        return Vec3::zero();
    }
    // 通过递归调用 ray_color 实现多次反射
    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        let target = hit_record.point + hit_record.normal + Vec3::random_in_unit_sphere();
        return ray_color(
            Ray::new(hit_record.point, target - hit_record.point),
            world,
            depth - 1,
        ) * 0.5;
    }
    // 不相交 则根据 y 值线性插值映射至白色 (1.0, 1.0, 1.0) 到蓝色 (0.5, 0.7, 1.0)
    let t = 0.5 + Vec3::unit_vector(ray.direction).y * 0.5;
    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // 图像
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // 生成
    let path = std::path::Path::new("output/book1/image7.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    // 物体
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // 镜头
    let cam = Camera::new();

    for j in 0..height {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, height - 1 - j);
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                // x,y方向分量 加入了多重采样抗锯齿
                let u_rand: f64 = rand::thread_rng().gen();
                let v_rand: f64 = rand::thread_rng().gen();
                let u: f64 = (i as f64 + u_rand) / ((width - 1) as f64);
                let v: f64 = (j as f64 + v_rand) / ((height - 1) as f64);

                // 生成光线
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world, max_depth);
            }
            let rgb = (pixel_color / samples_per_pixel as f64).to_u8();
            *pixel = image::Rgb([rgb.0, rgb.1, rgb.2]);
        }
        progress.inc(1);
    }
    progress.finish();
    world.clear();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
