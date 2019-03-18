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

use crate::{
    BinaryBuilder,
    ByteArray,
};
use std::mem::size_of;

macro_rules! generate {
($type: ident with default: $default: expr, from: $($from: tt )*) => {

impl BinaryBuilder for $type {
    fn new() -> Self {
        $default
    }
    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let now = ba.now();
        ba.seek_next(size_of::<Self>())?;
        Some(ba.as_vec()[now] $($from )*)
    }
    fn to_raw(&self, ba: &mut ByteArray) {
        ba.as_vec().push(*self as u8);
    }
}

};}

generate!(bool with default: false, from: != 0   );
generate!(u8   with default: 0,     from: as Self);
generate!(i8   with default: 0,     from: as Self);
