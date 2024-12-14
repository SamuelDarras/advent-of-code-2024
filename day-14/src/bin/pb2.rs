use std::{collections::HashMap, ops::AddAssign, time::Instant};

use rayon::iter::{ParallelBridge, ParallelIterator};

const MAP_WIDTH: i64 = 101;
const MAP_HEIGHT: i64 = 103;

fn main() {
    let src = include_str!("src2.txt");
    //     let src = "p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3
    // ";
    let robots = src.lines().map(|line| {
        let mut split = line.split_whitespace();
        let p = split.next().unwrap();
        let v = split.next().unwrap();

        let mut p = p[2..].split(",");
        let p_x = p.next().unwrap().parse::<i64>().unwrap();
        let p_y = p.next().unwrap().parse::<i64>().unwrap();

        let mut v = v[2..].split(",");
        let v_x = v.next().unwrap().parse::<i64>().unwrap();
        let v_y = v.next().unwrap().parse::<i64>().unwrap();

        ((p_x, p_y), (v_x, v_y))
    });

    let instant = Instant::now();
    let res = (0..10_000)
        .par_bridge()
        .map(|t| {
            let mut image_buffer = image::ImageBuffer::new(MAP_WIDTH as u32, MAP_HEIGHT as u32);
            let mut map: HashMap<(i64, i64), usize> = HashMap::new();
            let mut x_acc = 0;
            let mut y_acc = 0;
            robots.clone().for_each(|(p, v)| {
                let mut p_x = (p.0 + v.0 * t) % MAP_WIDTH;
                let mut p_y = (p.1 + v.1 * t) % MAP_HEIGHT;
                if p_x < 0 {
                    p_x += MAP_WIDTH;
                }
                if p_y < 0 {
                    p_y += MAP_HEIGHT;
                }
                x_acc += p_x;
                y_acc += p_y;
                map.entry((p_x, p_y)).or_default().add_assign(1);
                let pixel = image_buffer.get_pixel_mut(p_x as u32, p_y as u32);
                *pixel = image::Rgb([255u8, 0, 0]);
            });

            let cluster_center_x = x_acc / map.len() as i64;
            let cluster_center_y = y_acc / map.len() as i64;
            let clustering_score = map.keys().fold(0, |acc, &(x, y)| {
                acc + (x - cluster_center_x).pow(2) + (y - cluster_center_y).pow(2)
            }) / map.len() as i64;
            if clustering_score < 30 * 30 {
                image_buffer
                    .save(format!("res/image_{t}-{clustering_score:.3}.bmp"))
                    .ok()
            } else {
                None
            }
        })
        .filter(|v| v.is_some())
        .collect::<Vec<_>>();
    let duration = instant.elapsed();
    println!("created {} images, in {:?}", res.len(), duration);
}
