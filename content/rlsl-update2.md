+++
title= "RLSL - Progress report 2"
date        = 2019-03-07
+++

# Current status

Right now rlsl is in a prototyping stage. It was important to me to implement all the necessary features to see if Rust would be viable as a shading language. And I can tell you right now that I didn't run into any major problems. Right now the code base is a bit messy, but there are a few things that I need to do before I can clean everything up.

# Testing

I can not refactor the compiler without any test cases. Without tests I wouldn't know when I would run into some regressions. Testing is relatively simple with a little bit of macro magic.


```Rust
rlsl_test!{
    pub fn questionmark_option(_: u32, val: f32) -> f32 {
        fn test(f: f32) -> Option<f32> {
            let o = if f > 42.0 { Some(f) } else { None };
            let r = o?;
            Some(r + 10.0)
        }
        if let Some(val) = test(val) {
            val
        } else {
            -1.0
        }
    }
    pub fn break_loop(_: u32, val: f32) -> f32 {
        let mut sum = 0.0;
        let mut i = 0u32;
        while i > 100 {
            sum += 1.0;
            i += 1;
            if i < 50 {
                break;
            }
        }
        sum
    }
}

```

Essentially what happens is that rlsl creates a new tmp cargo project. Every test function compiles into its own spirv module. The function is then executed on the on the cpu with the help of `quickcheck`. Then we run a compute shader with the correct SPIR-V module and compare the results. If any of the results differ we assert.

# Debugging

I usually run into one of 3 issues:

* `spirv-val` is a validator for SPIR-V. This is usually run before everything, if it fails it will report where and what failed. It is a very valuable tool but unfortunately it is not complete.

* Segfault: If the SPIR-V module passes the validator it can still segfault if we try to create a shader module/pipeline in Vulkan. Debugging a segfault is currently fairly manual. I just try to recreate the segfault with the smallest possible test case that I can think of. In the future I want to integrate something like `spirv-reducer`.

* Loads but is incorrect: Sometimes the incorrect SPIR-V module still loads but it calculates the wrong values. Those errors are usually caught by the test suite.


It is quite hard to read complex SPIR-V code because it contains `GOTO`s, so I wrote a very small tool that I call [rspirv-cfg](https://github.com/MaikKlein/rspirv-cfg)
![](https://camo.githubusercontent.com/f85c1dd5fe5dfbbc0673fe75bf2622cf2e19acb8/68747470733a2f2f692e696d6775722e636f6d2f44484a467833382e706e67)

Essentially I display everything in a graph. Rust has a similar tool for MIR. I also slightly customized the textural representation, for example it will always include the `id`.

I usually spend most of my time reading faulty SPIR-V and this tool helps me to track down all the bugs.


# Reconstructing structured control flow

SPIR-V requires structured control flow. Essentially this means that if you have a conditional branch, you need to tell SPIR-V where it will merge again. Sadly this is a little bit more complicated than I initially thought.

The main problem is that the spec is a little bit underspecified, which means I quite often run into some undocumented edge cases.

Additionally it often is not possible at all to reconstruct structured control flow because the MIR (CFG) already violates a lot of the rules that are specified by SPIR-V. For example the conditional branch has to dominate the merge nodes, and that is not always guaranteed. 

One of those transformations looks like this:

Before:
![before](https://camo.githubusercontent.com/e7e9a2c7ffb321c9f5e61576cfcfc429c1bd8ef4/68747470733a2f2f692e696d6775722e636f6d2f416f34583543572e706e67)
After:
![after](https://camo.githubusercontent.com/519ea78121a4f3162a25c05cdafd3c86340de68a/68747470733a2f2f692e696d6775722e636f6d2f6d434f44786f682e706e67)
This is one reason why I want to focus on many test cases right now. I just don't have enough information to implement something that is 100% correct. Instead I try to find new test cases that will produce invalid SPIR-V and then I'll address those issues one by one.


# Transitioning torwards SPIR-V

Right now all of my transformations happen directly inside MIR. 
* Optimizing pointers away
* Every merge node needs to be unique
* Every selection construct needs to dominate its merge node
* ...

It doesn't really matter if I transform the MIR and then generate correct SPIR-V, or if I transform the SPIR-V afterwards.

Now I am toying with the idea of moving all of my transforms from MIR to SPIR-V.

Other projects could reuse this library to reconstruct all the control flow. This would make it much easier to generate SPIR-V in the first place.

The question that remains is if I should contribute this to [spirv-opt](https://github.com/KhronosGroup/SPIRV-Tools#optimizer) or if I should create a new library based on [rspirv](https://github.com/google/rspirv).

# Why generate SPIR-V in first place?

LLVM: No tools exist that generate vulkan compatible SPIR-V. There now exists a [fork](https://github.com/karolherbst/SPIRV-LLVM-Translator/commit/93c051c1d4a35d0ef178c34e6432f5f5ba094b5e) that structurizes the CFG. LLVM provides a rich eco system to transform the CFG.

DXIL: Very similar to LLVM, but sadly doesn't compile to SPIR-V. Afaik only HLSL can be translated to SPIR-V directly.

Cranelift: Can now be generated from MIR, I could have created a SPIR-V backend for it. Cranelift didn't do any optimizations when I started rlsl and so I didn't really consider using it over LLVM.

SPIR-V: Very easy to generate but misses a lot of optimizations if generated directly from MIR. Optimizations could still be possible with the bi-directional spir-v <-> llvm translator. `Rust -> MIR -> SPIR-V -> LLVM -> SPIR-V`. Additionally there is also [spirv-opt](https://github.com/KhronosGroup/SPIRV-Tools#optimizer-tool). The real challenge is to structurize the CFG, which I had to do anyways.

I ended up generating SPIR-V directly as I felt this was the easiest way to get started. MIR can be accessed with `#![feature(rustc_private)]`, and [rspirv](https://github.com/google/rspirv) already existed at the time.

# Playground

I am currently working on a shadertoy like framework for rlsl. Everything is still in flux but essentially you will be able to run a fragment shader on the cpu and gpu.


For example the following image was rendered both on the CPU and GPU.
{{twitter(id="1071852847669612545")}}

It is quite useful to discover invalid SPIR-V code. For example I recently tried to port the raycaster to a raytracer, unfortunately I ran into some visual artifacts. It is quite hard to know if I made a logic error or if the SPIR-V is incorrect. Because I can run everything on the CPU, I can easily verify the results myself.

This will also be an excellent place to learn and experiment with rlsl in the future.

# Integrating rlsl into an existing project

First of all please don't wait on rlsl. If you need to write shaders right now, use GLSL/HLSL etc. The first release of rlsl will focus on outputting SPIR-V. This means it won't do any magic behind your back, you specify every inputs / outputs explicitly and it will generate SPIR-V just as GLSL does.

rlsl should allow you to build more sophisticated tools on top of it at a later stage. Once rlsl has matured, you can start to replace each shader bit by bit.

Right now the current workflow looks like this:

You create a new crate `foo-shader`. You write all the shader code inside this crate. Every `bin` target will create a new spirv module. For example


```

foo-shader
  Cargo.toml
  src
    lib.rs
    bin
      foo.rs
      bar.rs
```

`RUSTC=rlsl cargo build` will create two new SPIR-V modules called `foo.spv` and `bar.spv`. You define all your entry points *(vertex, fragment, compute)* inside those `bin` targets.

You can even include external libraries that contain code that can not be translated to SPIR-V. This works as long as you only use functions and types that can be translated to SPIR-V.

For example consider:

```Rust
#[derive(Debug)]
pub struct Ray{..}
```

SPIR-V does not have the concept of strings and debug just doesn't make any sense in SPIR-V. But it does make sense on the CPU, it would be unfortunate if you could not use `Ray` in rlsl.




# Interfacing with SPIR-V

rlsl will not depend on a specific math library, although it will ship with an optional one called `rlsl_math`. For example it is possible to mark types with a custom attribute.

```Rust
#[spirv(Vec4)]
pub struct Vec4<T> {..}
```

This tells rlsl that this type should be an `OpTypeVector`. Porting existing math libraries like `cgmath` or `nalgebra` will be up the community. Up to date information can always be found in `rlsl_math`.

I could also imagine some 'database' that you define inside your library 
```Rust
spirv_types! {
    math::Vec4 => spirv::Vec4,
};

spirv_fns! {
    math::Vec4::dot => spirv::dot,
    math::Vec4::cross => spirv::cross,
};
```

# When will it be ready?

Everything is still in flux but my current goal is to remove all my custom compiler changes and focus on a specific nightly version. Installing should be as simple as `cargo install rlsl`. Using it is as simple as `RUSTC=rlsl cargo build`. rlsl will most likely require rustup.

A lot of things are not implemented yet. For example you will probably run into a lot of panics, many operators are still not implemented. Additionally I haven't implemented any custom error messages, which means features like recursion won't trigger an error message.

If you want to help out, the best thing you can do is to experiment with rlsl and report any issues on the [issue tracker](https://github.com/MaikKlein/rlsl/issues). Especially helpful would be additional test cases. I'll add all the necessary guides on how to do that once rlsl is installable.

After I have set up a custom CI, I will start to accept PRs.

[Project page](https://github.com/MaikKlein/rlsl)

If you have any questions don't hesitate to contact me.
