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

    let mut ba: ByteArray = ByteArray::new();

    let msg1 = "hello world";       // 8 + 11 bytes
    let msg2 = "Sweet \n Potato";   // 8 + 14 bytes
    let msg3 = "안녕하십니까";        // 8 + 18 bytes

    ba <<= &msg1.to_owned();
    ba <<= &msg2.to_owned();
    ba <<= &msg3.to_owned();

    ba.seek_first();

    assert_eq!(ba.read::<String>(), msg1.to_owned());
    assert_eq!(ba.read::<String>(), msg2.to_owned());
    assert_eq!(ba.read::<String>(), msg3.to_owned());

    assert_eq!(ba.bytes_available(), 0);
    assert_eq!(ba.now(), 8 * 3 + 11 + 14 + 18);

}
