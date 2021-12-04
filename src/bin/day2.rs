#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
unsafe fn submarine_sse<'a>(iter: impl Iterator<Item = &'a str>) -> (i32, i32) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let mask = _mm_setr_epi32(7, 4, 2, 0);
    let mut res = _mm_set1_epi32(0);
    let mut res_arr: [i32; 4] = [0; 4];

    for l in iter {
        let (left, right) = l.split_once(' ').unwrap();
        let right = _mm_set1_epi32(right.parse::<i32>().unwrap());

        res = _mm_add_epi32(
            res,
            _mm_and_si128(
                _mm_cmpeq_epi32(mask, _mm_set1_epi32(left.len() as i32)),
                right,
            ),
        );
    }

    _mm_store_si128(res_arr.as_mut_ptr() as *mut __m128i, res);

    (
        *res_arr.get_unchecked(0),
        res_arr.get_unchecked(1) - res_arr.get_unchecked(2),
    )
}

fn submarine<'a>(iter: impl Iterator<Item = &'a str>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;

    for l in iter {
        let (left, right) = l.split_once(' ').unwrap();
        let right = right.parse::<i32>().unwrap();

        match left.len() {
            7 => horizontal += right,
            4 => depth += right,
            2 => depth -= right,
            _ => unreachable!(),
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
            }
            "down" => aim += right,
            "up" => aim -= right,
            _ => unreachable!(),
        }
    }

    (horizontal, depth)
}

fn main() {
    let input = include_str!("../../day2.txt");

    let (_horizontal, _depth) = unsafe { submarine_sse(input.lines()) };
    println!(
        "non-simd: {}",
        easybench::bench_env(input.lines(), |l| submarine(l))
    );
    unsafe {
        println!(
            "simd: {}",
            easybench::bench_env(input.lines(), |l| submarine_sse(l))
        )
    };
    // println!(
    //     "pt one: horizontal {} depth {} h x d {}",
    //     horizontal,
    //     depth,
    //     horizontal * depth
    // );
    // let (horizontal, depth) = submarine_part_two(input.lines());
    // println!("pt two: horizontal {} depth {} h x d {}", horizontal, depth, horizontal * depth);
}
