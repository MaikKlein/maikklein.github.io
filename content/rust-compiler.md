+++
date        = "2017-08-27"
title       = "Tooling support while using Rust as a library"
tags        = [ "Rust"]
+++

I recently started a SPIR-V compiler from scratch but I also wanted to see if I can actually build the compiler with the Rust compiler as a library. One possibility is to fork the Rust compiler and to add a binary crate like [this](https://github.com/msiglreith/rust/tree/althaea). In this case it just adds another binary module in the rustc crate. The problem that I had with this approach is that the turn around time not that great as a simple change results in rustc being rebuilt which takes at least 6min. Yesterday I tried to extend the Rust build system to produce another simple binary, but I failed. The build system is largely undocumented (I think). My first thought was to see how `rls` and `clippy` use the compiler.

It turns out they just do 


```Rust
#![feature(rustc_private)]
extern crate rustc;
fn main() {
    ...
}
```

The problem is that you don't get any tooling support if you use rustc like this, but I came up with a simple workaround which you can find [here](https://github.com/MaikKlein/rlsl/commit/54b84576a5ee9b0eafaba0612a50fd336011ab0c).

I just add the Rust compiler as a submodule, and I add the library that I am going to use 

```Toml
[target.'cfg(NOT_A_PLATFORM)'.dependencies]
rustc = {path = "compiler/rust/src/librustc"}
```

This way Rust will not try to compile `librustc` but tools like `intellij-rust` can still access the metadata.

![](https://i.imgur.com/EVk1xv7.gif)

