use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Map {
    nodes: Vec<char>,
    coords_to_index: HashMap<(i64, i64), i64>,
    dim: (i64, i64),
    adjacency: HashMap<i64, HashSet<i64>>,
}

impl Map {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            coords_to_index: HashMap::new(),
            dim: (0, 0),
            adjacency: HashMap::new(),
        }
    }

    fn build(src: &str) -> Self {
        let mut s = Self::new();
        s.dim = (
            src.lines().next().unwrap().trim().len() as i64,
            (src.lines().count()) as i64,
        );

        for (y, row) in src.lines().enumerate() {
            for (x, c) in row.trim().chars().enumerate() {
                s.nodes.push(c);
                s.coords_to_index
                    .entry((x as i64, y as i64))
                    .or_insert((s.nodes.len() - 1) as i64);
            }
        }

        for x in 0..s.dim.0 {
            for y in 0..s.dim.1 {
                let node = s.get_node_at((x, y));
                let node_idx = s.get_node_index_at((x, y));
                match node {
                    Some(c) => {
                        if s.get_node_at((x.wrapping_sub(1), y)) == Some(c) {
                            let neighbour_index =
                                s.get_node_index_at((x.wrapping_sub(1), y)).unwrap();
                            s.adjacency
                                .entry(node_idx.unwrap())
                                .or_default()
                                .insert(neighbour_index);
                        }
                        if s.get_node_at((x + 1, y)) == Some(c) {
                            let neighbour_index = s.get_node_index_at((x + 1, y)).unwrap();
                            s.adjacency
                                .entry(node_idx.unwrap())
                                .or_default()
                                .insert(neighbour_index);
                        }
                        if s.get_node_at((x, y.wrapping_sub(1))) == Some(c) {
                            let neighbour_index =
                                s.get_node_index_at((x, y.wrapping_sub(1))).unwrap();
                            s.adjacency
                                .entry(node_idx.unwrap())
                                .or_default()
                                .insert(neighbour_index);
                        }
                        if s.get_node_at((x, y + 1)) == Some(c) {
                            let neighbour_index = s.get_node_index_at((x, y + 1)).unwrap();
                            s.adjacency
                                .entry(node_idx.unwrap())
                                .or_default()
                                .insert(neighbour_index);
                        }
                    }
                    None => {}
                }
            }
        }

        s
    }

    fn get_node_at(&self, coords: (i64, i64)) -> Option<char> {
        Some(self.nodes[self.get_node_index_at(coords)? as usize])
    }

    fn get_node_index_at(&self, coords: (i64, i64)) -> Option<i64> {
        Some(*self.coords_to_index.get(&coords)?)
    }

    fn get_adjacents_indices(&mut self, coords: (i64, i64)) -> Option<&HashSet<i64>> {
        let idx = self.get_node_index_at(coords)?;
        Some(self.adjacency.get(&idx)?)
    }

    fn get_adjacents_coords(&mut self, coords: (i64, i64)) -> Option<HashSet<(i64, i64)>> {
        let idx = self.get_node_index_at(coords)?;
        let idx_set = self.adjacency.get(&idx)?;

        Some(
            idx_set
                .iter()
                .map(|idx| (idx % self.dim.0, idx / self.dim.0))
                .collect::<HashSet<_>>(),
        )
    }

    fn get_region_coords(&mut self, coords: (i64, i64)) -> HashSet<(i64, i64)> {
        let mut visited = HashSet::new();
        visited.insert(coords);

        let mut queue = VecDeque::new();
        queue.push_back(coords);
        while let Some(c) = queue.pop_front() {
            visited.insert(c);
            match self.get_adjacents_coords(c) {
                Some(set) => set.iter().for_each(|c| {
                    if !visited.contains(c) && !queue.contains(c) {
                        queue.push_front(*c);
                    }
                }),
                None => {}
            }
            // print!("\r\t{:>6} {:>6}", queue.len(), visited.len());
            // std::io::stdout().flush();
        }

        visited
    }
}

fn main() {
    let src = include_str!("src2.txt");
    // let src = "AAAA
    //            BBCD
    //            BBCC
    //            EEEC";
//     let src = "AAAAAA
// AAABBA
// AAABBA
// ABBAAA
// ABBAAA
// AAAAAA
// ";
    //     let src = "RRRRIICCFF
    // RRRRIICCCF
    // VVRRRCCFFF
    // VVRCCCJFFF
    // VVVVCJJCFE
    // VVIVCCJJEE
    // VVIIICJJEE
    // MIIIIIJJEE
    // MIIISIJEEE
    // MMMISSJEEE
    // ";
    //     let src = "EEEEE
    // EXXXX
    // EEEEE
    // EXXXX
    // EEEEE
    // ";

    let mut map = Map::build(src);

    let mut total_cost = 0;

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    for x in 0..map.dim.0 {
        for y in 0..map.dim.1 {
            if visited.contains(&(x, y)) {
                continue;
            }

            let region = map.get_region_coords((x, y));
            println!(
                "({x:>4}, {y:>4}), visited {:>6} plots of {:>6}. Visiting {} plots.",
                visited.len(),
                map.nodes.len(),
                region.len(),
            );

            let mut region_corners = 0;
            for c in region.iter() {
                match map.get_adjacents_coords(*c) {
                    Some(s) => {
                        region_corners += count_corners(&region, &c);
                    }
                    None => {
                        region_corners += 4;
                    }
                }
            }

            let region_cost = region_corners * region.len();
            println!(
                "\tRegion {region_corners}(s) * {}(A) = {region_cost}",
                region.len()
            );
            total_cost += region_cost;

            visited = visited.union(&region).map(|c| *c).collect::<HashSet<_>>();
        }
    }

    println!("{total_cost}");
}

fn count_corners(region: &HashSet<(i64, i64)>, coords: &(i64, i64)) -> usize {
    let offsets = vec![
        vec![(0, -1), (-1, -1), (-1, 0)],
        vec![(-1, 0), (-1, 1), (0, 1)],
        vec![(0, 1), (1, 1), (1, 0)],
        vec![(1, 0), (1, -1), (0, -1)],
    ];

    let mut corners = 0;
    for offset in offsets {
        let top = (coords.0 + offset[0].0, coords.1 + offset[0].1);
        let top_left = (coords.0 + offset[1].0, coords.1 + offset[1].1);
        let left = (coords.0 + offset[2].0, coords.1 + offset[2].1);

        if region.contains(&top) && region.contains(&left) && !region.contains(&top_left) {
            corners += 1;
        } else if !region.contains(&top) && !region.contains(&left) && !region.contains(&top_left) {
            corners += 1;
        } else if !region.contains(&top) && !region.contains(&left) && region.contains(&top_left) {
            corners += 1;
        }
    }

    corners
}
