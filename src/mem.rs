#[allow(warnings)]
#[cfg(target_pointer_width = "16")]
type c_int = i16;
#[allow(warnings)]
#[cfg(not(target_pointer_width = "16"))]
type c_int = i32;

// mem functions have been rewritten to copy 8 byte chunks.  No compensation for
// alignment is made here with the requirement that the underlying hardware
// supports unaligned read/writes.

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i: isize = 0;
    let chunks = ((n as isize - i) / 8) as isize;
    if chunks != 0 {
        let s1_64 = s1 as *const _ as *const u64;
        let s2_64 = s2 as *const _ as *const u64;
        while i < chunks {
            let a = *s1_64.offset(i);
            let b = *s2_64.offset(i);
            if a != b {
                break;
            }
            i += 1;
        }
        i *= 8;
    }
    while i < n as isize {
        let a = *s1.offset(i);
        let b = *s2.offset(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i: isize = 0;
    let chunks = ((n as isize - i) / 8) as isize;
    if chunks != 0 {
        let dest_64 = dest as *mut _ as *mut u64;
        let src_64 = src as *const _ as *const u64;
        while i < chunks {
            *dest_64.offset(i) = *src_64.offset(i);
            i += 1;
        }
        i *= 8;
    }
    while i < n as isize {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    dest
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let chunks = (n / 8) as isize;
        let mut i = chunks;
        if i > 0 {
            let dest_64 = dest as *mut _ as *mut u64;
            let src_64 = src as *const _ as *const u64;
            while i > 0 {
                i -= 1;
                *dest_64.offset(i) = *src_64.offset(i);
            }
        }
        i = n as isize;
        while i > chunks * 8 {
            i -= 1;
            *dest.offset(i) = *src.offset(i);
        }
    } else {
        // copy from beginning
        let mut i: isize = 0;
        let chunks = ((n as isize - i) / 8) as isize;
        if chunks != 0 {
            let dest_64 = dest as *mut _ as *mut u64;
            let src_64 = src as *const _ as *const u64;
            while i < chunks {
                *dest_64.offset(i) = *src_64.offset(i);
                i += 1;
            }
            i *= 8;
        }
        while i < n as isize {
            *dest.offset(i) = *src.offset(i);
            i += 1;
        }
    }
    dest
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: c_int, n: usize) -> *mut u8 {
    let mut i: isize = 0;
    let chunks = ((n as isize - i) / 8) as isize;
    if chunks != 0 {
        let mut c_64 = c as u64 & 0xFF as u64;
        c_64 |= c_64 << 8;
        c_64 |= c_64 << 16;
        c_64 |= c_64 << 32;
        let s_64 = s as *mut _ as *mut u64;
        while i < chunks {
            *s_64.offset(i) = c_64;
            i += 1;
        }
        i *= 8;
    }
    while i < n as isize {
        *s.offset(i) = c as u8;
        i += 1;
    }
    s
}
