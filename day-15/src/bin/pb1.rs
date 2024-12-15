use std::collections::HashMap;

use bitflags::bitflags;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let src = include_str!("src1.txt");
    //     let src = "##########
    // #..O..O.O#
    // #......O.#
    // #.OO..O.O#
    // #..O@..O.#
    // #O#..O...#
    // #O..O..O.#
    // #.OO.O.OO#
    // #....O...#
    // ##########\r\n
    // <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    // vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    // ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    // <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    // ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    // ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    // >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    // <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    // ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    // v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    // ";

    //     let src = "########
    // #..O.O.#
    // ##@.O..#
    // #...O..#
    // #.#.O..#
    // #...O..#
    // #......#
    // ########\r\n
    // <^^>>>vv<v>>v<<";

    let mut src = src.split("\r\n\r\n");
    let map_src = src.next().unwrap();
    let move_src = src.next().unwrap();

    let mut robot_pos = (0, 0);
    let mut map = HashMap::new();
    map_src.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map.insert(
                (x, y),
                match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '.' => Tile::Empty,
                    '@' => {
                        robot_pos = (x, y);
                        Tile::Robot
                    }
                    _ => unreachable!(),
                },
            );
        })
    });

    let moves = move_src
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    moves.iter().for_each(|direction| {
        let next_pos = find_next_pos(robot_pos, direction);
        let next_tile = map.get(&next_pos).unwrap();
        match next_tile {
            Tile::Wall => {}
            Tile::Box => {
                let mut positions = Vec::new();
                positions.push(next_pos);

                let mut box_position = next_pos;
                loop {
                    let tile = map.get(&box_position).unwrap();
                    if *tile == Tile::Empty {
                        for win in positions.windows(2) {
                            let new_tile = map.get(&win[0]).cloned().unwrap();
                            map.entry(win[1]).and_modify(|e| *e = new_tile);
                        }

                        map.entry(robot_pos).and_modify(|v| {
                            *v = Tile::Empty;
                        });
                        map.entry(next_pos).and_modify(|v| {
                            *v = Tile::Robot;
                        });
                        robot_pos = next_pos;
                        break;
                    } else if *tile == Tile::Wall {
                        break;
                    }
                    box_position = find_next_pos(box_position, direction);
                    positions.push(box_position);
                }
            }
            Tile::Empty => {
                map.entry(robot_pos).and_modify(|v| {
                    *v = Tile::Empty;
                });
                map.entry(next_pos).and_modify(|v| {
                    *v = Tile::Robot;
                });
                robot_pos = next_pos;
            }
            Tile::Robot => unreachable!(),
        }
    });

    let result = map
        .iter()
        .filter_map(|((x, y), t)| {
            if *t == Tile::Box {
                Some(y * 100 + x)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{result}");
}

fn find_next_pos(pos: (usize, usize), direction: &Direction) -> (usize, usize) {
    match *direction {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}
