use std::str::Chars;

fn main() {
    let src = include_str!("src1.txt");
//     let src = "MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX
// ";

    let src = src
        .lines()
        .map(str::chars)
        .map(Chars::collect)
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for x in 0..src[0].len() {
        for y in 0..src.len() {
            // println!("{x},{y} = {}", src[y][x]);
            if src[y][x] == 'X' {
                count += check_xmas(&src, x, y);
                // println!("{count}");
            }
        }
    }

    println!("{count}");
}

fn check_xmas(src: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    check_h(src, x, y)
        + check_v(src, x, y)
        + check_diag_positiv(src, x, y)
        + check_diag_negativ(src, x, y)
}

fn check_h(src: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    (x >= 3 && src[y][(x - 3)..=x].iter().rev().collect::<String>() == "XMAS") as usize
        + (x <= src[y].len() - 4 && src[y][x..(x + 4)].iter().collect::<String>() == "XMAS")
            as usize
}

fn check_v(src: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    (y >= 3
        && src[(y - 3)..=y]
            .iter()
            .map(|l| l[x])
            .rev()
            .collect::<String>()
            == "XMAS") as usize
        + (y <= src.len() - 4 && src[y..(y + 4)].iter().map(|l| l[x]).collect::<String>() == "XMAS")
            as usize
}

fn check_diag_positiv(src: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let top = if x >= 3 && y >= 3 {
        let s = ((x - 3)..=x)
            .rev()
            .zip(((y - 3)..=y).rev())
            .map(|(x, y)| src[y][x])
            .collect::<String>();
        s == "XMAS"
    } else {
        false
    } as usize;
    let bottom = if x <= src[y].len() - 4 && y <= src.len() - 4 {
        let s = (x..(x + 4))
            .zip(y..(y + 4))
            .map(|(x, y)| src[y][x])
            .collect::<String>();
        s == "XMAS"
    } else {
        false
    } as usize;
    top + bottom
}

fn check_diag_negativ(src: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let top = if x <= src[y].len() - 4 && y >= 3 {
        let s = (x..(x + 4))
            .zip(((y - 3)..=y).rev())
            .map(|(x, y)| src[y][x])
            .collect::<String>();
        s == "XMAS"
    } else {
        false
    } as usize;
    let bottom = if x >= 3 && y <= src.len() - 4 {
        let s = ((x - 3)..=x)
            .rev()
            .zip(y..(y + 4))
            .map(|(x, y)| src[y][x])
            .collect::<String>();
        s == "XMAS"
    } else {
        false
    } as usize;
    top + bottom
}
