use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
#[target_feature(enable = "simd128")]
pub fn proc(x: &mut [u8], len: usize) {
    use core::arch::wasm32::*;

    // create filter
    let one_byte_filter = u32x4_splat(0xFF);
    let alpha_filter = u32x4_splat(0xFF000000);
    let red_ch = u16x8_splat((0.2126 * 32768.0) as u16);
    let green_ch = u16x8_splat((0.7152 * 32768.0) as u16);
    let blue_ch = u16x8_splat((0.0722 * 32768.0) as u16);

    unsafe {
        // process 8 pixels (8 * 4 = 32 bytes)
        for i in (0..len).step_by(32) {
            // get pointer
            let y = x.as_mut_ptr().add(i) as *mut v128;
            // load block0 / block1
            let block0 = v128_load(y);
            let block1 = v128_load(y.add(1));
            // get red data
            let r = u16x8_narrow_i32x4(v128_and(block0, one_byte_filter), v128_and(block1, one_byte_filter));
            // get green data
            let g = u16x8_narrow_i32x4(
                v128_and(u32x4_shr(block0, 8), one_byte_filter),
                v128_and(u32x4_shr(block1, 8), one_byte_filter)
            );
            // get blue data
            let b = u16x8_narrow_i32x4(
                v128_and(u32x4_shr(block0, 16), one_byte_filter),
                v128_and(u32x4_shr(block1, 16), one_byte_filter)
            );
            // make to grayscale
            let result = u16x8_add_sat(
                u16x8_add_sat(
                    i16x8_q15mulr_sat(r, red_ch), i16x8_q15mulr_sat(g, green_ch)
                ),
                i16x8_q15mulr_sat(b, blue_ch)
            );
            
            // prepare to store
            // | <- low -- high -> |
            let out0 = u32x4_extend_low_u16x8(result);
            let out1 = u32x4_extend_high_u16x8(result);
            // store
            v128_store(y, u32x4_add(u32x4_add(out0, u32x4_shl(out0, 8)), u32x4_add(u32x4_shl(out0, 16), v128_and(block0, alpha_filter))));
            v128_store(y.add(1), u32x4_add(u32x4_add(out1, u32x4_shl(out1, 8)), u32x4_add(u32x4_shl(out1, 16), v128_and(block1, alpha_filter))));
        }
    }
}

#[wasm_bindgen]
pub fn proc2(x: &mut [u8], len: usize) {
    for i in (0..len).step_by(4) {
        let r = (x[i] as f32) * 0.2126;
        let g = (x[i + 1] as f32) * 0.7152;
        let b = (x[i + 2] as f32) * 0.0722;
        let mut result = r + g + b;
        if result >= 256.0 { result = 256.0; }
        x[i] = result as u8;
        x[i + 1] = result as u8;
        x[i + 2] = result as u8;
    }
}