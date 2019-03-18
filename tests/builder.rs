/* ------------------------------------------------------------
    ByteArray
    Project.Github: "https://github.com/kerryeon/bytearray"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/9/2019"
------------------------------------------------------------ */

use byte_array::{
    BinaryBuilder,
    ByteArray,
};

struct MyData {
    pub id: u64,
    pub pw: u64,
}

impl BinaryBuilder for MyData {
    fn new() -> Self {
        MyData {
            id: 2,
            pw: 3,
        }
    }

    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        Some(Self {
            id: ba.read_safe()?,
            pw: ba.read_safe()?,
        })
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.id;
        ba <<= &self.pw;
    }
}

#[test]
fn test() {

    let mut ba = ByteArray::new();

    let my_data = MyData::new();
    ba <<= &my_data;

    let my_data_recon: MyData = ba.read();
    assert_eq!(my_data.id, my_data_recon.id);
    assert_eq!(my_data.pw, my_data_recon.pw);

}

#[test]
fn test_mut() {

    let mut ba = &mut ByteArray::new();

    let my_data = MyData::new();
    ba <<= &my_data;

    let my_data_recon: MyData = ba.read();
    assert_eq!(my_data.id, my_data_recon.id);
    assert_eq!(my_data.pw, my_data_recon.pw);

    assert_eq!(ba.bytes_available(), 0);
    assert_eq!(ba.now(), 16);

}
