use itertools::*;

#[macro_export]
macro_rules! parse {
    ( $line:expr, $( $t:ty, $sep:expr ),* ; $lt:ty ) => {{
        let mut rest = $line;
        (
            $({
                let mut iter = rest.split($sep);
                let elem = iter.next().unwrap().parse::<$t>().unwrap();
                rest = iter.next().unwrap();
                println!("{:?}", elem);
                elem
            },)*
            rest.parse::<$lt>().unwrap(),
        )
    }};
}

// ---------- adjust these to customize parsing ---------- //
type Line = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);
fn parse(line: &str) -> Line {
    parse!(line, String, " ", String, " ", String, " ", String, " ", String, " ", String, " ", String, " ", String, " ", String, " ", String, " | ", String, " ", String, " ", String, " " ; String)
}
// ------------------------------------------------------- //

fn main() {
    let input = include_str!("../input/input.txt");
    let input = input.lines().map(str::to_string).collect_vec();

    part1(input.clone());

    part2(input.clone());
}

fn part1(input: Vec<String>) {
    let mut count = 0;

    for line in input {
        let os = line.split(" | ").nth(1).unwrap();

        for o in os.split(" ") {
            match o.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }

    println!("{:?}", count)
}

fn overlap(s1: &str, s2: &str) -> usize {
    let mut count = 0;
    for a in s1.chars() {
        if s2.contains(a) {
            count += 1;
        }
    }
    count
}

fn in_a_not_in_b(s1: &str, s2: &str) -> char {
    for a in s1.chars() {
        if !s2.contains(a) {
            return a;
        }
    }
    ' '
}

fn part2(input: Vec<String>) {
    let mut count = 0;

    for line in input {
        // let os = line.split(" | ").nth(1).unwrap();
        let mut bruh = line.split(" | ");
        let mut yeet = bruh.join(" ");
        let vec = yeet.split(" ").collect_vec();

        let mut one = "".to_string();
        let mut four = "".to_string();
        let mut seven = "".to_string();
        let mut eight = "".to_string();

        for &o in vec.iter() {
            match o.len() {
                2 => one = o.to_owned(),
                3 => seven = o.to_owned(),
                4 => four = o.to_owned(),
                7 => eight = o.to_owned(),
                _ => {}
            }
        }

        let mut two = "".to_string();
        let mut three = "".to_string();
        let mut five = "".to_string();
        let mut six = "".to_string();
        let mut nine = "".to_string();
        let mut zero = "".to_string();
        for &o in vec.iter() {
            if o.len() == 5 {
                if overlap(o, &four) == 2 {
                    two = o.to_owned();
                } else if overlap(o, &seven) == 3 {
                    three = o.to_owned();
                } else {
                    five = o.to_owned();
                }
            }
            if o.len() == 6 {
                if overlap(o, &seven) == 2 {
                    six = o.to_owned();
                } else if overlap(o, &four) == 4 {
                    nine = o.to_owned();
                } else {
                    zero = o.to_owned();
                }
            }
        }

        let mut a = in_a_not_in_b(&seven, &one);
        let mut b = in_a_not_in_b(&nine, &three);
        let mut c = in_a_not_in_b(&eight, &six);
        let mut d = in_a_not_in_b(&eight, &zero);
        let mut e = in_a_not_in_b(&eight, &nine);

        let mut rem_1 = ' ';
        let mut rem_2 = ' ';
        for ch in "abcdefg".chars() {
            if ch == a || ch == b || ch == c || ch == d || ch == e {
            } else {
                rem_1 = ch
            }
        }
        for ch in "abcdefg".chars() {
            if ch == a || ch == b || ch == c || ch == d || ch == e || ch == rem_1 {
            } else {
                rem_2 = ch
            }
        }

        let mut f = ' ';
        let mut g = ' ';

        if one.contains(rem_1) {
            f = rem_1;
            g = rem_2
        } else {
            f = rem_2;
            g = rem_1
        }

        // println!("{}{}{}{}{}{}{}", a, b, c, d, e, f, g);

        let ten: i32 = 10;
        let mut num = 0;
        for (i, &o) in vec[10..vec.len()].iter().rev().enumerate() {
            if o.len() == 2 {
                num += 1 * ten.pow(i as u32);
            } else if o.len() == 3 {
                num += 7 * ten.pow(i as u32);
            } else if o.len() == 4 {
                num += 4 * ten.pow(i as u32);
            } else if o.len() == 7 {
                num += 8 * ten.pow(i as u32);
            } else if o.len() == 5 {
                if overlap(o, &two) == 5 {
                    num += 2 * ten.pow(i as u32);
                } else if overlap(o, &three) == 5 {
                    num += 3 * ten.pow(i as u32);
                } else {
                    num += 5 * ten.pow(i as u32);
                }
            } else if o.len() == 6 {
                if overlap(o, &zero) == 6 {
                } else if overlap(o, &six) == 6 {
                    num += 6 * ten.pow(i as u32);
                } else {
                    num += 9 * ten.pow(i as u32);
                }
            }
        }
        println!("{}", num);
        count += num;
    }

    println!("{:?}", count)
}
