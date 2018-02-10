+++
title= "RLSL (Rust -> SPIR-V compiler) Progress report"
date        = "2018-02-09"
+++


# Disclaimer
The code for RLSL can be found [here](https://github.com/MaikKlein/rlsl). It is **not** ready to be used at all, the build tools haven't been written yet. I encourage you **not** to build it. Also the code is still in a relatively bad shape.

# What is RLSL?

This is a [follow up post](https://maikklein.github.io/shading-language-part1/). RLSL is a Rust to SPIR-V compiler. SPIR-V is the shading language for Vulkan, similar to other shading languages like GLSL, HLSL but more low level. OpenGL, DX9/11/12, Vulkan, Metal are all graphic APIs that are able to use the GPU to draw pixels on the screen. Those APIs have certain stages that can be controlled by the developer by using the correct shading language.
![](https://i.imgur.com/jCuAYVl.png)

In this blog post, I will only cover the vertex and fragment shader.

Shaders are usually written in GLSL, HLSL and while those languages are relatively nice, they have some downsides. One downside is that you can not share code between files. Because of that game engines like Unity, Unreal Engine, CryEngine, Godot have developed their own shading language stack on top of an existing shading language. This is especially important if you are targeting multiple graphics backends, like OpenGL, DX.

In case of SPIR-V there already exists a [tool](https://github.com/KhronosGroup/SPIRV-Cross) for translating SPIR-V to GLSL/HLSL/MSL. SPIR-V is low level enough to be compiled from a higher level language like Rust.

RLSL should make it much easier to get started with shader development.

# Why Rust as a shading language?

There are a few reasons:

* Libraries: Code can actually be shared
* Package manager: RLSL integrates with cargo and therefor libraries can also be managed by cargo.`RUSTC=rlsl cargo build` will compile the cargo project with RLSL.
* Generics/Traits: RLSL allows generics and traits everywhere except in entry points. I'll go into that later.
* Tooling support: Existing tools still work. (Racer, Rustfmt, Rls etc)
* Possibly better shader compile times: A common approach for current shader languages found in game engines is to copy/paste common shader code behind the scenes. In SPIR-V it is possible to create a big shader module that contains multiple shaders. *This property will be lost if the resulting SPIR-V is cross compiled to a different shading langauge*.
* Integrates with Rust: Structures defined in Rust can be shared between Rust and RLSL, but RLSL should still be usable in other languages.
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

I recompile the RLSL code after every change, which produces a new `.spv` file. This is a modified version of the triangle example from [ash](https://github.com/MaikKlein/ash). Essentially I just recreate the graphics pipeline if the `.spv` file changes.

And this is how the `.spv` file looks in text form.
```C
; SPIR-V
; Version: 1.2
; Generator: Google rspirv; 0
; Bound: 29
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %2 "frag" %9 %12
               OpEntryPoint Vertex %5 "vertex" %9 %10 %12
               OpExecutionMode %2 OriginUpperLeft
               OpExecutionMode %5 OriginUpperLeft
               OpMemberName %Vertex 0 "position"
               OpMemberName %Vertex 1 "point_size"
               OpName %Vertex "Vertex"
               OpDecorate %9 Location 0
               OpDecorate %10 Location 1
               OpDecorate %12 Location 0
               OpMemberDecorate %Vertex 0 BuiltIn Position
               OpMemberDecorate %Vertex 1 BuiltIn PointSize
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Input_v4float = OpTypePointer Input %v4float
          %9 = OpVariable %_ptr_Input_v4float Input
         %10 = OpVariable %_ptr_Input_v4float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
         %12 = OpVariable %_ptr_Output_v4float Output
       %void = OpTypeVoid
         %14 = OpTypeFunction %void
     %Vertex = OpTypeStruct %v4float %float
%_ptr_Output_Vertex = OpTypePointer Output %Vertex
         %20 = OpVariable %_ptr_Output_Vertex Output
%_ptr_Function_v4float = OpTypePointer Function %v4float
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
          %2 = OpFunction %void None %14
         %15 = OpLabel
               OpBranch %16
         %16 = OpLabel
         %17 = OpLoad %v4float %9
               OpStore %12 %17
               OpReturn
               OpFunctionEnd
          %5 = OpFunction %void None %14
         %21 = OpLabel
               OpBranch %22
         %22 = OpLabel
         %23 = OpLoad %v4float %9
         %27 = OpAccessChain %_ptr_Output_v4float %20 %uint_0
               OpStore %27 %23
         %28 = OpLoad %v4float %10
               OpStore %12 %28
               OpReturn
               OpFunctionEnd

```
# Entry points

```Rust
#[spirv(vertex)]
fn vertex(vertex: &mut Vertex, pos: Vec4<f32>, color: Vec4<f32>) -> Vec4<f32> {
    vertex.position = pos;
    color
}
```

```Rust
vertex: &mut Vertex
```
This is the first argument in a vertex shader. This gives you mutable access to various PerVertex variables like  `gl_Position`, `gl_Pointsize` etc.

```Rust
pos: Vec4<f32>, color: Vec4<f32>
```
The next two arguments are the input to the vertex shader. In the future I will also allow user defined structs. In Vulkan you can only pass SPIR-V primitives into the vertex shader. I will work around this problem by 'unrolling' the type into primitive types.

```Rust
struct Input{
    pos: Vec4<f32>,
    uv: Vec4<f32>,
}
```
will be unrolled into two variables `pos: Vec4<f32>, color: Vec4<f32>`.

```Rust
-> Vec4<f32>
```
The return type is the output of the vertex shader. Currently in RLSL you can only output one type, but this can be a composition of types.

This is the resulting GLSL of the cross compiled `.spv` file with `spirv-cross`. *RLSL currently doesn't generate debug information for global variables*

```C
// vertex shader
#version 450

layout(location = 0) in vec4 _9;
layout(location = 1) in vec4 _10;
layout(location = 0) out vec4 _12;

void main()
{
    gl_Position = _9;
    _12 = _10;
}
```

```C
// fragment shader
#version 450

layout(location = 0) in vec4 _8;
layout(location = 0) out vec4 _10;

void main()
{
    _10 = _8;
}
```
Entry points are not allowed to contain any generics, but functions inside the body are allowed to be generic.


```Rust
fn some_generic_fn<V1: Vector, V2: Vector>(vertex: &mut Vertex,
                                           pos: V1,
                                           color: V2) -> Vec4<f32>{
    ...
}

#[spirv(vertex)]
fn vertex(vertex: &mut Vertex, pos: Vec4<f32>, color: Vec4<f32>) -> Vec4<f32> {
    some_generic_fn(vertex, pos, color)
}

#[spirv(vertex)]
fn vertex2(vertex: &mut Vertex, pos: Vec2<f32>, color: Vec4<f32>) -> Vec4<f32> {
    some_generic_fn(vertex, pos, color)
}
```

It should be easy to create multiple entry points that share the same logic. This is especially useful for instancing where usually only the entry point changes.

Entry points are roughly equivalent to the main function in Rust. The difference is that there can be multiple entry points, and that there are different types of entry points like the vertex and fragment shader.

Currently I just look at every function that has the `#[spirv(vertex)]` or `#[spirv(fragment)]` attribute and I include it in the `.spv` file. In the future I want entry points to be defined outside of the `main.rs` file and I also need a way to specify which entry points should be included in the `.spv` file.

Also you might have noticed that I don't explicitly specify the location of the input / output variables. This is all done behind the scene. This information is inside the `.spv` file, but I will also make this information available in an easier to parse format and I will also create the necessary library to retrieve this information.

Some pseudo API

```Rust
let reflection: Reflection = parse("reflection.toml");
let data: &EntryPoint = reflection.get(some_entry_point).expect("Entry point not found");
data.path() // path to .spv file
data.inputs() // Input information
data.output() // Output information
data.constants() // Constant information
reflection.entry_point_iter() // iterator over every entry point
...
```

If necessary I might also introduce an API to set this information explicitly.

# RLSL is a subset of Rust

RLSL is a subset of Rust. If it compiles in RLSL it should compile in Rust, but not the other way around. SPIR-V has many limitations like  no pointers inside structs. This means a lot of iterators won't work in RLSL.

One interesting design decision is that currently no shader primitives are inside the std library. RLSL ships with the std but only with a reduced version. All the necessary types and functions are defined in an external library called `rlsl_math`.

```Rust
#[spirv(Vec4)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
```

This is currently how a `Vec4<T>` is defined in `rlsl_math`.The `#[spirv(Vec4)]` attribute tells RLSL that this should map to the builtin `vec4` type in SPIR-V. Remember that `rlsl_math` is an external library, that means other math libraries like cgmath or nalgebra should be able to replace `rlsl_math` (maybe not completely). I will probably split common functionality into a separate crate.

# Optimizations

Currently RLSL takes the following steps. `Rust -> HIR -> MIR -> SPIR-V`. There are a few optimizations implemented for MIR, but a lot of things are missing. [spirv-opt](https://github.com/KhronosGroup/SPIRV-Tools#optimizer) can be used to further optimize the `.spv` file.

I am still hoping for a `SPIR-V <-> LLIR` compiler for better optimizations. If nothing pops up within a year or two, I'll probably start one on my own. I don't expect this to be much harder than a `MIR -> SPIR-V` compiler.

`SPIR-V` is an intermediary format, which means it gets compiled by the driver. Considering the fact that most GPU vendors are competitors, I would expect at least some optimizations.

Currently the SPIR-V produced by RLSL is comparable to the SPIR-V produced by [glslang](https://github.com/KhronosGroup/glslang)

# Compilation
In the future you will be able to install RLSL with `cargo install rlsl`. To compile a project you just have to replace rustc with rlsl `RUSTC=rlsl cargo build`. Currently RLSL is completely unoptimized, but the hello triangle example compiles in 0.11s. At the moment RLSL is completely single threaded but that will soon change.

Although I don't expect compile times to be an issue, considering that the amount of shader code should not be that high.

To give you some estimation of the amount of shader code in a commercial engine (Unreal Engine 4)
```Markdown
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 ...
 C++                 13503      5973298      4361179       586000      1026119
 C++ Header           3294       707450       513044        95156        99250
 Usf                   186        41230        27758         5396         8076
 ...
-------------------------------------------------------------------------------
 Total               70491     21705869     15499494      3410894      2795481
-------------------------------------------------------------------------------
```
`Usf` is the shading language in Unreal Engine 4, which I think is based on HLSL.

Currently RLSL is still tied to LLVM. If RLSL compiles a library it essentially falls back to rustc which then produces an `.rlib` file. This file includes the MIR which will be used to generate the SPIR-V. I am still hoping for [gh38913](https://github.com/rust-lang/rust/issues/38913) but it should be easy to do this on my own.

# Future

I wish I could work on this project full time, but at the current rate I think that RLSL should at least be installable and semi usable for small projects by the end of the year. At the same time I will open up the project for contributions.

If you are interested in using RLSL in the future, I would love to know your requirements.
