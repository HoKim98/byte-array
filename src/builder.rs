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

use crate::ByteArray;

/// This trait helps you handle user-defined structs via `ByteArray`.
///
/// # Examples
///
/// ```
/// use byte_array::{
///    BinaryBuilder,
///    ByteArray,
/// };
///
/// struct Foo(u64);
///
/// impl BinaryBuilder for Foo {
///    fn new() -> Self {
///        Self(123)
///    }
///
///    fn from_raw(ba: &mut ByteArray) -> Self {
///        Self(ba.read())
///    }
///    fn to_raw(&self, mut ba: &mut ByteArray) {
///        ba <<= &self.0;
///    }
///}
///
/// let mut ba = ByteArray::new();
///
/// // Write
/// ba <<=   &Foo::new();
/// ba <<=   &Foo(321);
/// ba.write(&Foo(222));
///
/// // Read
/// ba.seek_first();
/// assert_eq!(123, ba.read::<u64>());
/// assert_eq!(321, ba.read::<u64>());
/// assert_eq!(222, ba.read::<u64>());
/// ```
///
pub trait BinaryBuilder: Sized {

    /// Creates an empty Data.
    ///
    fn new() -> Self;

    /// Gets the data of the specified type in the `ByteArray`.
    ///
    /// # Panics
    /// Panics when unexpected EOF.
    ///
    fn from_raw(ba: &mut ByteArray) -> Self;

    /// Adds data to the `ByteArray`.
    ///
    fn to_raw(&self, ba: &mut ByteArray);
}
