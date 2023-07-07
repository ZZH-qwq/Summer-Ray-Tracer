mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use console::style;
use hittable::*;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use material::*;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::sync::Arc;
use std::thread;
use std::{fs::File, process::exit};
use vec3::{Color, Vec3};

// 接受一个光线做为参数 然后计算这条光线所产生的颜色
fn ray_color(ray: Ray, world: &Arc<HittableList>, depth: i32) -> Color {
    // 加入了漫反射材质
    // 限制递归层数
    if depth <= 0 {
        return Color::zero();
    }
    // 调用不同材质产生不同的反射
    if let Some(hit_record) = world.hit(ray, 0.0001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(&ray, &hit_record) {
            return attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return Color::zero();
        }
    }
    // 不相交 则根据 y 值线性插值映射至白色 (1.0, 1.0, 1.0) 到蓝色 (0.5, 0.7, 1.0)
    let t = 0.5 + Vec3::unit_vector(ray.direction).y * 0.5;
    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
}

// 生成随机场景
fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
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
                    let albedo = Color::random() * Color::random();
                    world.add(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
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
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));

    world
}

fn main() {
    // 图像
    let aspect_ratio = 3.0 / 2.0;
    let width = 600;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    // 生成
    let path = std::path::Path::new("output/book1/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(width as u64)
    };
    progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/bue}] [{pos}/{len}] ({eta})",
            )
            .progress_chars("#>-"),
    );

    // 物体
    let world = random_scene();

    // 镜头
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // 线程
    let thread_count = 6;
    let mul_cam = Arc::new(cam);
    let mul_world = Arc::new(world);
    let mul_progress = Arc::new(progress);
    let mut handles = vec![];

    for i in 0..thread_count {
        let mut curr = i;
        let step = thread_count;
        let tot_width = width;
        let tot_height = height;
        let cur_samples_per_pixel = samples_per_pixel;
        let cur_max_depth = max_depth;
        let thread_cam = Arc::clone(&mul_cam);
        let thread_world = Arc::clone(&mul_world);
        let cur_progress = Arc::clone(&mul_progress);

        // 生成新线程
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut result = vec![];
            while curr < tot_width {
                let mut col = vec![];
                for j in 0..tot_height {
                    let mut pixel_color = Vec3::zero();
                    for _ in 0..cur_samples_per_pixel {
                        // x,y方向分量 加入了多重采样抗锯齿
                        let u_rand: f64 = rng.gen();
                        let v_rand: f64 = rng.gen();
                        let u: f64 = (curr as f64 + u_rand) / ((tot_width - 1) as f64);
                        let v: f64 = (j as f64 + v_rand) / ((tot_height - 1) as f64);

                        // 生成光线
                        let ray = thread_cam.get_ray(u, v);
                        pixel_color += ray_color(ray, &thread_world, cur_max_depth);
                    }
                    let rgb = (pixel_color / cur_samples_per_pixel as f64).to_u8();
                    col.push(rgb);
                }
                cur_progress.inc(1);
                result.push((curr, col));
                curr += step;
            }
            // 返回结果
            result
        });
        handles.push(handle);
    }

    // 接收结果并汇总
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    mul_progress.finish();
    for result in &results {
        for (col_name, col_rgb) in result {
            for (i, pix) in col_rgb.iter().enumerate() {
                let pixel = img.get_pixel_mut(*col_name, height - 1 - i as u32);
                *pixel = image::Rgb([pix.0, pix.1, pix.2]);
            }
        }
    }

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
