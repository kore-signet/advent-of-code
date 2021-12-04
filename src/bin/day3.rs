static ASCII_ZERO: i8 = 48;
static ASCII_ONE: i8 = 49;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn part_one<'a>(file: &[u8]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // set two masks, one for ascii_zero and one for ascii_one. you can think of this as a list of integers, set to the character: [48,48,48,48...]
    let zmask = _mm_set1_epi8(ASCII_ZERO);
    let omask = _mm_set1_epi8(ASCII_ONE);
    let mut ptr = file.as_ptr();

    // 12 bytes relevant, rest isn't.
    // if a byte is positive, 0 was more common. else, 1 was more common.
    let mut res = _mm_set1_epi8(0);

    for _ in (0..file.len() - 8).step_by(13) {
        // load in a line's worth of bytes at our current position
        let v = _mm_loadu_si128(ptr as *const __m128i);
        // XOR the current line against a mask for ascii zero.
        // so if the character is '0' this will yield the int 0, if the character is '1' this will yield 1
        let zero_mask = _mm_xor_si128(zmask, v);
        // same, but in reverse for ascii one
        let one_mask = _mm_xor_si128(omask, v);

        // subtract the results of our zero XOR from the accumulator.
        // so for every '1', it will subtract 1.
        res = _mm_sub_epi8(res, zero_mask);
        // add the results of our one XOR to the accumulator.
        // so for every '0', it will add 1.
        res = _mm_add_epi8(res, one_mask);

        // move pointer forward
        ptr = ptr.add(13);
    }

    let zero_slots_masked = _mm_cmpgt_epi8(res, _mm_set1_epi8(0));
    let one_slots_masked = _mm_cmplt_epi8(res, _mm_set1_epi8(0));

    let mut most_common_arr: [u8; 16] = [0; 16];
    let mut least_common_arr: [u8; 16] = [0; 16];

    _mm_maskmoveu_si128(
        zmask,
        zero_slots_masked,
        most_common_arr.as_mut_ptr() as *mut i8,
    );
    _mm_maskmoveu_si128(
        omask,
        one_slots_masked,
        most_common_arr.as_mut_ptr() as *mut i8,
    );

    _mm_maskmoveu_si128(
        omask,
        zero_slots_masked,
        least_common_arr.as_mut_ptr() as *mut i8,
    );
    _mm_maskmoveu_si128(
        zmask,
        one_slots_masked,
        least_common_arr.as_mut_ptr() as *mut i8,
    );

    let _gamma_str = println!(
        "gamma {}",
        i32::from_str_radix(&String::from_utf8_lossy(&most_common_arr[0..12]), 2).unwrap()
    );
    println!(
        "epsilon {}",
        i32::from_str_radix(&String::from_utf8_lossy(&least_common_arr[0..12]), 2).unwrap()
    );
}

fn main() {
    let s = vec![include_bytes!("../../day3.txt").to_vec(), [0u8; 8].to_vec()].concat();
    unsafe { part_one(&s) };
}
