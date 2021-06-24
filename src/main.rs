use std::{thread::sleep, time::Duration};

use image::{open, GrayImage};
use rand::{thread_rng, Rng};

fn main() {
    // 1. perkenalan
    // 2. tools
    // 3. + -
    // 4. algoritma
    // 5. run
    // 6. penutup
    let mut img = open("sora.jpg").unwrap().into_luma8();

    let mut k1 = generate_random_index(256);
    let mut k2 = generate_random_index(256);

    let mut k1_gray_sum: i32 = 0;
    let mut k1_gray_i: i32 = 0;
    let mut k2_gray_sum: i32 = 0;
    let mut k2_gray_i: i32 = 0;

    let mut color1: u8 = 0;
    let mut color2: u8 = 255;

    let mut i = 0;
    loop {
        println!("loop start");
        for p in img.pixels_mut() {
            let group_k1 = (k1 - p.0[0] as i16).abs() < (k2 - p.0[0] as i16).abs();
            let group_k2 = (k2 - p.0[0] as i16).abs() < (k1 - p.0[0] as i16).abs();

            if group_k1 {
                k1_gray_sum += p.0[0] as i32;
                k1_gray_i += 1;
                p.0[0] = color1;
            } else if group_k2 {
                k2_gray_sum += p.0[0] as i32;
                k2_gray_i += 1;
                p.0[0] = color2;
            }
        }

        let mean1 = (k1_gray_sum / k1_gray_i) as u8;
        let mean2 = (k2_gray_sum / k2_gray_i) as u8;

        if (k1 == mean1 as i16) && (k2 == mean2 as i16) {
            println!(
                "sum: {} i: {} i32div: {}",
                k1_gray_sum,
                k1_gray_i,
                k1_gray_sum / k1_gray_i
            );
            println!(
                "sum2: {} i2: {} i32div2: {}",
                k2_gray_sum,
                k2_gray_i,
                k2_gray_sum / k2_gray_i
            );
            if i < 3 {
                i += 1;
            } else {
                break;
            }
        } else {
            k1 = mean1 as i16;
            k2 = mean2 as i16;

            k1_gray_sum = 0;
            k1_gray_i = 0;
            k2_gray_sum = 0;
            k2_gray_i = 0;

            img = open("sora.jpg").unwrap().into_luma8();
        }

        // println!("mean1: {} mean2: {}", mean1, mean2);
    }
    img.save("f2.png").unwrap();
}

fn generate_random_index(max: i16) -> i16 {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}
