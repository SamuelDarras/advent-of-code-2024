use std::collections::BinaryHeap;

fn main() {
    let src = include!("src2.txt");

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
            _ => panic!("Invalid character in input"),
        })
        .collect();

    let mut visited = vec![vec![None::<(u32, Vec<(i16, i8)>)>; 4]; map.len()];
    let mut queue = BinaryHeap::new();
    queue.push((std::cmp::Reverse(0), start, 0i8, None));
    queue.extend(
        (1..)
            .map(|steps| {
                (
                    std::cmp::Reverse(steps),
                    start + steps as i16,
                    0i8,
                    Some((start + steps as i16 - 1, 0i8)),
                )
            })
            .take_while(|&(_, pos, _, _)| !map[pos as usize]),
    );

    let mut end_cost = u32::MAX;
    let mut end_positions = Vec::new();

    while let Some((std::cmp::Reverse(cost), pos, direction, came_from)) = queue.pop() {
        if cost > end_cost {
            break;
        }

        match &mut visited[pos as usize][direction as usize] {
            Some((visited_cost, visited_came_from)) => {
                if cost == *visited_cost {
                    if let Some(came_from) = came_from {
                        if !visited_came_from.contains(&came_from) {
                            visited_came_from.push(came_from);
                        }
                    }
                }
                continue;
            }
            x @ None => {
                *x = Some((
                    cost,
                    if let Some(came_from) = came_from {
                        vec![came_from]
                    } else {
                        Vec::new()
                    },
                ));
            }
        }

        if pos == end {
            end_cost = cost;
            end_positions.push((pos, direction));
            continue;
        }

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
                                Some((
                                    pos + movement * (steps as i16 - 1),
                                    if steps == 1 { direction } else { new_direction },
                                )),
                            )
                        })
                        .take_while(|&(_, new_pos, _, _)| !map[new_pos as usize])
                }),
        );
    }

    let mut count = 0;
    let mut checked = vec![[false; 4]; map.len()];
    let mut counted = vec![false; map.len()];
    let mut to_check = end_positions;

    while let Some((pos, direction)) = to_check.pop() {
        if checked[pos as usize][direction as usize] {
            continue;
        }

        checked[pos as usize][direction as usize] = true;

        if !counted[pos as usize] {
            counted[pos as usize] = true;
            count += 1;
        }

        to_check.extend(
            visited[pos as usize][direction as usize]
                .as_ref()
                .unwrap()
                .1
                .iter()
                .copied(),
        );
    }

    println!("{count}");
}
