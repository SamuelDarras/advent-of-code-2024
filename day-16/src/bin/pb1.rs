use std::collections::BinaryHeap;

fn main() {
    // let src = include_str!("src1.txt");

    let src = include!("src1.txt");

    //     let src = "###############
    // #.......#....E#
    // #.#.###.#.###.#
    // #.....#.#...#.#
    // #.###.#####.#.#
    // #.#.#.......#.#
    // #.#.#####.###.#
    // #...........#.#
    // ###.#.#####.#.#
    // #...#.....#.#.#
    // #.#.#.###.#.#.#
    // #.....#...#.#.#
    // #.###.#.#.#.#.#
    // #S..#.....#...#
    // ###############
    // ";
    //     let src = "#################
    // #...#...#...#..E#
    // #.#.#.#.#.#.#.#.#
    // #.#.#.#...#...#.#
    // #.#.#.#.###.#.#.#
    // #...#.#.#.....#.#
    // #.#.#.#.#.#####.#
    // #.#...#.#.#.....#
    // #.#.#####.#.###.#
    // #.#.#.......#...#
    // #.#.###.#####.###
    // #.#.#...#.....#.#
    // #.#.#.#####.###.#
    // #.#.#.........#.#
    // #.#.#.#########.#
    // #S#.............#
    // #################";

    let width = src.chars().position(|c| c == '\n').unwrap();
    let mut end = 0;
    let mut start = 0;
    let map: Vec<_> = src
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(n, c)| match c {
            '#' => true,
            '.' => false,
            'E' => {
                end = n as i16;
                false
            }
            'S' => {
                start = n as i16;
                false
            }
            _ => panic!("Invalid character in input: `{c}`@{n}"),
        })
        .collect();

    let mut visited = vec![[false; 4]; map.len()];
    let mut queue = BinaryHeap::new();
    queue.extend(
        (0..)
            .map(|steps| (std::cmp::Reverse(steps), start + steps as i16, 0i8))
            .take_while(|&(_, pos, _)| !map[pos as usize]),
    );

    let mut final_cost = 0;
    while let Some((std::cmp::Reverse(cost), pos, direction)) = queue.pop() {
        // println!("{pos} -> {end}");
        // println!("{queue:?}");
        if pos == end {
            final_cost = cost;
            break;
        }

        if visited[pos as usize][direction as usize] {
            continue;
        }

        visited[pos as usize][direction as usize] = true;

        queue.extend(
            [(direction - 1).rem_euclid(4), (direction + 1).rem_euclid(4)]
                .into_iter()
                .flat_map(|new_direction| {
                    let movement = match new_direction {
                        0 => 1,
                        1 => width as i16,
                        2 => -1,
                        3 => -(width as i16),
                        _ => panic!("Invalid direction"),
                    };

                    (1..)
                        .map(move |steps| {
                            (
                                std::cmp::Reverse(cost + 1000 + steps),
                                pos + movement * steps as i16,
                                new_direction,
                            )
                        })
                        .take_while(|&(_, new_pos, _)| !map[new_pos as usize])
                }),
        );
    }

    println!("{final_cost}");
}
