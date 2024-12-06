use std::time::Duration;

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    None,
    Obstacle,
    Visited(Direction),
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let src = include_str!("src1.txt");
    // let src = " ....#.....
    //             .........#
    //             ..........
    //             ..#.......
    //             .......#..
    //             ..........
    //             .#..^.....
    //             ........#.
    //             #.........
    //             ......#...";

    // 9
    // let src = " .#...
    //             ....#
    //             .....
    //             #....
    //             .^.#.";

    let mut start = None;
    let map = src
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::None,
                    '#' => Tile::Obstacle,
                    '^' => {
                        start = Some((x, y));
                        Tile::None
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut map = map
        .iter()
        .map(|line| {
            let mut line = line.clone();
            line.insert(0, Tile::Obstacle);
            line.push(Tile::Obstacle);
            line
        })
        .collect::<Vec<_>>();
    map.insert(0, vec![Tile::Obstacle; map[0].len()]);
    map.push(vec![Tile::Obstacle; map[0].len()]);

    if let None = start {
        panic!("No guard found");
    }

    let mut direction = Direction::North;
    let mut pos = start.unwrap();
    pos.0 += 1;
    pos.1 += 1;
    loop {
        let next_obstacle_pos = find_obstacle(&direction, &pos, &map);
        // println!("{pos:?}{next_obstacle_pos:?} {direction:?}");
        let (new_direction, new_pos) = match direction {
            Direction::North => {
                let direction = Direction::East;
                (direction, (pos.0, next_obstacle_pos.1 + 1))
            }
            Direction::South => {
                let direction = Direction::West;
                (direction, (pos.0, next_obstacle_pos.1 - 1))
            }
            Direction::West => {
                let direction = Direction::North;
                (direction, (next_obstacle_pos.0 + 1, pos.1))
            }
            Direction::East => {
                let direction = Direction::South;
                (direction, (next_obstacle_pos.0 - 1, pos.1))
            }
        };
        let mut end = false;
        if pos == new_pos
            || map[new_pos.1][new_pos.0] == Tile::Visited(new_direction.clone())
            || new_pos.0 == 1
            || new_pos.0 == map.len() - 2
            || new_pos.1 == 1
            || new_pos.1 == map[0].len() - 2
        {
            end = true;
        }
        for x in pos.0.min(new_pos.0)..=pos.0.max(new_pos.0) {
            for y in pos.1.min(new_pos.1)..=pos.1.max(new_pos.1) {
                map[y][x] = Tile::Visited(direction.clone());
            }
        }
        // map.iter().for_each(|row| {
        //     row.iter().for_each(|t| match t {
        //         Tile::None => print!("."),
        //         Tile::Obstacle => print!("#"),
        //         Tile::Visited(direction) => match direction {
        //             Direction::North => print!("^"),
        //             Direction::South => print!("v"),
        //             Direction::West => print!("<"),
        //             Direction::East => print!(">"),
        //         },
        //     });
        //     println!();
        // });
        // std::thread::sleep(Duration::from_millis(100));
        if end {
            break;
        }
        pos = new_pos;
        direction = new_direction;
    }

    let result = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| match tile {
                    Tile::None => 0,
                    Tile::Obstacle => 0,
                    Tile::Visited(_) => 1,
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{result}");
}

fn find_obstacle(
    direction: &Direction,
    pos: &(usize, usize),
    map: &Vec<Vec<Tile>>,
) -> (usize, usize) {
    match direction {
        Direction::North => {
            let mut cur_tile = map[pos.1][pos.0].clone();
            let mut pos = *pos;
            while cur_tile != Tile::Obstacle {
                pos.1 -= 1;
                cur_tile = map[pos.1][pos.0].clone();
            }
            pos
        }
        Direction::South => {
            let mut cur_tile = map[pos.1][pos.0].clone();
            let mut pos = *pos;
            while cur_tile != Tile::Obstacle {
                pos.1 += 1;
                cur_tile = map[pos.1][pos.0].clone();
            }
            pos
        }
        Direction::West => {
            let mut cur_tile = map[pos.1][pos.0].clone();
            let mut pos = *pos;
            while cur_tile != Tile::Obstacle {
                pos.0 -= 1;
                cur_tile = map[pos.1][pos.0].clone();
            }
            pos
        }
        Direction::East => {
            let mut cur_tile = map[pos.1][pos.0].clone();
            let mut pos = *pos;
            while cur_tile != Tile::Obstacle {
                pos.0 += 1;
                cur_tile = map[pos.1][pos.0].clone();
            }
            pos
        }
    }
}
