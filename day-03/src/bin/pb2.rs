fn main() {
    let src = include_str!("src2.txt");
    // let src = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[derive(Debug)]
    enum State {
        None,
        Mul(u8),
        Do(u8),
        Dont(u8),
        Open,
        Num(String),
        Separator,
    }

    let mut state = State::None;
    let mut left = 0;
    let mut right;
    let mut result = 0;
    let mut enabled = true;
    for c in src.chars() {
        let next_state = match (c, &state) {
            ('d', _) => State::Do(1),
            ('o', State::Do(1)) => State::Do(2),
            ('n', State::Do(2)) => State::Dont(3),
            ('(', State::Do(2)) => State::Do(3),
            (')', State::Do(3)) => {
                enabled = true;
                State::None
            }
            ('\'', State::Dont(3)) => State::Dont(4),
            ('t', State::Dont(4)) => State::Dont(5),
            ('(', State::Dont(5)) => State::Dont(6),
            (')', State::Dont(6)) => {
                enabled = false;
                State::None
            }
            _ if !enabled => State::None,
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
            }
            _ => State::None,
        };
        state = next_state;
    }

    println!("{result}");
}
