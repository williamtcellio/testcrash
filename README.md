# testcrash
Reproduces a crash in Rust's test framework on stable Rust 16.0 on Mac OSX   
```cargo test```  
Will sometimes result in   
``error: process didn't exit successfully: `/Users/william/projects/repros/testcrash/target/debug/deps/testcrash-051b006336e89f31` (signal: 11, SIGSEGV: invalid memory reference)``
