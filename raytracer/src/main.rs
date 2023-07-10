mod aabb;
mod bvh_node;
mod camera;
mod hittable;
mod material;
mod ray;
mod texture;
mod vec3;
use camera::Camera;
use console::style;
use hittable::*;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use ray::Ray;
use std::sync::Arc;
use std::thread;
use std::{fs::File, process::exit};
use vec3::{Color, Vec3};

use crate::bvh_node::BVHNode;

// 接受一个光线做为参数 然后计算这条光线所产生的颜色
fn ray_color(ray: Ray, background: &Color, world: &Arc<HittableList>, depth: i32) -> Color {
    // 限制递归层数
    if depth <= 0 {
        return Color::zero();
    }
    // 调用不同材质产生不同的反射
    if let Some(hit_record) = world.hit(ray, 0.0001, f64::INFINITY) {
        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, hit_record.point);
        if let Some((attenuation, scattered)) = hit_record.material.scatter(&ray, &hit_record) {
            return emitted + attenuation * ray_color(scattered, background, world, depth - 1);
        } else {
            return emitted;
        }
    }
    // 不相交 则返回背景颜色
    *background
}

fn main() {
    // 图像
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // 生成
    let path = std::path::Path::new("output/book2/image16.jpg");
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

    // 世界
    let world_type = 0;
    let lookfrom: Vec3;
    let lookat: Vec3;
    let vfov: f64;
    let aperture: f64;
    let background: Color;
    let world: HittableList;
    match world_type {
        1 => {
            world = HittableList {
                objects: vec![BVHNode::create(generator::random_scene(), 0.0, 1.0)],
            };
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = HittableList {
                objects: vec![BVHNode::create(generator::two_spheres(), 0.0, 1.0)],
            };
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        3 => {
            world = HittableList {
                objects: vec![BVHNode::create(generator::two_perlin_spheres(), 0.0, 1.0)],
            };
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        4 => {
            world = HittableList {
                objects: vec![BVHNode::create(generator::earth(), 0.0, 1.0)],
            };
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        _ => {
            world = HittableList {
                objects: vec![BVHNode::create(generator::simple_light(), 0.0, 1.0)],
            };
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
    };

    // 镜头
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        (lookfrom, lookat),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        (0.0, 1.0),
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
                        pixel_color += ray_color(ray, &background, &thread_world, cur_max_depth);
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
