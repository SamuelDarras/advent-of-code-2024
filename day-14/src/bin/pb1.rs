use std::time::Instant;

const MAP_WIDTH: i64 = 101;
const MID_WIDTH: i64 = MAP_WIDTH / 2;
const MAP_HEIGHT: i64 = 103;
const MID_HEIGHT: i64 = MAP_HEIGHT / 2;

fn main() {
    let src = include_str!("src1.txt");
//     let src = "p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3
// ";
    let robots = src.lines().map(|line| {
        let mut split = line.split_whitespace();
        let p = split.next().unwrap();
        let v = split.next().unwrap();

        let mut p = p[2..].split(",");
        let p_x = p.next().unwrap().parse::<i64>().unwrap();
        let p_y = p.next().unwrap().parse::<i64>().unwrap();

        let mut v = v[2..].split(",");
        let v_x = v.next().unwrap().parse::<i64>().unwrap();
        let v_y = v.next().unwrap().parse::<i64>().unwrap();

        ((p_x, p_y), (v_x, v_y))
    });

    // let mut map: [HashMap<Point<i64>, usize>; 4] = core::array::from_fn(|_| HashMap::new());

    let mut quadrants = [0usize; 4];

    let start = Instant::now();
    let t = 100;
    (0..t);
    robots.for_each(|(p, v)| {
        let mut p_x = (p.0 + v.0 * t) % MAP_WIDTH;
        let mut p_y = (p.1 + v.1 * t) % MAP_HEIGHT;
        if p_x < 0 {
            p_x += MAP_WIDTH;
        }
        if p_y < 0 {
            p_y += MAP_HEIGHT;
        }
        if p_x == MID_WIDTH {
            return;
        }
        if p_y == MID_HEIGHT {
            return;
        }

        if p_x < MID_WIDTH && p_y < MID_HEIGHT {
            quadrants[0] += 1;
            // map[0].entry(p).or_default().add_assign(1);
        } else if p_x > MID_WIDTH && p_y < MID_HEIGHT {
            quadrants[1] += 1;
            // map[1].entr1(p).or_default().add_assign(1);
        } else if p_x < MID_WIDTH {
            quadrants[2] += 1;
            // map[2].entr1(p).or_default().add_assign(1);
        } else {
            quadrants[3] += 1;
            // map[3].entry(p).or_default().add_assign(1);
        }
    });
    // let security_factor = map
    //     .map(|quadrant| quadrant.values().sum::<usize>())
    //     .iter()
    //     .product::<usize>();
    let security_factor = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    let duration = start.elapsed();
    println!("{security_factor}, in {duration:?}");
}
