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

use byte_array::ByteArray;

#[test]
fn test() {

    let mut ba: ByteArray = ByteArray::new();

    ba.write(&false);
    ba.write(&true );
    ba <<= &(false   as bool );
    ba <<= &(true    as bool );

    ba <<= &( 123e+0 as u8   );
    ba <<= &( 123e+2 as u16  );
    ba <<= &( 123e+5 as u32  );
    ba <<= &( 123e+7 as u64  );
    ba <<= &( 123e+7 as u128 );

    ba <<= &(-123e+0 as i8   );
    ba <<= &(-123e+2 as i16  );
    ba <<= &(-123e+5 as i32  );
    ba <<= &(-123e+7 as i64  );
    ba <<= &(-123e+7 as i128 );

    ba <<= &(1234567 as usize);
    ba <<= &(-123456 as isize);

    ba <<= &(-123.45 as f32  );
    ba <<= &(-123.45 as f64  );

    ba.seek_first();

    assert_eq!(ba.read::<bool >(), false  as bool );
    assert_eq!(ba.read::<bool >(), true   as bool );
    assert_eq!(ba.read::<bool >(), false  as bool );
    assert_eq!(ba.read::<bool >(), true   as bool );

    assert_eq!(ba.read::<u8   >(),  123e+0 as u8  );
    assert_eq!(ba.read::<u16  >(),  123e+2 as u16 );
    assert_eq!(ba.read::<u32  >(),  123e+5 as u32 );
    assert_eq!(ba.read::<u64  >(),  123e+7 as u64 );
    assert_eq!(ba.read::<u128 >(),  123e+7 as u128);

    assert_eq!(ba.read::<i8   >(), -123e+0 as i8  );
    assert_eq!(ba.read::<i16  >(), -123e+2 as i16 );
    assert_eq!(ba.read::<i32  >(), -123e+5 as i32 );
    assert_eq!(ba.read::<i64  >(), -123e+7 as i64 );
    assert_eq!(ba.read::<i128 >(), -123e+7 as i128);

    assert_eq!(ba.read::<usize>(), 1234567 as usize);
    assert_eq!(ba.read::<isize>(), -123456 as isize);

    assert_eq!(ba.read::<f32  >(), -123.45 as f32  );
    assert_eq!(ba.read::<f64  >(), -123.45 as f64  );

    assert_eq!(ba.bytes_available(), 0);
    assert_eq!(ba.now(), 94);

}
