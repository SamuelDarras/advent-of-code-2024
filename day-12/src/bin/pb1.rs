use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Write,
};

#[derive(Debug)]
struct Map {
    nodes: Vec<char>,
    coords_to_index: HashMap<(usize, usize), usize>,
    dim: (usize, usize),
    adjacency: HashMap<usize, HashSet<usize>>,
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
            src.lines().next().unwrap().trim().len(),
            src.lines().count(),
        );

        for (y, row) in src.lines().enumerate() {
            for (x, c) in row.trim().chars().enumerate() {
                s.nodes.push(c);
                s.coords_to_index.entry((x, y)).or_insert(s.nodes.len() - 1);
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

    fn get_node_at(&self, coords: (usize, usize)) -> Option<char> {
        Some(self.nodes[self.get_node_index_at(coords)?])
    }

    fn get_node_index_at(&self, coords: (usize, usize)) -> Option<usize> {
        Some(*self.coords_to_index.get(&coords)?)
    }

    fn get_adjacents_indices(&mut self, coords: (usize, usize)) -> Option<&HashSet<usize>> {
        let idx = self.get_node_index_at(coords)?;
        Some(self.adjacency.get(&idx)?)
    }

    fn get_adjacents_coords(&mut self, coords: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
        let idx = self.get_node_index_at(coords)?;
        let idx_set = self.adjacency.get(&idx)?;

        Some(
            idx_set
                .iter()
                .map(|idx| (idx % self.dim.0, idx / self.dim.0))
                .collect::<HashSet<_>>(),
        )
    }

    fn get_region_coords(&mut self, coords: (usize, usize)) -> HashSet<(usize, usize)> {
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
    let src = include_str!("src1.txt");
    // let src = "AAAA
    //            BBCD
    //            BBCC
    //            EEEC
    //         ";
    //     let src = "OOOOO
    // OXOXO
    // OOOOO
    // OXOXO
    // OOOOO
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

    let mut map = Map::build(src);

    // println!("{:?}", map.get_region_coords((0, 1)));

    let mut total_cost = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..map.dim.0 {
        for y in 0..map.dim.1 {
            if visited.contains(&(x, y)) {
                continue;
            }

            let region = map.get_region_coords((x, y));
            // println!(
            //     "({x:>4}, {y:>4}), visited {:>6} plots of {:>6}. Visiting {} plots. {}",
            //     visited.len(),
            //     map.nodes.len(),
            //     region.len(),
            //     map.get_node_at((x, y)).unwrap(),
            // );

            let mut perimeter = 0;
            for c in region.iter() {
                let neigbours_count = match map.get_adjacents_coords(*c) {
                    Some(s) => s.len(),
                    None => 0,
                };
                // println!("\t{c:?} has {neigbours_count} neighbours");
                perimeter += 4 - neigbours_count;
            }

            let region_cost = perimeter * region.len();
            // println!(
            //     "\tRegion {perimeter}(p) * {}(A) = {region_cost}",
            //     region.len()
            // );
            total_cost += region_cost;

            visited = visited.union(&region).map(|c| *c).collect::<HashSet<_>>();
        }
    }

    println!("{total_cost}");
}
