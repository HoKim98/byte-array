/* ------------------------------------------------------------
    ByteArray
    Project.Github: "https://github.com/kerryeon/bytearray"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/8/2019"
------------------------------------------------------------ */

use byteorder::{
    BigEndian,
    ByteOrder,
};
use crate::{
    BinaryBuilder,
    ByteArray,
};
use std::mem::size_of;

macro_rules! generate {
($type: ident with default: $default: expr, toward: $toward: ident,
    endian: $endian: ident, read: $read: ident, write: $write: ident) => {

impl BinaryBuilder for $type {
    fn new() -> Self {
        $default
    }
    fn from_raw(ba: &mut ByteArray) -> Self {
        let now = ba.now();
        ba.seek_next(size_of::<Self>());
        $endian::$read(&ba.as_vec()[now..]) as $type
    }
    fn to_raw(&self, ba: &mut ByteArray) {
        let len = ba.len();
        let vec = ba.as_vec();
        // Alloc
        for _ in 0..size_of::<Self>() {
            vec.push(0);
        }
        $endian::$write(&mut vec[len..], *self as $toward);
    }
}

};}

generate!(u16   with default: 0 , toward: u16, endian: BigEndian, read: read_u16, write: write_u16);
generate!(u32   with default: 0 , toward: u32, endian: BigEndian, read: read_u32, write: write_u32);
generate!(u64   with default: 0 , toward: u64, endian: BigEndian, read: read_u64, write: write_u64);

generate!(i16   with default: 0 , toward: i16, endian: BigEndian, read: read_i16, write: write_i16);
generate!(i32   with default: 0 , toward: i32, endian: BigEndian, read: read_i32, write: write_i32);
generate!(i64   with default: 0 , toward: i64, endian: BigEndian, read: read_i64, write: write_i64);

generate!(usize with default: 0 , toward: u64, endian: BigEndian, read: read_u64, write: write_u64);
generate!(isize with default: 0 , toward: i64, endian: BigEndian, read: read_i64, write: write_i64);

generate!(f32   with default: 0., toward: f32, endian: BigEndian, read: read_f32, write: write_f32);
generate!(f64   with default: 0., toward: f64, endian: BigEndian, read: read_f64, write: write_f64);
