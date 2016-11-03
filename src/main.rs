// Copyright 2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(simd)]

extern crate simd;

use simd::u8x16;

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
struct Unalign<T>(T);

#[inline(always)]
pub unsafe fn load16_unaligned(ptr: *const u8) -> u8x16 {
    let loaded = *(ptr as *const Unalign<u8x16>);
    loaded.0
//    (*(ptr as *const Unalign<u8x16>)).0
}

fn main() {
        let ascii: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let simd = unsafe{ load16_unaligned(ascii.as_ptr())};
        println!("{:?}", simd);
}
