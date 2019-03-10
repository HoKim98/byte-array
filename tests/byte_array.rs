/* ------------------------------------------------------------
    ByteArray
    Project.Github: "https://github.com/kerryeon/bytearray"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/11/2019"
------------------------------------------------------------ */

use byte_array::ByteArray;

#[test]
fn test() {

    let mut ba_out: ByteArray = ByteArray::new();
    let mut ba_inn: ByteArray = ByteArray::new();

    ba_inn <<= &(1 as u8);
    ba_inn <<= &(2 as u8);
    ba_inn <<= &(3 as u16);

    ba_inn.seek_first();
    ba_inn.seek_next(1);

    /* ba_inn
        raw: 4 bytes
        ptr: 1
    */

    ba_out <<= &ba_inn;

    let mut ba_inn_new: ByteArray = ba_out.read();

    assert_eq!(ba_inn_new.read::<u8 >(), 2);
    assert_eq!(ba_inn_new.read::<u16>(), 3);

    assert_eq!(ba_inn_new.bytes_available(), 0);
    assert_eq!(ba_inn_new.now(), 4);

}
