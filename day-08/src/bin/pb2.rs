use std::collections::{HashMap, HashSet};

fn main() {
    let src = include_str!("src1.txt");
//     let src = "............
//     ........0...
//     .....0......
//     .......0....
//     ....0.......
//     ......A.....
//     ............
//     ............
//     ........A...
//     .........A..
//     ............
//     ............
// ";
//     let src = "T.........
// ...T......
// .T........
// ..........
// ..........
// ..........
// ..........
// ..........
// ..........
// ..........
// ";

    let dim = (
        src.lines().next().unwrap().trim().len() as i32,
        src.lines().count() as i32,
    );
    println!("{dim:?}");

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    src.lines().map(str::trim).enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            antennas.entry(c).or_default().push((x as i32, y as i32));
        })
    });

    let mut antinodes = HashSet::new();
    antennas.iter().for_each(|(_, coords)| {
        if coords.len() == 1 {
            antinodes.insert(coords[0]);
        } else {
            for i in 0..coords.len() {
                for j in 0..coords.len() {
                    if i == j {
                        continue;
                    }
                    let a = coords[i];
                    let b = coords[j];

                    let ab = (a.0 - b.0, a.1 - b.1);
                    let mut t = 0;
                    let mut antinode = (a.0 + ab.0 * t, a.1 + ab.1 * t);
                    while antinode.0 >= 0
                        && antinode.0 < dim.0
                        && antinode.1 >= 0
                        && antinode.1 < dim.1
                    {
                        antinodes.insert(antinode);
                        t += 1;
                        antinode = (a.0 + ab.0 * t, a.1 + ab.1 * t);
                    }
                }
            }
        }
    });
    src.lines().map(str::trim).enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if antinodes.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!("{c}");
            }
        });
        println!();
    });
    println!("{}", antinodes.len());
}
