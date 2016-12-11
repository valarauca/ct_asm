//!Nightly Specific Methods
//!
//!These methods are handed rolled aseembly that are guaranteed to be
//!correct. This extensions is born out of the frustration of
//!constantly fighting the rust compiler to do the right thing.

#![feature(asm)]
#![no_std]


use core::mem::transmute as trans;


#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn tick() -> u64 {
    let out: u64;
    unsafe{asm!("
        lfence;
        xor rdx, rdx;
        xor rax, rax;
        rdtsc
        shr rdx, 32;
        or rax, rdx"
        :
            "={rax}"(out)
        :
        :
            "rdx",
            "rax"
        :
            "volatile",
            "intel"
    );
    out
}}
#[test]
#[allow(unused_assignments)]
fn test_tick() {
    let mut delta = 0u64;
    let mut start = 0u64;
    let mut end = 0u64;
    start = tick();
    end = tick();
    delta = end - start;
    assert!( delta < 100u64);
}

#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ct_equal_u64(x: u64, y: u64) -> bool {
    let r: u8;
    //assembly code
    unsafe{asm!("
        xor r10, r11;
        xor r10, -1;
        xor r11, r11;
        mov r11, r10;
        shr r10, 32;
        and r10, r11;
        mov r11, r10;
        shr r10, 16;
        and r10, r11;
        mov r11, r10;
        shr r10, 8;
        and r10, r11;
        mov r11, r10;
        shr r10, 4;
        and r10, r11;
        mov r11, r10;
        shr r10, 2;
        and r10, r11;
        mov r11, r10;
        shr r10, 1;
        and r10, r11;
        and r10, 1"
    : //output
        "={r10b}"(r)
    : //input
        "{r10}"(x),
        "{r11}"(y)
    : //modified registers
        "r10",
        "r11"
    : //args
        "volatile",
        "intel"
    );
    trans(r)
}}


#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ct_equal_u8(x: u8, y: u8) -> bool {
    let r: u8;
    //assembly code
    unsafe{asm!("
        xor r10b, r11b;
        xor r10b, -1;
        xor r11b, r11b;
        mov r11b, r10b;
        shr r10b, 4;
        and r10b, r11b;
        mov r11b, r10b;
        shr r10b, 2;
        and r10b, r11b;
        mov r11b, r10b;
        shr r10b, 1;
        and r10b, r11b;
        and r10b, 1"
    : //output
        "={r10b}"(r)
    : //input
        "{r10b}"(x),
        "{r11b}"(y)
    : //modified registers
        "r10",
        "r11"
    : //args
        "volatile",
        "intel"
    );
    trans(r)
}}
#[test]
#[allow(unused_assignments)]
fn test_ct_equal_u8() {
    let mut dir = false;
    let mut t = 0u64;
    let mut f = 0u64;
    let mut start = 0u64;
    let mut end = 0u64;
    let mut delta = 0u64;
    let mut t_count = 0u64;
    let mut f_count = 0u64;
    for x in 0..255u8 {
        for y in 0..255u8 {
            start = tick();
            dir = ct_equal_u8(x,y);
            end = tick();
            delta = end - start;
            if dir {
                assert_eq!(x,y);
                t_count += 1;
                t += delta;

            } else {
                assert!( x != y);
                f_count += 1;
                f += delta;
            }
        }
    }
    assert_eq!(t_count, 255);
    assert_eq!(f_count, 64770);
}

#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ct_optional_swap_u64(flag: bool, x: u64, y: u64) -> u64 {
    let ret: u64;
    unsafe{ asm!("
        sub r8, 1;
        mov r9, r8;
        xor r9, -1;
        and r10, r9;
        and r11, r8;
        xor rax, rax;
        or  rax, r11;
        or  rax, r10;"
        :
            "={rax}"(ret)
        :
            "{r8}"(flag),
            "{r10}"(x),
            "{r11}"(y)
        :
            "rax", "r8", "r9", "r10", "r11"
        :
            "volatile", "intel"
    );
    ret
}}
#[test]
fn test_ct_optional_swap_u64() {
    let ret = ct_optional_swap_u64(true,5,6);
    assert_eq!(ret, 5);
    let ret = ct_optional_swap_u64(false,5,6);
    assert_eq!(ret,6);
}


///Check if two slices are identical.
///
///#Panic:
///
///This function will panic if the two slices do not have equal length
#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "x86_64")]
pub fn ct_slice_eq(x: &[u8], y: &[u8]) -> bool {
    let x_len = x.len() as u64;
    let y_len = y.len() as u64;
    let x_ptr = x.as_ptr();
    let y_ptr = y.as_ptr();
    let mut flag = 0u8;
    if ! ct_equal_u64(x_len,y_len) {
        panic!("CT ASM: attempted to equate non equal length slices.");
    }
    for i in 0..x_len {
        unsafe{ asm!("
            mov r10b, [r8+rdx];
            mov r11b, [r9+rdx];
            xor r10b, r11b;
            or  cl, r10b;"
            :
                "={cl}"(flag)
            :
                "{cl}"(flag),
                "{r8}"(x_ptr),
                "{r9}"(y_ptr),
                "{rdx}"(i)
            :
                "r8", "r9", "r10", "r11", "rdx", "rcx"
            :
                "volatile", "intel"
        )};
    }
    ct_equal_u8(flag,0)
}
#[test]
fn test_ct_slice_eq() {
    let a = b"hello";
    let b = b"world";
    assert!( ct_slice_eq(a,a));
    assert!( ct_slice_eq(b,b));
    assert!( ! ct_slice_eq(a,b));
    assert!( ! ct_slice_eq(b,a));
}
