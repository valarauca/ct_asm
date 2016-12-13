#Consistent Time Assembly
---

This is crate born out of the frustration of fighting with modern optimizing compilers. They're very smart. This is not a bad thing.
But for some operations stupidity is a good thing.

####Usage

This crate requires `nightly rust`. It used inline `asm!` macro extensively. 

This create will only work on `x86_64` platforms. There is no `arm`, `aarch64`, or `mips` support. There maybe in the future

####Interfaces

```rust

//return currrent number of CPU cycles since startup, or last roll over
tick() -> u64; 

//returns if two u64's are equal in constant time
ct_equal_u64(u64,u64) -> bool;

//returns if two u8's are equal in constant time
ct_equal_u8(u8,u8) -> bool; 

//Check if two slices are equal
//Panics if the lenghts are non-equal
ct_slice_eq( &[u8], &[u8]) -> bool;

//Optionally return one of two values
ct_optional_swap_u64(bool,u64,u64) -> u64;
```
