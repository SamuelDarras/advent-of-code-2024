use std::time::Instant;

use pest::Parser;
use template::InputParser;

fn main() {
    let src = include_str!("src2.txt");
    //     let src = "Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176

    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450

    // Button A: X+69, Y+23
    // Button B: X+27, Y+71
    // Prize: X=18641, Y=10279
    // ";

    let result = InputParser::parse(template::Rule::machines, src).unwrap();
    let mut systems: [([i64; 4], [i64; 2]); 320] = [([0; 4], [0; 2]); 320];

    result.enumerate().for_each(|(i, pair)| {
        let inner = pair.into_inner();
        let mut button_a_x = 0;
        let mut button_a_y = 0;
        let mut button_b_x = 0;
        let mut button_b_y = 0;
        let mut prize_x = 0;
        let mut prize_y = 0;
        for pair in inner {
            match pair.as_rule() {
                template::Rule::buttonA => {
                    let inner = pair.into_inner();
                    let inner = inner.collect::<Vec<_>>();
                    button_a_x = inner[0].as_span().as_str().parse::<i64>().unwrap();
                    button_a_y = inner[1].as_span().as_str().parse::<i64>().unwrap();
                }
                template::Rule::buttonB => {
                    let inner = pair.into_inner();
                    let inner = inner.collect::<Vec<_>>();
                    button_b_x = inner[0].as_span().as_str().parse::<i64>().unwrap();
                    button_b_y = inner[1].as_span().as_str().parse::<i64>().unwrap();
                }
                template::Rule::target => {
                    let inner = pair.into_inner();
                    let inner = inner.collect::<Vec<_>>();
                    prize_x = 10000000000000 + inner[0].as_span().as_str().parse::<i64>().unwrap();
                    prize_y = 10000000000000 + inner[1].as_span().as_str().parse::<i64>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        systems[i] = (
            [button_a_x, button_b_x, button_a_y, button_b_y],
            [prize_x, prize_y],
        );
    });

    let start = Instant::now();
    let mut price = 0;
    for system in &systems[..] {
        match solve_system(system) {
            Some(res) => price += res[0] * 3 + res[1],
            None => {}
        }
    }
    let duration = start.elapsed();

    println!("{price}, in {duration:?}");
}

fn solve_system(system: &([i64; 4], [i64; 2])) -> Option<[i64; 2]> {
    let a = system.0[0];
    let b = system.0[1];
    let c = system.0[2];
    let d = system.0[3];

    let s_1 = system.1[0];
    let s_2 = system.1[1];

    let det = a * d - b * c;

    if det == 0 {
        return None;
    }

    let row_1 = d * s_1 - b * s_2;
    let row_2 = a * s_2 - c * s_1;

    if row_1 % det != 0 || row_2 % det != 0 {
        return None;
    }

    let res = [row_1 / det, row_2 / det];

    Some(res)
}
