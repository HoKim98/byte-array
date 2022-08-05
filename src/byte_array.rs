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
/// assert_eq!(1234, ba.read_safe::<u16>().unwrap());
/// ```
///
pub struct ByteArray {
    pub raw: BytesDyn,
    pub pointer: usize,
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
        self.read_safe::<T>().unwrap()
    }

    /// Gets the data of the specified type in the `ByteArray` safely.
    ///
    /// But, the data to be read must implement `BinaryBuilder` trait.
    ///
    pub fn read_safe<T>(&mut self) -> Option<T> where T: BinaryBuilder {
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

    /// Skips as many bytes as you want safely.
    ///
    pub fn seek_next(&mut self, stride: usize) -> Option<()> {
        self.pointer += stride;
        match self.pointer <= self.len() {
            true => Some(()),
            false => {
                self.pointer = self.len();
                None
            },
        }
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

    /// Gets the byte distance from the end of the data to the current read.
    ///
    pub fn bytes_available(&self) -> usize {
        self.len() - self.now()
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

    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let pointer: usize = ba.read_safe()?;
        let raw: BytesDyn = ba.read_safe()?;
        Some(ByteArray {
            raw,
            pointer,
        })
    }

    fn to_raw(&self, mut ba: &mut Self) {
        ba <<= &self.pointer;
        ba <<= &self.raw;
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
