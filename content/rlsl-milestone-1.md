+++
title= "RLSL - Milestone 1"
date        = 2018-02-26
+++


## Shadertoy default shader

{{ iframe(src="UnrulyNeglectedBedbug") }}



```Rust
#[spirv(fragment)]
fn color_frag(
    frag: Fragment,
    uv: Input<N0, Vec2<f32>>,
    time: Descriptor<N2, N0, f32>,
) -> Output<N0, Vec4<f32>> {
    let uv = uv.data;
    let time = time.data;
    let offset = Vec3::new(0.0, 2.0, 4.0);
    let coord = uv.extend(uv.y)
        .add(offset)
        .map(move |f| f32::cos(time + f) * 0.5)
        .add(Vec3::single(0.5))
        .extend(1.0);
    Output::new(coord)
}

#[spirv(vertex)]
fn vertex(
    vertex: &mut Vertex,
    pos: Input<N0, Vec4<f32>>,
    uv: Input<N1, Vec2<f32>>,
) -> Output<N0, Vec2<f32>> {
    vertex.position = pos.data;
    Output::new(uv.data)
}
```

Today I reached the first milestone: translating the default shader from Shadertoy to rlsl.

## Input variables
```Rust
uv: Input<N0, Vec2<f32>>
```

Every entry point now specifies input and uniform buffers explicitly. `Input<N0, Vec2<f32>>` tells rlsl that there is an input variable at location 0 of type `Vec2<f32>`. The `N0` here is a custom struct that maps to the number `0` inside rlsl. When Rust gets `const-generics`, you will be able to write `Input<0, Vec2<f32>`.

## Uniform buffers and descriptor sets

```Rust
time: Descriptor<N2, N0, f32>,
```

Similarly to input variables, you specify descriptor sets with `Descriptor<N2, N0, f32>`, where currently the first type parameter is the binding and the second type parameter is the set. Also rlsl now supports the `std140` layout, and custom structs as the type parameter are also supported `Descriptor<N0, N0, SomeStruct>`. In the future I will also implement seamless interop of uniforms structs between rlsl and rust. Currently you have to make sure that the struct in rust is aligned properly according to the rules in std140.

## Closures and pointers

rlsl now supports closures, and closures that own the captured variables. In the future I will also allow closures to capture variables by reference. Additionally rlsl now allows pointers as variables and function parameters. While this is not allowed in SPIR-V, rlsl optimizes those pointers away.


## Build times


Build times are still quick, although the examples are still very small in terms of loc.

```Rust
➜  shader git:(master) time env RUST_BACKTRACE=1 RUSTC=rlsl cargo build
   Compiling shader v0.1.0 (file:///home/maik/projects/rlsl/quad/shader)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11 secs
0.19user 0.05system 0:00.25elapsed 94%CPU (0avgtext+0avgdata 77456maxresident)k
0inputs+96outputs (0major+14998minor)pagefaults 0swaps
```


