// not simd this time. couldn't be bothered

fn submarine<'a>(iter: impl Iterator<Item = &'a str>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;

    for l in iter {
        let (left, right) = l.split_once(' ').unwrap();
        let right = right.parse::<i32>().unwrap();

        match left {
            "forward" => horizontal += right,
            "down" => depth += right,
            "up" => depth -= right,
            _ => unreachable!() 
        }
    }

    (horizontal, depth)
}

fn submarine_part_two<'a>(iter: impl Iterator<Item = &'a str>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in iter {
        let (left, right) = l.split_once(' ').unwrap();
        let right = right.parse::<i32>().unwrap();

        match left {
            "forward" => {
                horizontal += right;
                depth += aim * right;
            },
            "down" => aim += right,
            "up" => aim -= right,
            _ => unreachable!() 
        }
    }

    (horizontal, depth)
}

fn main() {
    let input = include_str!("../../day2.txt");
    let (horizontal, depth) = submarine(input.clone().lines());
    println!("pt one: horizontal {} depth {} h x d {}", horizontal, depth, horizontal * depth);
    let (horizontal, depth) = submarine_part_two(input.lines());
    println!("pt two: horizontal {} depth {} h x d {}", horizontal, depth, horizontal * depth);
}