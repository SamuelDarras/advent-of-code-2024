use core::str;
use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
    time::Duration,
};

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    BoxLeft,
    BoxRight,
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
    // ##########\r\n\r\n<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
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
    // ########\r\n\r\n<^^>>>vv<v>>v<<";

    //     let src = "#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######\r\n\r\n<vv<<^^<<^^";

//     let src = "######
// #....#
// #....#
// #..O.#
// #@OO.#
// #..O##
// #..O.#
// #....#
// ######\r\n\r\n>vvv>>>^<<^^^^^>>vv";

    let mut src = src.split("\r\n\r\n");
    let map_src = src.next().unwrap();
    let move_src = src.next().unwrap();

    let dim = (
        map_src.lines().next().unwrap().len() * 2,
        map_src.lines().count(),
    );

    let mut robot_pos = (0, 0);
    let mut map = HashMap::new();
    map_src.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let (left, right) = match c {
                '#' => (Tile::Wall, Tile::Wall),
                'O' => (Tile::BoxLeft, Tile::BoxRight),
                '.' => (Tile::Empty, Tile::Empty),
                '@' => {
                    robot_pos = (x * 2, y);
                    (Tile::Robot, Tile::Empty)
                }
                _ => unreachable!("`{c}`"),
            };
            map.insert((x * 2, y), left);
            map.insert((x * 2 + 1, y), right);
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

    // println!("{dim:?}");
    // println!("{map:?}");
    // println!("{moves:?}");

    print!("\x1B[2J");

    show_map(&map, dim.0, dim.1);

    moves.iter().for_each(|direction| {
        let next_pos = find_next_pos(robot_pos, direction);
        let next_tile = map.get(&next_pos).unwrap();
        match next_tile {
            Tile::Wall => {}
            Tile::BoxLeft | Tile::BoxRight => match direction {
                Direction::Left | Direction::Right => {
                    let mut positions = Vec::new();
                    positions.push(next_pos);

                    let mut box_position = next_pos;
                    loop {
                        let tile = map.get(&box_position).unwrap();
                        if *tile == Tile::Empty {
                            for win in positions.windows(2).rev() {
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
                Direction::Up | Direction::Down => {
                    let (ok, schedule) =
                        move_box(&mut map, find_next_pos(robot_pos, direction), direction);
                    if ok {
                        let mut schedule = schedule.iter().collect::<Vec<_>>();
                        schedule.sort_by_key(|v| {
                            if *direction == Direction::Up {
                                v.1 .1
                            } else {
                                dim.1 - v.1 .1
                            }
                        });
                        for (from, to) in schedule {
                            let mut old_tile = Tile::Empty;
                            let before_tile = map
                                .get(&find_next_pos(
                                    *from,
                                    match direction {
                                        Direction::Up => &Direction::Down,
                                        Direction::Down => &Direction::Up,
                                        Direction::Left => &Direction::Right,
                                        Direction::Right => &Direction::Left,
                                    },
                                ))
                                .cloned()
                                .unwrap();
                            map.entry(*from).and_modify(|e| {
                                old_tile = e.clone();
                                if before_tile == Tile::Empty
                                    || before_tile == Tile::Wall
                                    || (*e == Tile::BoxLeft && before_tile == Tile::BoxRight)
                                    || (*e == Tile::BoxRight && before_tile == Tile::BoxLeft)
                                {
                                    *e = Tile::Empty;
                                }
                            });
                            map.entry(*to).and_modify(|e| *e = old_tile);
                        }
                        map.entry(robot_pos).and_modify(|v| {
                            *v = Tile::Empty;
                        });
                        map.entry(next_pos).and_modify(|v| {
                            *v = Tile::Robot;
                        });
                        robot_pos = next_pos;
                    }
                }
            },
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
        show_map(&map, dim.0, dim.1);
        // println!();
    });

    let result = map
        .iter()
        .filter_map(|((x, y), t)| {
            if *t == Tile::BoxLeft {
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

fn show_map(
    map: &HashMap<(usize, usize), Tile>,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(Vec::new());
    // write!(buffer, "\x1B[2J")?;
    write!(buffer, "\x1B[0G")?;
    write!(buffer, "\x1B[0d")?;
    let mut total_box_count = 0;
    for y in 0..height {
        let mut box_count = 0;
        for x in 0..width {
            match map.get(&(x, y)) {
                Some(Tile::Wall) => write!(buffer, "\x1B[47m#\x1B[0m")?,
                Some(Tile::BoxLeft) => {
                    box_count += 1;
                    write!(buffer, "\x1B[33m[")?
                }
                Some(Tile::BoxRight) => write!(buffer, "]\x1B[0m")?,
                Some(Tile::Empty) => write!(buffer, "\x1B[38;2;40;40;40m.\x1B[0m")?,
                Some(Tile::Robot) => write!(buffer, "\x1B[45;34m@\x1B[0m")?,
                _ => {}
            }
        }

        total_box_count += box_count;
        writeln!(buffer, "  {box_count:>4}")?;
    }
    writeln!(buffer, "{total_box_count} boxes",)?;
    println!(
        "{}",
        str::from_utf8(buffer.into_inner().unwrap().as_slice()).unwrap()
    );
    std::thread::sleep(Duration::from_millis(10));
    Ok(())
}

fn move_box(
    map: &mut HashMap<(usize, usize), Tile>,
    pos: (usize, usize),
    direction: &Direction,
) -> (bool, HashSet<((usize, usize), (usize, usize))>) {
    let current_tile = map.get(&pos).unwrap().clone();
    match current_tile {
        Tile::Empty => (true, HashSet::new()),
        Tile::Wall => (false, HashSet::new()),
        Tile::BoxLeft => {
            let (can_move_left, scheduled_l) =
                move_box(map, find_next_pos(pos, direction), direction);
            let (can_move_right, scheduled_r) = move_box(
                map,
                find_next_pos(find_next_pos(pos, &Direction::Right), direction),
                direction,
            );
            (
                can_move_left && can_move_right,
                scheduled_l
                    .union(&scheduled_r)
                    .map(|v| *v)
                    .collect::<HashSet<_>>()
                    .union(&HashSet::from_iter(
                        vec![
                            (pos, find_next_pos(pos, direction)),
                            (
                                find_next_pos(pos, &Direction::Right),
                                find_next_pos(find_next_pos(pos, direction), &Direction::Right),
                            ),
                        ]
                        .iter()
                        .map(|v| *v),
                    ))
                    .map(|v| *v)
                    .collect::<HashSet<_>>(),
            )
        }
        Tile::BoxRight => {
            let (can_move_right, scheduled_r) =
                move_box(map, find_next_pos(pos, direction), direction);
            let (can_move_left, scheduled_l) = move_box(
                map,
                find_next_pos(find_next_pos(pos, &Direction::Left), direction),
                direction,
            );
            (
                can_move_left && can_move_right,
                scheduled_l
                    .union(&scheduled_r)
                    .map(|v| *v)
                    .collect::<HashSet<_>>()
                    .union(&HashSet::from_iter(
                        vec![
                            (pos, find_next_pos(pos, direction)),
                            (
                                find_next_pos(pos, &Direction::Left),
                                find_next_pos(find_next_pos(pos, direction), &Direction::Left),
                            ),
                        ]
                        .iter()
                        .map(|v| *v),
                    ))
                    .map(|v| *v)
                    .collect::<HashSet<_>>(),
            )
        }
        _ => (false, HashSet::new()),
    }
}
