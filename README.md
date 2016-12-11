#Consistent Time Assembly
---

This is crate born out of the frustration of fighting with modern optimizing compilers. They're very smart. This is not a bad thing.
But for some operations stupidity is a good thing.

####Usage

This crate requires `nightly rust`. It used inline `asm!` macro extensively. 

This create will only work on `x86_64` platforms. There is no `arm`, `aarch64`, or `mips` support. There maybe in the future
