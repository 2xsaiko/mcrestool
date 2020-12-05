use std::io;
use std::io::{Read, Write};

use byteorder::ReadBytesExt;

pub fn encode_min(num: i64) -> u64 {
    let u_num = num as u64;
    (u_num << 1 ^ (num >> 63) as u64) | u_num >> 63
}

pub fn decode_min(num: u64) -> i64 {
    (num >> 1) as i64 ^ ((num << 63) as i64) >> 63
}

pub fn varint_write<W: Write>(num: u64, mut pipe: W) -> io::Result<usize> {
    let mut num_pos = 0;
    let mut idx = 0;
    let mut buf = [0; 9];
    let data_bits = 64 - num.leading_zeros();

    loop {
        let next = if data_bits - num_pos > 7 {
            0b10000000
        } else {
            0
        };
        let piece = (num >> num_pos) as u8 & 0b01111111 | next;
        buf[idx] = piece;
        idx += 1;
        num_pos += 7;

        if !(num_pos < data_bits) {
            break;
        }
    }

    pipe.write(&buf[..idx])?;

    Ok(idx)
}

pub fn varint_read<R: Read>(mut pipe: R) -> io::Result<u64> {
    let mut offset = 0;
    let mut num = 0;

    loop {
        let byte = pipe.read_u8()?;
        let has_next = byte & 0b10000000 != 0;
        num |= (byte as u64 & 0b01111111) << offset;
        offset += 7;

        if !has_next {
            break;
        }
    }

    Ok(num)
}

#[test]
fn test_encode_min() {
    for i in -5..5 {
        let r = encode_min(i);
        println!("{} = {:016X}", i, r);
        assert_eq!(i, decode_min(r));
    }

    for i in i64::MAX - 5..=i64::MAX {
        let r = encode_min(i);
        println!("{} = {:016X}", i, r);
        assert_eq!(i, decode_min(r));
    }

    for i in i64::MIN..i64::MIN + 5 {
        let r = encode_min(i);
        println!("{} = {:016X}", i, r);
        assert_eq!(i, decode_min(r));
    }
}

#[test]
fn test_varint() {
    use std::io::Cursor;

    let mut buf = Cursor::new(Vec::new());

    varint_write(0, &mut buf).unwrap();
    varint_write(16, &mut buf).unwrap();
    varint_write(234567892322414124, &mut buf).unwrap();
    varint_write(encode_min(-18), &mut buf).unwrap();
    varint_write(encode_min(20000000), &mut buf).unwrap();

    buf.set_position(0);
    for item in buf.get_ref().iter() {
        print!("{:02X}", item);
        if item & 0b10000000 == 0 {
            print!("] ");
        } else {
            print!(" ");
        }
    }
    println!();

    assert_eq!(0, varint_read(&mut buf).unwrap());
    assert_eq!(16, varint_read(&mut buf).unwrap());
    assert_eq!(234567892322414124, varint_read(&mut buf).unwrap());
    assert_eq!(encode_min(-18), varint_read(&mut buf).unwrap());
    assert_eq!(encode_min(20000000), varint_read(&mut buf).unwrap());
}
