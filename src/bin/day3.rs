static ASCII_ZERO: i8 = 48;
static ASCII_ONE: i8 = 49;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn part_one<'a>(file: &[u8]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use std::mem;

    let zmask = _mm_set1_epi8(ASCII_ZERO);
    let omask = _mm_set1_epi8(ASCII_ONE);
    let mut ptr = file.as_ptr();

    // 12 bytes relevant, rest isn't.
    // if a byte is positive, 0 was more common. else, 1 was more common.
    let mut res = _mm_set1_epi8(0);

    for _ in (0..file.len() - 8).step_by(13) {
        let v = _mm_loadu_si128(ptr as *const __m128i);
        let zero_mask = _mm_xor_si128(zmask, v);
        let one_mask = _mm_xor_si128(omask, v);

        res = _mm_sub_epi8(res, zero_mask);
        res = _mm_add_epi8(res, one_mask);

        ptr = ptr.add(13);
    }

    let res_arr = mem::transmute::<__m128i, [i8; 16]>(res);
    let mut epsilon = String::new();
    let mut gamma = String::new();

    for v in &res_arr[0..12] {
        let zero_most_common = *v > 0;
        if zero_most_common {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    println!("{}", i32::from_str_radix(&gamma, 2).unwrap());
    println!("{}", i32::from_str_radix(&epsilon, 2).unwrap());
}

fn main() {
    let s = vec![include_bytes!("../../day3.txt").to_vec(), [0u8; 8].to_vec()].concat();
    unsafe { part_one(&s) };
}
