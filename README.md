# ByteArray
[![Version](https://docs.rs/byte_array/badge.svg)](https://crates.io/crates/byte_array) \
A library that supports `Java`-like series processing.
This is useful when you already know the format of your data.
On the other hand, the processing of invalid data is relatively poor,
so it is not recommended to use it for uncertain data.

# Installation
```toml
[dependencies]
byte_array = "0.1"
```

# Usage
```rust
use byte_array::ByteArray;

fn main() {

    // Create an empty ByteArray
    let mut ba = ByteArray::new();
    
    // Input data
    let a: f64      = 3.14;
    let b: u16      = 1234;
    let c: String   = String::from("hello");
    
    // Write data to ByteArray
    ba.write(&a);
    // ( Using Operator <<= )
    ba <<= &b;
    ba <<= &c;
    
    // Read data from ByteArray
    ba.seek_first();
    assert_eq!(a, ba.read::<f64>());                // 3.14
    assert_eq!(b, ba.read_safe::<u16>().unwrap());  // 1234
    assert_eq!(c, ba.read::<String>());             // "hello"
}
```

# Supported Data Types
|   Data Type   | Supported |
|--------------:|:---------:|
| bool          | Yes       |
| u8            | Yes       |
| u16           | Yes       |
| u32           | Yes       |
| u64           | Yes       |
| u128          | Yes       |
| i8            | Yes       |
| i16           | Yes       |
| i32           | Yes       |
| i64           | Yes       |
| i128          | Yes       |
| f32           | Yes       |
| f64           | Yes       |
|---------------|-----------|
| usize as u64  | Yes       |
| isize as i64  | Yes       |
|---------------|-----------|
| Vec<u8>       | Yes       |
| String        | Yes       |
| ByteArray     | Yes       |
| User-Defined  | Optional  |

# Documentation
[Docs.rs](https://docs.rs/byte_array)

# License
Distributed under MIT License since 2019.
