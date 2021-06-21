use std::{thread::sleep, time::Duration};

use image::{open, GrayImage};
use rand::{thread_rng, Rng};

fn main() {
    // 1. ambil gambar sebagai luma
    // 2. kelompokkan terdekat
    // 3. hitung rata2
    // 4. ulangi dari 2
    let mut img: GrayImage = open("sora.jpg").unwrap().into_luma8();
    let group: Vec<u8> = Vec::new();

    let mut n1 = generate_random_index(256);
    let mut n2 = generate_random_index(256);

    let mut n1_gray_sum: i32 = 0;
    let mut n1_gray_i: i32 = 0;
    let mut n2_gray_sum: i32 = 0;
    let mut n2_gray_i: i32 = 0;

    let mut color1: u8 = 0;
    let mut color2: u8 = 255;

    let mut i = 0;
    loop {
        println!("loop start");
        for p in img.pixels_mut() {
            let group_n1 = (n1 - p.0[0] as i16).abs() < (n2 - p.0[0] as i16).abs();
            let group_n2 = (n2 - p.0[0] as i16).abs() < (n1 - p.0[0] as i16).abs();

            if group_n1 {
                n1_gray_sum += p.0[0] as i32;
                n1_gray_i += 1;
                p.0[0] = color1;
            } else if group_n2 {
                n2_gray_sum += p.0[0] as i32;
                n2_gray_i += 1;
                p.0[0] = color2;
            }
        }

        let mean1: u8 = (n1_gray_sum / n1_gray_i) as u8;
        let mean2: u8 = (n2_gray_sum / n2_gray_i) as u8;

        if (n1 == mean1 as i16) && (n2 == mean2 as i16) {
            println!(
                "sum: {} i: {} i32div: {}",
                n1_gray_sum,
                n1_gray_i,
                n1_gray_sum / n1_gray_i
            );
            println!(
                "sum2: {} i2: {} i32div2: {}",
                n2_gray_sum,
                n2_gray_i,
                n2_gray_sum / n2_gray_i
            );
            if i < 3 {
                i += 1;
            } else {
                break;
            }
        } else {
            n1 = mean1 as i16;
            n2 = mean2 as i16;

            n1_gray_sum = 0;
            n1_gray_i = 0;
            n2_gray_sum = 0;
            n2_gray_i = 0;

            img = open("sora.jpg").unwrap().into_luma8();
        }

        // println!("mean1: {} mean2: {}", mean1, mean2);
    }
    img.save("f2.png");
}

fn generate_random_index(max: i16) -> i16 {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}
