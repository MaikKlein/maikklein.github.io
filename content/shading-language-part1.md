+++
date        = "2017-08-16"
title       = "RLSL- A new shading language"
tags        = [ "Rust", "Vulkan", "SPIR-V"]
+++

## Introduction
I always wanted to create a shading language but I never thought that I could actually do it. Today I reached a personal milestone, I can now compile a simple triangle shader into SPIR-V and use it with Vulkan. I have absolutely zero experience writing compilers and this is the first part in a hopefully long series where I talk about my progress of writing this shading language. In this blog post I will talk a bit about the language itself.



## Why a new language?

I think GLSL is actually a pretty decent language but it has a few problems. Code sharing might be the biggest problem in GLSL. It is not easy to share code between different shaders.

Imagine you have the following shader: (from [learnopengl.com](https://learnopengl.com/#!Advanced-OpenGL/Instancing))
```C
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;

out vec2 TexCoords;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
```

But you realize that you want to use instancing. This means that you have to change the interface.

```C
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in mat4 instanceMatrix;

out vec2 TexCoords;

uniform mat4 projection;
uniform mat4 view;
```

But what if you want to use both versions? Vulkan also introduces [subpasses](https://github.com/SaschaWillems/Vulkan/blob/master/data/shaders/subpasses/composition.frag#L6) which also adds another keyword. Then you might also want to support many different backends such as Metal, OpenGL 2/3/4/ES2/ES3, DX9/11/12, Vulkan all with similar but different shading languages, which means that you end up with even more duplicated shader code. Many major game engines and frameworks also have their own tooling to automate this process, but nothing is really usable outside those engine/frameworks. There are currently a few tools that are able to compile SPIR-V to languages like MSL, GLSL, HLSL. I intend to target each language individually but I currently focus in SPIR-V.

GLSL comes with a very tiny but useful "standard library". Because GLSL doesn't really allow code sharing, we often end up reimplementing the same code over and over again. Wouldn't it be nice to write and share libraries? I imagine a place like https://crates.io/ where you can share libraries like physically based shading, common BRDFs, Quaternions etc.

GLSL also only allows one entry point which means that you need separate shader modules. For example, imagine that you have two similar vertex shaders and both shaders use some library. There is no concept of dynamic linking which means that you end up recompiling the same code over and over again. I should probably mention that there are a couple of workarounds like subroutines or dynamic branching. Because a SPIR-V module can have multiple entry points, the whole shader module only has to be compiled once. (There are a few restrictions but this is outside of the scope of this blog post). Although this might be something that GLSL could introduce in the future. (At least for SPIR-V).

SPIR-V is easy to parse and you can extract information such as inputs, output, uniforms for a specific entry point. In my renderer when I create a graphics pipeline, I already know which shaders I can accept. This means that I can check at runtime if the shader is actually comaptible with my pipeline. I also allow shaders to be reloaded at runtime. Because of that one goal for the compiler is performance, I don't want to wait long for a shader build. I currently have only a SPIR-V backend, that translates
your code 1:1 into SPIR-V without any optimizations. In the future I also intend to support other backends such as LLVM. [spirv-llvm](https://github.com/KhronosGroup/SPIRV-LLVM) can target SPIR-V, but I don't think it currently supports Vulkan.

Shaders can be compiled offline into SPIR-V, but I could also imagine translating them into SPIR-V at runtime. For example you might not want to ship with every permutation of shaders for every backend. The alterantive would be to compile the shaders in user space. One benefit would be less space and more freedom. You could let the user decide which algorithms, lighting equations he wants to use and then invoke the shader compiler to compile efficient shaders without runtime overhead. This means
that the compiler needs to expose a good API. At this point I should probably mention that I'll also expose a C interface. A major goal is that this language is accessible from any programming language.

Shaders should be testable. I would like to write tests for a few pure functions and run the shader on the CPU. My first thought was to just have an LLVM backend and then run the shader on the CPU. The problem is that this only makes sense if all backends are actually correct and produce similar code. But because Vulkan uses SPIR-V, it might be useful to write a VM that can execute SPIR-V directly on the CPU. Of course this only makes sense if the VM is actually correct.

## TL;DR

I am creating a new shading language that

* allows code to be modular
* comes with a package manager to share libraries
* compiles to many different shading languages (currently SPIR-V only)
* runs on the CPU
* is testable
* builds quickly
* exposes the compiler as a library
* is useable from many different programming languages (not just Rust)


## Why not Rust?
I personally think that Rust is a fantastic language and it would probably be much easier to create a SPIR-V backend for Rust, than to create a compiler from scratch.
There are a few reason why I chose to write a compiler from scratch:

- I can easily design the language to my needs, which might be more difficult with Rust.
- Many concepts in Rust are not useful for shading languages. For example, probably all types are `Copy`, which means lifetimes and ownership semantics might not be that useful. Not using Rust might result in an easier language.

Currently the compiler is my personal playground that I use for experimentation. In the end I might still create a pure Rust backend.

## How does it look?
```C
// GLSL
layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 color;


layout (location = 0) out vec4 o_color;
void main() {
    o_color = color;
    gl_Position = pos;
}
```

```C
// GLSL
layout (location = 0) in vec4 o_color;
layout (location = 0) out vec4 uFragColor;

void main() {
    uFragColor = o_color;
}
```
And the equalivalent RLSL code. **R**ust **L**ike **S**hading **L**anguage, I know I am terrible at naming.
```Rust
#[spirv(Vec<f32, 4>)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

#[builtin(position)]
builtin position: Vec4;

vertex triangle_vs(pos: Input<Vec4>, color: Input<Vec4>) -> Vec4 {
    position = pos;
    color
}

fragment triangle_fs(color: Input<Vec4>) -> Vec4{
    color
}
```
You might ask your self why I chose a Rust like syntax. The reason is that it is the syntax that I am most familar with and I actually like it, but it not set in stone. Actually everything that you see here is very likely to change.

```Rust
#[spirv(Vec<f32, 4>)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}
```

This is just how I currently map custom types to a specific SPIR-V type. In the future this will be part of the standard library.

```Rust
#[builtin(position)]
builtin position: Vec4;
```
SPIR-V is still very similar to GLSL and this is equivalent to `gl_Position`. This will also be part of the standard library.


```Rust
vertex triangle_vs(pos: Input<Vec4>, color: Input<Vec4>) -> Vec4 {
    position = pos;
    color
}
```

This is where it gets interesting. `vertex` indicates that it is an entry point for a vertex shader. `pos: Input<Vec4>` is equivalent to `in vec4 pos`. `-> Vec4` indicates the output of the 
entry point. The last statement inside an entry point (without `;`) is the output. I also auto generate explicit layout indices for inputs and outputs. 


```Rust
fragment triangle_fs(color: Input<Vec4>) -> Vec4{
    color
}
```

Similar to the vertex shader, this defines the fragment shader. Interestingly, SPIR-V allows multiple entry points per shader module.

```
}

; SPIR-V
; Version: 1.1
; Generator: Google rspirv; 0
; Bound: 22
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %triangle_vs "triangle_vs" %pos %color %output
               OpEntryPoint Fragment %triangle_fs "triangle_fs" %color_0 %output_0
               OpName %position "position"
               OpName %pos "pos"
               OpName %color "color"
               OpName %output "output"
               OpName %triangle_vs "triangle_vs"
               OpName %color_0 "color"
               OpName %output_0 "output"
               OpName %triangle_fs "triangle_fs"
               OpDecorate %position BuiltIn Position
               OpDecorate %pos Location 0
               OpDecorate %color Location 1
               OpDecorate %output Location 0
               OpDecorate %color_0 Location 0
               OpDecorate %output_0 Location 0
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %position = OpVariable %_ptr_Output_v4float Output
%_ptr_Input_v4float = OpTypePointer Input %v4float
        %pos = OpVariable %_ptr_Input_v4float Input
      %color = OpVariable %_ptr_Input_v4float Input
     %output = OpVariable %_ptr_Output_v4float Output
       %void = OpTypeVoid
         %12 = OpTypeFunction %void
    %color_0 = OpVariable %_ptr_Input_v4float Input
   %output_0 = OpVariable %_ptr_Output_v4float Output
%triangle_vs = OpFunction %void None %12
         %14 = OpLabel
         %15 = OpLoad %v4float %pos
               OpStore %position %15
         %16 = OpLoad %v4float %color
               OpStore %output %16
               OpReturn
               OpFunctionEnd
%triangle_fs = OpFunction %void None %12
         %20 = OpLabel
         %21 = OpLoad %v4float %color_0
               OpStore %output_0 %21
               OpReturn
               OpFunctionEnd
 ```
 
![img](https://camo.githubusercontent.com/fcf8368d94842046bcb1856ecd69386109ce672f/687474703a2f2f692e696d6775722e636f6d2f50515a634c36772e6a7067)

And it even runs on RADV, although it throws a few warnings.


> amdgpu_device_initialize: Cannot parse ASIC IDs, 0xffffffea.WARNING: radv is not a conformant vulkan implementation, testing use only. ../../../../../src/amd/vulkan/radv_pipeline.c:189: FINISHME: Multiple shaders per module not really supported

## Roadmap

* Implement uniforms and instancing
* Add field access and methods
* Add mut to variables
* Add references
* Syntax for descriptor sets and pipelines
* Modules / libraries
* Add custom operators
* Allow custom structs as inputs in the vertex shader
* Write the standard library
* Rewrite the parser and expose better error messages
* Refactor the compiler
* ...
* Generics? Maybe with traits?


## What is next

In the next blog post, I'll talk about "interesting" problems that I have encountered.

## How can you help?

I currently haven't open sourced the compiler yet. First the code quality at this stage is not very good. The other reason is that I might use this project as my bachelor's thesis and I don't think that I can accept contributions yet. I'll keep you updated.

I would like to hear your thoughts. What features do you want to see?

