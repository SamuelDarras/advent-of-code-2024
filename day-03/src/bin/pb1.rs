fn main() {
    let src = include_str!("src1.txt");
    // let src = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[derive(Debug)]
    enum State {
        None,
        Mul(u8),
        Open,
        Num(String),
        Separator,
    }

    let mut state = State::None;
    let mut left = 0;
    let mut right;
    let mut result = 0;
    for c in src.chars() {
        let next_state = match (c, &state) {
            ('m', _) => State::Mul(1),
            ('u', State::Mul(1)) => State::Mul(2),
            ('l', State::Mul(2)) => State::Mul(3),
            ('(', State::Mul(3)) => State::Open,
            (c, State::Open) if c.is_numeric() => State::Num(String::from(c)),
            (c, State::Num(s)) if c.is_numeric() => State::Num(format!("{s}{c}")),
            (',', State::Num(s)) => {
                left = s.parse::<u32>().unwrap();
                State::Separator
            }
            (c, State::Separator) if c.is_numeric() => State::Num(String::from(c)),
            (c, State::Num(s)) if c.is_numeric() => State::Num(format!("{s}{c}")),
            (')', State::Num(s)) => {
                right = s.parse::<u32>().unwrap();
                result += left * right;
                State::None
            },
            _ => State::None,
        };
        state = next_state;
    }

    println!("{result}");
}
