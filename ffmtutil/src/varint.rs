use std::io::Write;
use std::io;

pub fn encode_min(num: i32) -> u32 {
    let u_num = num as u32;
    (u_num << 1 ^ (num >> 31) as u32) | u_num >> 31
}

pub fn decode_min(num: u32) -> i32 {
    (num >> 1) as i32 ^ ((num << 31) as i32) >> 31
}

pub fn varint_write<W: Write>(num: u32, mut pipe: W) -> io::Result<usize> {
    let mut bits_left = 32;
    let mut idx = 0;
    let mut buf = [0; 5];
    if bits_left - num.leading_zeros() > 7 {
        let piece = num & 0b01111111 | 0b10000000;
        buf[idx] = piece;
        idx += 1;
    }
    pipe.write(&buf[..idx]);

    Ok((0))
}

#[test]
fn test_encode_min() {
    for i in -5..5 {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, decode_min(r));
    }

    for i in i32::MAX - 5..=i32::MAX {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, decode_min(r));
    }

    for i in i32::MIN..i32::MIN + 5 {
        let r = encode_min(i);
        println!("{} = {:08X}", i, r);
        assert_eq!(i, decode_min(r));
    }
}