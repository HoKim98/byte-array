/* ------------------------------------------------------------
    ByteArray
    Project.Github: "https://github.com/kerryeon/bytearray"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/10/2019"
------------------------------------------------------------ */

use crate::{
    BinaryBuilder,
    ByteArray,
};

impl BinaryBuilder for Vec<u8> {
    fn new() -> Self {
        vec![]
    }
    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        // Length
        let len: usize = ba.read_safe()?;
        // Bytes Begin
        let now: usize = ba.now();
        // Bytes End
        ba.seek_next(len)?;
        // Read
        let mut raw = vec![0; len];
        raw.clone_from_slice(&ba.as_vec()[now..now+len]);
        // Return
        Some(raw)
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        // Length
        ba <<= &self.len();
        // Write
        ba.as_vec().extend_from_slice(self);
    }
}
