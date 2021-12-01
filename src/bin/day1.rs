#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn part_one(input: &[i16]) -> u32 {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let (l, r) = input.split_at(input.len() % 16);
    let mut acc = if l.is_empty() {
        1
    } else {
        l.windows(2).fold(0u32, |acc, c| acc + (c[0] < c[1]) as u32)
    };

    let mut left_indices = _mm256_setr_epi32(1, 2, 3, 4, 5, 6, 7, 8);
    let mut right_indices = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
    let eight = _mm256_set1_epi32(16);

    for _ in (0..r.len()).step_by(16) {
        let lhs = _mm256_i32gather_epi32::<2>(r.as_ptr() as *const i32, left_indices);
        let rhs = _mm256_i32gather_epi32::<2>(r.as_ptr() as *const i32, right_indices);

        let mask = _mm256_cmpgt_epi16(lhs, rhs);
        acc += _mm256_movemask_epi8(mask).count_ones() / 2;

        left_indices = _mm256_add_epi32(left_indices, eight);
        right_indices = _mm256_add_epi32(right_indices, eight);
    }

    acc
}

unsafe fn part_two(input: &[i16]) -> u32 {
    let v = input
        .windows(3)
        .map(|c| (c[0] + c[1] + c[2]) as i16)
        .collect::<Vec<i16>>();

    part_one(&v[..])
}

fn main() {
    let val = include_str!("../../day1.txt")
        .lines()
        .map(|v| v.parse::<i16>().unwrap())
        .collect::<Vec<i16>>();

    unsafe { println!("{}", part_one(&val[..])) };
    unsafe { println!("{}", part_two(&val[..])) };
}
