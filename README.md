libralloc - skeleton for implementing `malloc`/`free` in Rust
=============================================================

To hack on, first [install rust +
cargo](https://doc.rust-lang.org/book/getting-started.html).  Then:

```
$ git clone https://github.com/bpowers/libralloc
$ cd libralloc
$ cargo build
```

That should show you some warnings about unused variables, but
otherwise compile.  [src/lib.rs](src/lib.rs) exports the usual C
allocation and free functions, and you can hook these up to your
allocator here.

After building, you can manually test binaries on Linux with:

```
$ LD_PRELOAD=target/debug/libralloc.so ls
```
