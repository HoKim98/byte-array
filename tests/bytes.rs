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

use byte_array::ByteArray;

#[test]
fn test() {

    type Bytes = Vec<u8>;

    let mut ba: ByteArray = ByteArray::new();

    ba <<= &vec!(2, 0, 1, 9);
    ba <<= &vec!(1, 2, 3, 4);
    ba <<= &vec!();

    ba.seek_first();

    assert_eq!(ba.read::<Bytes>(), [2, 0, 1, 9]);
    assert_eq!(ba.read::<Bytes>(), [1, 2, 3, 4]);
    assert_eq!(ba.read::<Bytes>(), []);

    assert_eq!(ba.bytes_available(), 0);
    assert_eq!(ba.now(), 8 * 3 + 8);

}
