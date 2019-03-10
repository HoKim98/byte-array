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

#![feature(external_doc)]
#[doc(include = "../README.md")]

mod builder;
mod byte_array;
mod bytes;
mod numeric;
mod string;

pub use self::builder::BinaryBuilder;
pub use self::byte_array::ByteArray;
