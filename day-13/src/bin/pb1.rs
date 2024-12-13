use pest::Parser;
use template::InputParser;

fn main() {
    let src = include_str!("src1.txt");
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

    let mut systems = Vec::new();
    for pair in result {
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
                    prize_x = inner[0].as_span().as_str().parse::<i64>().unwrap();
                    prize_y = inner[1].as_span().as_str().parse::<i64>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        systems.push((
            vec![vec![button_a_x, button_b_x], vec![button_a_y, button_b_y]],
            vec![prize_x, prize_y],
        ));
    }

    let mut price = 0;
    for system in systems {
        println!("{system:?}");
        let solve = solve_system(&system);
        println!("{solve:?}");
        match solve {
            Some(x) => {
                if x[0] <= 100.0
                    && x[1] <= 100.0
                    && (x[0].round() - x[0]).abs() < 0.000001
                    && (x[1].round() - x[1]).abs() < 0.000001
                {
                    price += x[0].round() as i64 * 3 + x[1].round() as i64
                }
            }
            None => todo!(),
        }
    }
    println!("{price}");
}

fn solve_system(system: &(Vec<Vec<i64>>, Vec<i64>)) -> Option<Vec<f64>> {
    let det = system.0[0][0] * system.0[1][1] - system.0[0][1] * system.0[1][0];

    if det == 0 {
        return None;
    }

    let inv_det = 1.0 / det as f64;

    let inverted_a = vec![
        vec![
            inv_det * system.0[1][1] as f64,
            -inv_det * system.0[0][1] as f64,
        ],
        vec![
            -inv_det * system.0[1][0] as f64,
            inv_det * system.0[0][0] as f64,
        ],
    ];

    Some(vec![
        inverted_a[0][0] * system.1[0] as f64 + inverted_a[0][1] * system.1[1] as f64,
        inverted_a[1][0] * system.1[0] as f64 + inverted_a[1][1] * system.1[1] as f64,
    ])
}
