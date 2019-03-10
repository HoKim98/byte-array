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

// Core
mod builder;
mod byte_array;

// Built-in Implementation
mod numeric;
mod bytes;
mod string;

pub use self::builder::BinaryBuilder;
pub use self::byte_array::ByteArray;
