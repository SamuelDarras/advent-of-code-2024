fn main() {
    let src = include_str!("src1.txt");
    //     let src = "190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20
    // 10: 2 2 5
    // ";
    //     let src = "190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 108: 4 5 3 9
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20
    // 108: 2 2 5 12
    // 10: 2 5 1";

    let equations = src
        .lines()
        .map(|line| {
            let mut splitted = line.split(':');
            let result = splitted.next().unwrap().parse::<usize>().unwrap();
            let values = splitted
                .next()
                .unwrap()
                .split_whitespace()
                .map(str::trim)
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            (result, values)
        })
        .collect::<Vec<_>>();

    println!("{equations:?}");
    let correct = equations
        .iter()
        .filter(|(result, values)| {
            for i in 0..=(3usize.pow((values.len() - 1) as u32)) {
                let calculated_result =
                    values[1..]
                        .iter()
                        .enumerate()
                        .fold(values[0], |acc, (shift, value)| {
                            let operator_choice = (i / 3usize.pow(shift as u32)) % 3;
                            match operator_choice {
                                0b00 => acc + value,
                                0b01 => acc * value,
                                0b10 => format!("{acc}{value}").parse::<usize>().unwrap(),
                                _ => unreachable!(),
                            }
                        });
                if calculated_result == *result {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>();

    println!("{correct:?}");

    let result = correct.iter().map(|(r, _)| r).sum::<usize>();
    println!("{result}");
}
