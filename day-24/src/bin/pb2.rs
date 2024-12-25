use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::Write,
    rc::Rc,
    usize,
};

type NodeId = usize;

#[derive(PartialEq, Clone, Debug)]
enum Gate<'a> {
    In,
    Out(&'a str),
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    id: NodeId,
    gate: Gate<'a>,
    inputs: HashSet<&'a str>,
    output: &'a str,
}

fn main() {
    // css,cwt,gdd,jmv,pqt,z05,z09,z37
    let src = include_str!("src2.txt");
    // let src = "x00: 1
    //     x01: 0
    //     y00: 0
    //     y01: 0
    //
    //     x00 OR y00 -> a
    //     x00 AND a -> z00
    //     x01 XOR y01 -> z01";

    // let src = "x00: 1
    //     x01: 0
    //     x02: 1
    //     x03: 1
    //     x04: 0
    //     y00: 1
    //     y01: 1
    //     y02: 1
    //     y03: 1
    //     y04: 1
    //
    //     ntg XOR fgs -> mjb
    //     y02 OR x01 -> tnw
    //     kwq OR kpj -> z05
    //     x00 OR x03 -> fst
    //     tgd XOR rvg -> z01
    //     vdt OR tnw -> bfw
    //     bfw AND frj -> z10
    //     ffh OR nrd -> bqk
    //     y00 AND y03 -> djm
    //     y03 OR y00 -> psh
    //     bqk OR frj -> z08
    //     tnw OR fst -> frj
    //     gnj AND tgd -> z11
    //     bfw XOR mjb -> z00
    //     x03 OR x00 -> vdt
    //     gnj AND wpb -> z02
    //     x04 AND y00 -> kjc
    //     djm OR pbm -> qhw
    //     nrd AND vdt -> hwm
    //     kjc AND fst -> rvg
    //     y04 OR y02 -> fgs
    //     y01 AND x02 -> pbm
    //     ntg OR kjc -> kwq
    //     psh XOR fgs -> tgd
    //     qhw XOR tgd -> z09
    //     pbm OR djm -> kpj
    //     x03 XOR y03 -> ffh
    //     x00 XOR y04 -> ntg
    //     bfw OR bqk -> z06
    //     nrd XOR fgs -> wpb
    //     frj XOR qhw -> z04
    //     bqk OR frj -> z07
    //     y03 OR x01 -> nrd
    //     hwm AND bqk -> z03
    //     tgd XOR rvg -> z12
    //     tnw OR pbm -> gnj";

    let mut names_mapping: HashMap<String, NodeId> = HashMap::new();

    let lines = src.lines();
    let inputs = lines
        .clone()
        .take_while(|line| line.trim().len() > 0)
        .map(|line| {
            let line = line.trim();
            let mut split = line.split(":");
            let name = split.next().unwrap();

            names_mapping.insert(name.to_string(), names_mapping.len());

            (
                name.to_string(),
                split.next().unwrap().trim().parse::<u8>().unwrap() != 0,
            )
        })
        .collect::<HashMap<_, _>>();

    // let mut gates = Vec::new();

    let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut adjacency_reverse: HashMap<usize, HashSet<usize>> = HashMap::new();

    // println!("{names_mapping:?}");

    // let swap_map = HashMap::from([("XOR130", "OR122"), ("OR122", "XOR130")]);
    // let swap_map: HashMap<&str, &str> = HashMap::new();

    let mut id = names_mapping.len() - 1;
    lines.skip(inputs.len() + 1).for_each(|line| {
        let rule = line.trim();
        let mut split = rule.split("->");
        let gate = split.next().unwrap().trim();
        let output = split.next().unwrap().trim();

        let mut split = gate.split_whitespace();
        let left = split.next().unwrap();
        let gate_name = split.next().unwrap();
        let right = split.next().unwrap();

        let left_id = if let Some(id) = names_mapping.get(&left.to_string()) {
            *id
        } else {
            id += 1;
            names_mapping.insert(left.to_string(), id);
            id
        };
        let right_id = if let Some(id) = names_mapping.get(&right.to_string()) {
            *id
        } else {
            id += 1;
            names_mapping.insert(right.to_string(), id);
            id
        };
        let output_id = if let Some(id) = names_mapping.get(&output.to_string()) {
            *id
        } else {
            id += 1;
            names_mapping.insert(output.to_string(), id);
            id
        };
        id += 1;
        let gate_id = id;
        names_mapping.insert(format!("{}{}", gate_name, id), id);

        // println!("{left}({left_id}) -- {right}({right_id}) -> {output}({output_id})");

        let set = adjacency.entry(left_id).or_default();
        set.insert(gate_id);

        let set = adjacency.entry(right_id).or_default();
        set.insert(gate_id);

        let set = adjacency.entry(gate_id).or_default();
        set.insert(output_id);

        let set = adjacency_reverse.entry(gate_id).or_default();
        set.insert(left_id);

        let set = adjacency_reverse.entry(gate_id).or_default();
        set.insert(right_id);

        let set = adjacency_reverse.entry(output_id).or_default();
        set.insert(gate_id);
    });

    // println!("{names_mapping:?}");
    // println!("{nodes:?}");

    // println!("{adjacency:?}");
    // println!("{adjacency_reverse:?}");

    let mut file = std::fs::File::create("input.dot").unwrap();
    let _ = write!(file, "{}", dot(&names_mapping, &adjacency));

    // nodes.swap(
    //     *names_mapping.get(&"wgh".to_string()).unwrap(),
    //     *names_mapping.get(&"sgj".to_string()).unwrap(),
    // );
    // nodes.swap(
    //     *names_mapping.get(&"z09".to_string()).unwrap(),
    //     *names_mapping.get(&"cwt".to_string()).unwrap(),
    // );

    // println!();
    // println!("{:?}", nodes.iter().enumerate().collect::<Vec<_>>());

    let mut nodes = vec![String::new(); names_mapping.len()];
    for (k, v) in names_mapping.iter() {
        nodes[*v] = k.to_string();
    }

    for i in (1u128 << 37)..(1u128 << 63) {
        let x = i;
        let y = 0b1u128;

        let mut inputs = HashMap::new();
        for i in 0..64 {
            inputs.insert(format!("x{i:02}"), x & (1u128 << i) != 0);
            inputs.insert(format!("y{i:02}"), y & (1u128 << i) != 0);
        }

        let sort = topological_sort(names_mapping.len(), &adjacency);
        let result = run_network(&sort, &nodes, &adjacency_reverse, &inputs);

        let mut result = result.iter().collect::<Vec<_>>();
        result.sort_by_key(|v| v.0[1..].parse::<usize>().unwrap());
        let result = result
            .iter()
            .map(|v| v.1)
            .enumerate()
            .fold(0u128, |acc, (i, x)| acc + ((if *x { 1 } else { 0 }) << i));

        println!("0b{x:064b} (0x{x:016x}) ({x:08})");
        println!("0b{y:064b} (0x{y:016x}) ({y:08})");
        println!("+ ----------------------------------------------------------------");
        println!("0b{result:064b} (0x{result:016x}) ({result:08})");
        println!();
        if result != x + y {
            break;
        }
    }
}

fn dot<'a>(
    names_mapping: &HashMap<String, usize>,
    adjacency: &HashMap<NodeId, HashSet<usize>>,
) -> String {
    let mut s = "digraph logic {\n".to_string();
    for (name, id) in names_mapping.iter() {
        s.push_str(format!("\t{id} [label=\"{}({})\"];\n", name, id).as_str());
    }
    for (k, v) in adjacency.iter() {
        // let name = names_mapping
        //     .iter()
        //     .find(|(_, v)| **v == *k)
        //     .map(|v| v.0)
        //     .unwrap();

        for k_p in v.iter() {
            s.push_str(format!("\t{k} -> {k_p};\n").as_str());
        }
    }
    s.push_str("}");
    s
}

fn run_network<'a>(
    sort: &Vec<NodeId>,
    nodes: &'a Vec<String>,
    adjacency_reverse: &HashMap<NodeId, HashSet<usize>>,
    inputs: &HashMap<String, bool>,
) -> HashMap<&'a String, bool> {
    let mut computed = HashMap::new();
    let mut outputs = HashMap::new();

    for &node in sort.iter() {
        if nodes[node].starts_with("AND") {
            let result = adjacency_reverse
                .get(&node)
                .unwrap()
                .iter()
                .all(|v| *computed.get(v).unwrap());
            computed.insert(node, result);
        } else if nodes[node].starts_with("OR") {
            let result = adjacency_reverse
                .get(&node)
                .unwrap()
                .iter()
                .any(|v| *computed.get(v).unwrap());
            computed.insert(node, result);
        } else if nodes[node].starts_with("XOR") {
            let result = adjacency_reverse
                .get(&node)
                .unwrap()
                .iter()
                .fold(false, |acc, v| acc != *computed.get(v).unwrap());
            computed.insert(node, result);
        } else if nodes[node].starts_with("z") {
            for parent in adjacency_reverse.get(&node).unwrap().iter() {
                let result = *computed.get(&parent).unwrap();
                computed.insert(node, result);
                outputs.insert(&nodes[node], result);
            }
        } else {
            if inputs.contains_key(&nodes[node]) {
                computed.insert(node, *inputs.get(&nodes[node]).unwrap());
            } else {
                computed.insert(
                    node,
                    *computed
                        .get(adjacency_reverse.get(&node).unwrap().iter().next().unwrap())
                        .unwrap(),
                );
            }
        }
    }
    outputs
}

fn topological_sort(
    nodes_count: usize,
    adjacency: &HashMap<NodeId, HashSet<NodeId>>,
) -> Vec<usize> {
    let stack = Rc::new(RefCell::new(Vec::new()));
    let mut visited = HashSet::new();
    for i in 0..nodes_count {
        if !visited.contains(&i) {
            find_topological_sort(i, &mut visited, Rc::clone(&stack), adjacency);
        }
    }
    let mut sort = Rc::into_inner(stack).unwrap().into_inner();
    sort.reverse();
    sort
}

fn find_topological_sort<'a>(
    node: NodeId,
    mut visited: &mut HashSet<NodeId>,
    stack: Rc<RefCell<Vec<NodeId>>>,
    adjacency: &HashMap<NodeId, HashSet<NodeId>>,
) {
    visited.insert(node);

    for neighbour in adjacency.get(&node).map_or(&HashSet::new(), |v| v).iter() {
        if !visited.contains(neighbour) {
            find_topological_sort(*neighbour, &mut visited, Rc::clone(&stack), adjacency);
        }
    }
    stack.borrow_mut().push(node);
}
