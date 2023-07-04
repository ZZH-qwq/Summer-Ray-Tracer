use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

mod vec3;
use vec3::{Color, Vec3};
mod ray;
use ray::Ray;

// 返回光线与球面交点的根
fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = Vec3::dot(ray.direction, ray.direction);
    let half_b = Vec3::dot(oc, ray.direction);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        // 返回较小 即离镜头更近的根
        (-half_b - discriminant.sqrt()) / a
    }
}

// 接受一个光线做为参数 然后计算这条光线所产生的颜色
fn ray_color(ray: Ray) -> Color {
    // 单位方向向量
    let unit_dir = Vec3::unit_vector(ray.direction);

    // 在本示例中 与球体相交的光线将会着色对应点单位外法向量映射至颜色空间的结果
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        // 单位外法向量即为球心指向交点的单位向量
        let n = Vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        // 将单位向量的坐标值映射为颜色
        return Color::new(0.5 + 0.5 * n.x, 0.5 + 0.5 * n.y, 0.5 + 0.5 * n.z);
    }

    // 不相交 则根据 y 值线性插值映射至白色 (1.0, 1.0, 1.0) 到蓝色 (0.5, 0.7, 1.0)
    let t = 0.5 + unit_dir.y * 0.5;
    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        // y方向进度
        let v: f64 = (j as f64) / ((height - 1) as f64);

        for i in 0..width {
            // x方向进度
            let u: f64 = (i as f64) / ((width - 1) as f64);
            let pixel = img.get_pixel_mut(i, height - 1 - j);

            // 生成光线
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let color = ray_color(ray);
            let rgb = color.to_u8();
            *pixel = image::Rgb([rgb.0, rgb.1, rgb.2]);
        }
        progress.inc(1);
    }
    progress.finish();

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
