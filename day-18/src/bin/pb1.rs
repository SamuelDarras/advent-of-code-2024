use core::str;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::{BufWriter, Write},
    time::Instant,
    usize,
};

const MAP_WIDTH: usize = 71;
const MAP_HEIGHT: usize = 71;

fn main() {
    let src = include_str!("src1.txt");

    //     let src = "5,4
    // 4,2
    // 4,5
    // 3,0
    // 2,1
    // 6,3
    // 2,4
    // 1,5
    // 0,6
    // 3,3
    // 2,6
    // 5,1
    // 1,2
    // 5,5
    // 2,5
    // 6,5
    // 1,4
    // 0,4
    // 6,4
    // 1,1
    // 6,1
    // 1,0
    // 0,5
    // 1,6
    // 2,0";

    let now = Instant::now();
    let coords = src
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let path = solve((0, 0), (MAP_WIDTH - 1, MAP_HEIGHT - 1), &coords[0..1024]);

    let elapsed = now.elapsed();
    println!("{}, in {elapsed:?}", path.len() - 1);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Node {
    pos: (usize, usize),
    f_cost: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn h(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn solve(
    start: (usize, usize),
    goal: (usize, usize),
    map: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    // println!("{start:?} -> {goal:?}");

    let mut open_list = BinaryHeap::new();
    let mut g_cost: HashMap<(usize, usize), usize> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    g_cost.insert(start, 0);
    open_list.push(Node {
        pos: start,
        f_cost: h(start, goal),
    });

    let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];

    while let Some(current_node) = open_list.pop() {
        let current_position = current_node.pos;
        // if g_cost.len() % 40 == 0 {
        //     let _ = show_map(map, &g_cost.iter().map(|(k, _)| *k).collect::<HashSet<_>>());
        // }

        if current_position == goal {
            let mut path = vec![current_position];
            let mut current = current_position;

            while let Some(&previous) = came_from.get(&current) {
                path.push(previous);
                current = previous;
            }
            path.reverse();
            // let _ = show_map(map, &g_cost.iter().map(|(k, _)| *k).collect::<HashSet<_>>());
            return path;
        }

        for direction in &directions {
            let neighbour = (
                (current_position.0 as isize + direction.0) as usize,
                (current_position.1 as isize + direction.1) as usize,
            );

            if neighbour.0 >= MAP_WIDTH || neighbour.1 >= MAP_HEIGHT {
                continue;
            }
            if map.contains(&neighbour) {
                continue;
            }

            let tentative_g_cost = g_cost.get(&current_position).unwrap() + 1;

            if tentative_g_cost < *g_cost.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour, current_position);
                g_cost.insert(neighbour, tentative_g_cost);
                let f_cost = tentative_g_cost + h(neighbour, goal);
                open_list.push(Node {
                    pos: neighbour,
                    f_cost,
                });
            }
        }
    }
    Vec::new()
}

fn show_map(map: &[(usize, usize)], visited: &HashSet<(usize, usize)>) -> std::io::Result<()> {
    let mut buff = BufWriter::new(Vec::new());

    write!(buff, "\x1B[2J")?;
    write!(buff, "\x1B[0G")?;
    write!(buff, "\x1B[0d")?;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let c = if map.contains(&(x, y)) {
                "\x1B[48;2;255;255;255m\x1B[38;2;255;255;255m#\x1B[0m"
            } else if visited.contains(&(x, y)) {
                "\x1B[38;2;0;100;0mo\x1B[0m"
            } else {
                "\x1B[38;2;70;70;70m.\x1B[0m"
            };
            write!(buff, "{c}")?;
        }
        writeln!(buff, "")?;
    }

    println!(
        "{}",
        str::from_utf8(buff.into_inner().unwrap().as_slice()).unwrap()
    );
    // std::thread::sleep(Duration::from_millis(1));
    Ok(())
}
