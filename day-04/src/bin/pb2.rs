use std::str::Chars;

fn main() {
    let src = include_str!("src2.txt");
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
    for x in 1..(src[0].len() - 1) {
        for y in 1..(src.len() - 1) {
            // println!("{x},{y} = {}", src[y][x]);
            if src[y][x] == 'A' {
                // a.b
                // .A.
                // a.b
                if src[y - 1][x - 1] != src[y - 1][x + 1]
                    && "MS".contains(src[y - 1][x - 1])
                    && "MS".contains(src[y - 1][x + 1])
                    && src[y - 1][x - 1] == src[y + 1][x - 1]
                    && src[y - 1][x + 1] == src[y + 1][x + 1]
                {
                    count += 1;
                    println!(
                        "{}.{}\n.A.\n{}.{}",
                        src[y - 1][x - 1],
                        src[y - 1][x + 1],
                        src[y + 1][x - 1],
                        src[y + 1][x + 1]
                    );
                    println!();
                    continue;
                }
                // a.a
                // .A.
                // b.b
                if src[y - 1][x - 1] != src[y + 1][x - 1]
                    && "MS".contains(src[y - 1][x - 1])
                    && "MS".contains(src[y + 1][x - 1])
                    && src[y - 1][x - 1] == src[y - 1][x + 1]
                    && src[y + 1][x - 1] == src[y + 1][x + 1]
                {
                    count += 1;
                    println!(
                        "{}.{}\n.A.\n{}.{}",
                        src[y - 1][x - 1],
                        src[y - 1][x + 1],
                        src[y + 1][x - 1],
                        src[y + 1][x + 1]
                    );
                    println!();
                    continue;
                }
            }
        }
    }
    println!("{count}");
}
