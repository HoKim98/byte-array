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

use crate::BinaryBuilder;
use std::ops::ShlAssign;

type BytesDyn = Vec<u8>;

/// An object that can read and store time series data.
///
/// # Examples
///
/// ```
/// use byte_array::ByteArray;
///
/// let mut ba = ByteArray::new();
///
/// // Write
/// ba <<= &3.14;
/// ba.write(&(1234 as u16));
///
/// // Read
/// ba.seek_first();
/// assert_eq!(3.14, ba.read::<f64>());
/// assert_eq!(1234, ba.read::<u16>());
/// ```
///
pub struct ByteArray {
    raw: BytesDyn,
    pointer: usize,
}

impl ByteArray {

    /// Creates an empty `ByteArray`.
    ///
    pub fn new() -> Self {
        ByteArray {
            raw: vec!(),
            pointer: 0,
        }
    }

    /// Gets the data of the specified type in the `ByteArray`.
    ///
    /// But, the data to be read must implement `BinaryBuilder` trait.
    ///
    /// # Panics
    /// Panics when unexpected EOF.
    ///
    pub fn read<T>(&mut self) -> T where T: BinaryBuilder {
        T::from_raw(self)
    }

    /// Adds data to the `ByteArray`.
    ///
    /// But, the data to be read must implement `BinaryBuilder` trait.
    ///
    pub fn write<T>(&mut self, value: &T) where T: BinaryBuilder {
        value.to_raw(self)
    }

    /// Starts reading the data stored in the `ByteArray` from the beginning.
    ///
    pub fn seek_first(&mut self) {
        self.seek(0)
    }

    /// Makes the `ByteArray` ready for adding data.
    ///
    pub fn seek_last(&mut self) {
        self.seek(self.len())
    }

    /// Skips as many bytes as you want.
    ///
    pub fn seek_next(&mut self, stride: usize) {
        self.pointer += stride
    }

    /// Gets the actual data stored in the `ByteArray`.
    ///
    pub fn as_vec(&mut self) -> &mut BytesDyn {
        &mut self.raw
    }

    /// Gets the number of bytes of data stored in the `ByteArray`.
    ///
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Gets the byte distance from the start of the data to the current read.
    ///
    pub const fn now(&self) -> usize {
        self.pointer
    }

    fn seek(&mut self, value: usize) {
        self.pointer = value
    }
}

impl BinaryBuilder for ByteArray {

    fn new() -> Self {
        Self::new()
    }

    fn from_raw(raw: &mut Self) -> Self {
        ByteArray {
            raw: raw.as_vec().clone(),
            pointer: 0,
        }
    }

    fn to_raw(&self, ba: &mut Self) {
        ba.as_vec().append(&mut self.raw.clone());
    }
}

impl<'a, T> ShlAssign<&'a T> for ByteArray where T: BinaryBuilder {

    fn shl_assign(&mut self, rhs: &'a T) {
        self.write(rhs);
    }
}

impl<'a, T> ShlAssign<&'a T> for &mut ByteArray where T: BinaryBuilder {

    fn shl_assign(&mut self, rhs: &'a T) {
        self.write(rhs);
    }
}
