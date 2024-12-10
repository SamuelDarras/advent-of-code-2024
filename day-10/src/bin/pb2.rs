use std::{
    collections::{HashSet, VecDeque},
    io::Write,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let src = include_str!("src2.txt");
    //     let src = "0123
    // 1234
    // 8765
    // 9876
    // ";
//     let src = "89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732
// ";

    let map = src
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u8 - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut tracks = Vec::new();
    for y in 0..map.len() {
        let mut row = Vec::new();
        for x in 0..map[y].len() {
            let current_hight = map[y][x];
            let mut info = 0;
            if y >= 1 && current_hight + 1 == map[y - 1][x] {
                info |= 0b0001;
            }
            if x < map[y].len() - 1 && current_hight + 1 == map[y][x + 1] {
                info |= 0b0010;
            }
            if y < map.len() - 1 && current_hight + 1 == map[y + 1][x] {
                info |= 0b0100;
            }
            if x >= 1 && current_hight + 1 == map[y][x - 1] {
                info |= 0b1000;
            }
            row.push(info);
        }
        tracks.push(row);
    }

    show_map(&map);
    println!();
    show_map(&tracks);

    let mut score = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut queue = VecDeque::new();
                queue.push_back((x, y));

                let mut visited = HashSet::new();
                let mut local_score = 0;
                while !queue.is_empty() {
                    let (x, y) = queue.pop_front().unwrap();
                    visited.insert((x, y));

                    if map[y][x] == 9 {
                        local_score += 1;
                    }

                    let track_info = tracks[y][x];
                    if track_info & 0b0001 != 0 {
                        let next = (x, y - 1);
                        queue.push_front(next);
                    }
                    if track_info & 0b0010 != 0 {
                        let next = (x + 1, y);
                        queue.push_front(next);
                    }
                    if track_info & 0b0100 != 0 {
                        let next = (x, y + 1);
                        queue.push_front(next);
                    }
                    if track_info & 0b1000 != 0 {
                        let next = (x - 1, y);
                        queue.push_front(next);
                    }

                    // println!("({x}, {y}) ; {queue:?} ; {visited:?}");
                }
                // println!("{local_score}");
                score += local_score;
            }
        }
    }
    println!("{score}");
}

fn show_map(map: &Vec<Vec<u8>>) {
    let mut buffer = StandardStream::stdout(ColorChoice::Always);

    let colors = [
        Color::Ansi256(0),
        Color::Ansi256(8),
        Color::Ansi256(5),
        Color::Ansi256(4),
        Color::Ansi256(6),
        Color::Ansi256(2),
        Color::Ansi256(7),
        Color::Ansi256(3),
        Color::Ansi256(1),
        Color::Ansi256(9),
    ];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            buffer
                .set_color(ColorSpec::new().set_bg(Some(
                    *colors.get(map[y][x] as usize).unwrap_or(&Color::Black),
                )))
                .unwrap();
            let _ = write!(buffer, "{:x}", map[y][x]);
            buffer.reset().unwrap();
        }
        let _ = writeln!(buffer);
    }

    buffer.reset().unwrap();
}
