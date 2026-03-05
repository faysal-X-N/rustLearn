use std::path::Path;

use clap::{Parser, ValueEnum};
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use thiserror::Error;

/// 将图片转换为 ASCII 字符画的命令行工具
#[derive(Parser, Debug)]
#[command(name = "img2ascii")]
#[command(about = "将图片转换为 ASCII 字符画并输出到终端")]
struct Args {
    /// 输入图片：本地文件路径或 http/https 链接
    input: String,
    /// 输出字符宽度（默认 80）
    #[arg(short = 'w', long = "width", default_value_t = 80)]
    width: u32,
    /// 纵向压缩系数（字符瘦高；建议 0.43~0.6，默认 0.5）
    #[arg(short = 'a', long = "aspect", default_value_t = 0.5)]
    aspect: f32,
    /// 伽马校正（>1 提亮暗部，<1 压暗高光，默认 1.0）
    #[arg(short = 'g', long = "gamma", default_value_t = 1.0)]
    gamma: f32,
    /// 对比度系数（围绕 0.5 拉伸/收缩亮度，默认 1.0）
    #[arg(short = 'c', long = "contrast", default_value_t = 1.0)]
    contrast: f32,
    /// 字符集预设：simple|classic|dense
    #[arg(long = "ramp", default_value = "classic")]
    ramp: String,
    /// 是否使用抖动（Floyd–Steinberg）提升细节
    #[arg(long = "dither")]
    dither: bool,
    /// 字符集模式：ascii（默认）/ blocks（半块高分辨率）/ color（ANSI 彩色）
    #[arg(long = "charset", value_enum, default_value_t = Charset::Ascii)]
    charset: Charset,
    /// Braille 模式阈值（0.0~1.0，默认 0.5）
    #[arg(long = "threshold", default_value_t = 0.5)]
    threshold: f32,
    /// 反转明暗映射（黑底白字/白底黑字切换）
    #[arg(short = 'i', long = "invert")]
    invert: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum, Debug)]
enum Charset {
    Ascii,
    Blocks,
    Braille,
    Color,
}

#[derive(Error, Debug)]
enum AsciiError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("图片解码错误: {0}")]
    Img(#[from] image::ImageError),
    #[error("下载错误: {0}")]
    Net(#[from] reqwest::Error),
    #[error("宽度必须大于 0")]
    InvalidWidth,
}

fn main() -> Result<(), AsciiError> {
    let args = Args::parse();
    if args.width == 0 {
        return Err(AsciiError::InvalidWidth);
    }
    let out = match args.charset {
        Charset::Ascii => image_to_ascii(
            &args.input,
            args.width,
            args.aspect,
            args.gamma,
            args.contrast,
            &args.ramp,
            args.dither,
            args.invert,
        )?,
        Charset::Blocks => image_to_blocks(
            &args.input,
            args.width,
            args.aspect,
            args.gamma,
            args.contrast,
            args.invert,
        )?,
        Charset::Braille => image_to_braille(
            &args.input,
            args.width,
            args.aspect,
            args.gamma,
            args.contrast,
            args.threshold,
            args.invert,
        )?,
        Charset::Color => image_to_color(
            &args.input,
            args.width,
            args.aspect,
        )?,
    };
    print!("{out}");
    Ok(())
}

fn read_image(input: &str) -> Result<DynamicImage, AsciiError> {
    let is_url = input.starts_with("http://") || input.starts_with("https://");
    if is_url {
        let resp = reqwest::blocking::get(input)?;
        let bytes = resp.bytes()?;
        let img = image::load_from_memory(&bytes)?;
        Ok(img)
    } else {
        let path = Path::new(input);
        let img = image::ImageReader::open(path)?.decode()?;
        Ok(img)
    }
}

fn build_ramp(name: &str) -> Vec<char> {
    // 不同风格的字符密度表（由暗到亮）
    match name.to_lowercase().as_str() {
        "simple" => " .:-=+*#%@".chars().collect(),
        "dense" => "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "
            .chars()
            .collect(),
        _ /* classic */ => "@%#*+=-:. ".chars().collect(),
    }
}

fn image_to_ascii(
    input: &str,
    target_width: u32,
    aspect_fix: f32,
    gamma: f32,
    contrast: f32,
    ramp_name: &str,
    dither: bool,
    invert: bool,
) -> Result<String, AsciiError> {
    let img = read_image(input)?;
    let (w, h) = img.dimensions();

    // 计算目标尺寸
    // 终端字符通常是“宽:高 ≈ 1:2”（字符更“瘦高”），
    // 为了在终端中看起来比例协调，我们在计算高度时乘以一个压缩系数。
    // 常用经验值约为 0.5（也有人用 0.43），可用 --aspect 调节。
    let scale = target_width as f32 / w as f32;
    let mut target_height = ((h as f32) * scale * aspect_fix).round() as u32;
    if target_height == 0 {
        target_height = 1;
    }

    // 缩放到目标大小（使用 Lanczos3 提升缩放质量）
    let resized = img.resize_exact(target_width, target_height, FilterType::Lanczos3);

    // 转灰度
    // 灰度常用公式：Y = 0.299 R + 0.587 G + 0.114 B
    // image 的 to_luma8() 会基于标准权重得到单通道亮度
    let gray = resized.to_luma8();

    // 密度字符映射（从“暗”到“亮”）
    // 可以根据偏好调整字符序列以改变视觉风格
    let mut ramp: Vec<char> = build_ramp(ramp_name);
    if invert {
        ramp.reverse();
    }
    let n = ramp.len() as f32;

    // 亮度预处理（伽马 + 对比度）
    // 先归一化到 0..1，再做 gamma 与对比度变换
    let mut buf: Vec<f32> = gray
        .pixels()
        .map(|p| {
            let mut v = p.0[0] as f32 / 255.0;
            // 伽马：gamma>1 提亮暗部，<1 压暗高光
            if (gamma - 1.0).abs() > f32::EPSILON {
                v = v.powf(gamma.max(0.01));
            }
            // 对比度：围绕 0.5 拉伸/收缩
            if (contrast - 1.0).abs() > f32::EPSILON {
                v = (v - 0.5) * contrast + 0.5;
            }
            v.clamp(0.0, 1.0)
        })
        .collect();

    let width = target_width as usize;
    let height = target_height as usize;

    let mut out = String::with_capacity((width + 1) * height);

    if dither {
        // Floyd–Steinberg 抖动，将量化误差扩散到邻居像素
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                let v = buf[i];
                let idx_f = (v * (n - 1.0)).round();
                let idx = idx_f as usize;
                let q = idx_f / (n - 1.0); // 反映射回 0..1
                let err = v - q;
                out.push(ramp[idx]);

                // 误差扩散到周围像素（注意边界）
                //    x+1,y     : 7/16
                // x-1,y+1; x,y+1; x+1,y+1 : 3/16, 5/16, 1/16
                let distribute = |buf: &mut [f32], xi: isize, yi: isize, w: usize, h: usize, delta: f32| {
                    if xi >= 0 && yi >= 0 && (xi as usize) < w && (yi as usize) < h {
                        let idx = yi as usize * w + xi as usize;
                        buf[idx] = (buf[idx] + delta).clamp(0.0, 1.0);
                    }
                };
                distribute(&mut buf, (x as isize) + 1, y as isize, width, height, err * 7.0 / 16.0);
                distribute(&mut buf, (x as isize) - 1, (y as isize) + 1, width, height, err * 3.0 / 16.0);
                distribute(&mut buf, x as isize, (y as isize) + 1, width, height, err * 5.0 / 16.0);
                distribute(&mut buf, (x as isize) + 1, (y as isize) + 1, width, height, err * 1.0 / 16.0);
            }
            out.push('\n');
        }
    } else {
        // 无抖动：直接按像素映射
        for y in 0..height {
            for x in 0..width {
                let v = buf[y * width + x];
                let idx = (v * (n - 1.0)).round() as usize;
                out.push(ramp[idx]);
            }
            out.push('\n');
        }
    }
    Ok(out)
}

fn image_to_braille(
    input: &str,
    target_width: u32,
    aspect_fix: f32,
    gamma: f32,
    contrast: f32,
    threshold: f32,
    invert: bool,
) -> Result<String, AsciiError> {
    // 每个 Braille 字符代表 2x4 个像素点
    let img = read_image(input)?;
    let (w, h) = img.dimensions();
    let scale = target_width as f32 / w as f32;
    let mut target_height = ((h as f32) * scale * aspect_fix).round() as u32;
    if target_height == 0 {
        target_height = 1;
    }
    let out_cols = target_width;
    let out_rows = target_height;
    let res_w = out_cols * 2;
    let res_h = out_rows * 4;
    let resized = img.resize_exact(res_w, res_h, FilterType::Lanczos3).to_luma8();

    // 亮度预处理 + 归一化
    let mut buf: Vec<f32> = resized
        .pixels()
        .map(|p| {
            let mut v = p.0[0] as f32 / 255.0;
            if (gamma - 1.0).abs() > f32::EPSILON {
                v = v.powf(gamma.max(0.01));
            }
            if (contrast - 1.0).abs() > f32::EPSILON {
                v = (v - 0.5) * contrast + 0.5;
            }
            v.clamp(0.0, 1.0)
        })
        .collect();

    let width = res_w as usize;
    let height = res_h as usize;

    // 可选：对像素做简单抖动以改善二值化效果（FS 误差扩散）
    // 这里针对 0/1 阈值进行抖动，减少大块平面和锯齿
    let use_dither = true;
    if use_dither {
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                let old = buf[i];
                let new = if invert { old > threshold } else { old < threshold };
                let new_val = if new { 1.0 } else { 0.0 };
                let err = old - new_val;
                buf[i] = new_val;
                let distribute = |buf: &mut [f32], xi: isize, yi: isize, w: usize, h: usize, delta: f32| {
                    if xi >= 0 && yi >= 0 && (xi as usize) < w && (yi as usize) < h {
                        let idx = yi as usize * w + xi as usize;
                        buf[idx] = (buf[idx] + delta).clamp(0.0, 1.0);
                    }
                };
                distribute(&mut buf, (x as isize) + 1, y as isize, width, height, err * 7.0 / 16.0);
                distribute(&mut buf, (x as isize) - 1, (y as isize) + 1, width, height, err * 3.0 / 16.0);
                distribute(&mut buf, x as isize, (y as isize) + 1, width, height, err * 5.0 / 16.0);
                distribute(&mut buf, (x as isize) + 1, (y as isize) + 1, width, height, err * 1.0 / 16.0);
            }
        }
    }

    let mut out = String::with_capacity((out_cols as usize + 1) * out_rows as usize);
    for cy in 0..out_rows as usize {
        for cx in 0..out_cols as usize {
            // 2x4 cell -> Braille
            let px = cx * 2;
            let py = cy * 4;
            let mut bits = [false; 8];
            let sample = |x: usize, y: usize| -> f32 { buf[y * width + x] };
            // 映射到点阵顺序：1,2,3,7,4,5,6,8
            let v1 = sample(px + 0, py + 0);
            let v2 = sample(px + 0, py + 1);
            let v3 = sample(px + 0, py + 2);
            let v7 = sample(px + 0, py + 3);
            let v4 = sample(px + 1, py + 0);
            let v5 = sample(px + 1, py + 1);
            let v6 = sample(px + 1, py + 2);
            let v8 = sample(px + 1, py + 3);

            let on = |v: f32| -> bool { if invert { v > 0.5 } else { v < 0.5 } };
            bits[0] = on(v1); // dot1
            bits[1] = on(v2); // dot2
            bits[2] = on(v3); // dot3
            bits[6] = on(v7); // dot7
            bits[3] = on(v4); // dot4
            bits[4] = on(v5); // dot5
            bits[5] = on(v6); // dot6
            bits[7] = on(v8); // dot8

            let mut code: u8 = 0;
            for (i, b) in bits.iter().enumerate() {
                if *b {
                    code |= 1 << i;
                }
            }
            let ch = char::from_u32(0x2800 + code as u32).unwrap_or(' ');
            out.push(ch);
        }
        out.push('\n');
    }
    Ok(out)
}

fn image_to_blocks(
    input: &str,
    target_width: u32,
    aspect_fix: f32,
    gamma: f32,
    contrast: f32,
    invert: bool,
) -> Result<String, AsciiError> {
    let img = read_image(input)?;
    let (w, h) = img.dimensions();
    let scale = target_width as f32 / w as f32;
    // 为了在单字符行内表示上下两行像素，这里把内部高度翻倍，再两行合一
    let mut target_height = ((h as f32) * scale * aspect_fix * 2.0).round() as u32;
    if target_height == 0 {
        target_height = 2;
    }
    let resized = img.resize_exact(target_width, target_height, FilterType::Lanczos3);
    let gray = resized.to_luma8();

    // 灰度预处理
    let mut buf: Vec<f32> = gray
        .pixels()
        .map(|p| {
            let mut v = p.0[0] as f32 / 255.0;
            if (gamma - 1.0).abs() > f32::EPSILON {
                v = v.powf(gamma.max(0.01));
            }
            if (contrast - 1.0).abs() > f32::EPSILON {
                v = (v - 0.5) * contrast + 0.5;
            }
            v.clamp(0.0, 1.0)
        })
        .collect();

    let width = target_width as usize;
    let height = target_height as usize;
    let out_rows = height / 2;
    let mut out = String::with_capacity((width + 1) * out_rows);
    let thresh = 0.5f32;

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let top = buf[y * width + x];
            let bottom = if y + 1 < height { buf[(y + 1) * width + x] } else { top };
            let t_on = if invert { top > thresh } else { top < thresh };
            let b_on = if invert { bottom > thresh } else { bottom < thresh };
            let ch = match (t_on, b_on) {
                (false, false) => ' ',
                (true, false) => '▀',
                (false, true) => '▄',
                (true, true) => '█',
            };
            out.push(ch);
        }
        out.push('\n');
    }
    Ok(out)
}

fn image_to_color(
    input: &str,
    target_width: u32,
    aspect_fix: f32,
) -> Result<String, AsciiError> {
    let img = read_image(input)?;
    let (w, h) = img.dimensions();
    let scale = target_width as f32 / w as f32;
    let mut target_height = ((h as f32) * scale * aspect_fix * 2.0).round() as u32;
    if target_height == 0 {
        target_height = 2;
    }
    let resized = img.resize_exact(target_width, target_height, FilterType::Lanczos3).to_rgba8();
    let width = target_width as usize;
    let height = target_height as usize;
    let out_rows = height / 2;
    let mut out = String::with_capacity((width + 10) * out_rows);

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let p_top = resized.get_pixel(x as u32, y as u32).0;
            let p_bot = if y + 1 < height {
                resized.get_pixel(x as u32, (y + 1) as u32).0
            } else {
                p_top
            };
            // ANSI 24-bit: 前景 = 顶部颜色，背景 = 底部颜色，字符用 '▀'
            out.push_str(&format!(
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                p_top[0], p_top[1], p_top[2], p_bot[0], p_bot[1], p_bot[2]
            ));
        }
        out.push('\x1b');
        out.push('[');
        out.push_str("0m");
        out.push('\n');
    }
    Ok(out)
}
