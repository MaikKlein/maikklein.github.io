+++
title= "Rlsl (Rust -> SPIR-V compiler) Progress report"
date        = "2018-02-09"
+++


# What is RLSL?

This is a [follow up post](https://maikklein.github.io/shading-language-part1/). RLSL is a Rust to SPIR-V compiler. SPIR-V is the shading language for Vulkan, similar to other shading languages like GLSL, HLSL but more low level. OpenGL, DX9/11/12, Vulkan, Metal are all graphic APIs that are able to use the GPU to draw pixels on the screen. Those APIs have certain stages that can be controlled by the developer by using the correct shading language.
![](https://i.imgur.com/jCuAYVl.png)

In this blog post, I will only cover the vertex and fragment shader.

Shaders are usually written in GLSL, HLSL and while those languages are relatively nice, they have some downsides. One downside is that you can not share code between files. Because of that game engines like Unity, Unreal Engine, CryEngine, Godot have developed they own shading langauge stack ontop of an existing shading language. This is especially important if you are targeting multiple grahpics backends, like OpenGL, DX. You don't want to rewrite your shaders every time.

In case of SPIR-V there already exists a [tool](https://github.com/KhronosGroup/SPIRV-Cross) for translating SPIR-V to GLSL/HLSL/MSL. SPIR-V is low level enough to be compiled from a higher level langauge like Rust.

# Why Rust as a shading language?

There are a few reasons:

* Libraries: Code can actually be shared
* Package manager: RLSL integrated with cargo and thefore libraries can also be managed by cargo.`RUSTC=rlsl cargo build` will compile the cargo project with rlsl.
* Generics/Traits: RLSL allows generics and traits everywhere excpet in entry points. I'll go into that later.
* Tooling supoport: Existing tools still work. (Racer, Rustfmt, Rls etc)
* Better shader compile times: A common approach for current shader langauges found in game engines is to copy/paste common shader code behind the scenes. In RLSL it is possible to create a big shader module that contains multiple shaders. *This property will be lost if the resulting SPIR-V is cross compiled to a different shading langauge*.
* Integrates with Rust: Structures defined in Rust can be shared between Rust and RLSL.
* Testing: RLSL is a subset of Rust. If your code compiles in RLSL, it will also compile in Rust. This means that you can run your shader code on the CPU.


# Showcase
{{ iframe(src="PersonalAdventurousInganue") }}
[Link to gif](https://gfycat.com/PersonalAdventurousInganue)



```Rust
#![feature(custom_attribute)]
extern crate rlsl_math;
use rlsl_math::{Vec4, Vertex};

#[spirv(fragment)]
fn frag(color: Vec4<f32>) -> Vec4<f32> {
    color
}

#[spirv(vertex)]
fn vertex(vertex: &mut Vertex, pos: Vec4<f32>, color: Vec4<f32>) -> Vec4<f32> {
    vertex.position = pos;
    color
}
```

I was relatively silent the last couple of months and I had very little time to work on RLSL, but today I reached my first milestone.
# Entry points

```Rust
#[spirv(vertex)]
fn vertex(vertex: &mut Vertex, pos: Vec4<f32>, color: Vec4<f32>) -> Vec4<f32> {
    vertex.position = pos;
    color
}
```


