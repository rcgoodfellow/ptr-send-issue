# Pointer Send in Rust 2018 vs 2021

This is
[MRE](https://stackoverflow.com/help/minimal-reproducible-example?_ga=2.166032573.203023859.1640655461-2041605478.1640655461)
code that uses an opaque pointer from `dlopen`. It compiles and works 
in Rust 2018 on Illumos, Linux and MacOS. It does not compile in Rust 2021.

I'm not seeing anything that jumps out in the 
[2021 overview](https://blog.rust-lang.org/2021/05/11/edition-2021.html).

```
error[E0277]: `*mut c_void` cannot be sent between threads safely
   --> src/main.rs:32:14
    |
32  |       let j0 = spawn(move || {
    |  ______________^^^^^_-
    | |              |
    | |              `*mut c_void` cannot be sent between threads safely
33  | |         // find the read function
34  | |         let name = CString::new("read").unwrap();
35  | |         let fp = unsafe{ dlsym(h0.0, name.as_c_str().as_ptr()) };
...   |
41  | |         println!("blueberry: {:?}", fp);
42  | |     });
    | |_____- within this `[closure@src/main.rs:32:20: 42:6]`
    |
    = help: within `[closure@src/main.rs:32:20: 42:6]`, the trait `Send` is not implemented for `*mut c_void`
    = note: required because it appears within the type `[closure@src/main.rs:32:20: 42:6]`
note: required by a bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
```

```
rustup show
Default host: x86_64-unknown-illumos
rustup home:  /home/ry/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-illumos (default)
nightly-2021-11-24-x86_64-unknown-illumos
nightly-2021-12-03-x86_64-unknown-illumos
nightly-x86_64-unknown-illumos

active toolchain
----------------

stable-x86_64-unknown-illumos (default)
rustc 1.57.0 (f1edd0429 2021-11-29)
```
