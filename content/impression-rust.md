+++
date        = 2016-06-21
title       = "First impression of Rust after two years - Part 1"
tags        = [ "Rust"]
+++


# First impression of Rust after two years - Part 1

The last time I was using Rust was in version 0.6 - 0.7, which was roughly 2 years ago. I decided to come back to take another look.

I decided to write two small libraries over the weekend. An n-dimensional, generic and typesafe linear algebra library and a task pool implementation with fibers based on naughty dogs [GDC talk](http://www.gdcvault.com/play/1022186/Parallelizing-the-Naughty-Dog-Engine).

But I came from D where I could easily create a nice vector library with templates.
```D
alias Vec3f = Vector<float, 3>;
```

I kinda wanted to do the same thing, but the problem is that Rust doesn't have type level integers. This is probably why cgmath and nalgebra create their types manually and implement functionality with macros. Luckily I found [typenum](https://github.com/paholg/typenum), which is sort of hack to emulate type level integers.

Before I go on, I want to demonstrate some ergonomics of my experimental vector library. The vector types are defined like this:

```Rust
pub type Vec4<T> = Vector<T, U4>;
pub type Vec3<T> = Vector<T, U3>;
pub type Vec2<T> = Vector<T, U2>;

pub type Vec4f = Vec4<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;
```

Vectors can be created like this

```Rust
let v1 = Vec2f::new(&[0.0, 2.0]);
let v2 = Vec2f::new(&[0.0, 10.0]);
```

You can also create a vector with a lower dimensional vector + additional value like this:

```Rust
let v = Vec3f::from_one_less(Vec2f::new(&[1.0, 2.0]), 3.0);
assert!(v == Vec3f::new(&[1.0, 2.0, 3.0]));
```

It is not as ergonomic as my vector library in D where you can arbitrarily initialize a new vector but it is a start. You may wonder how I implemented it. The type signature looks like this

```Rust
// I left out the type constrains for readability
pub fn from_one_less<>(first: Vector<T, Sub1<N>>, val: T) -> Vector<T, N>{...}
```
It breaks down for a `Vec2` because then it would want a `Vec1` + a value, which would be a verbose way of writing just two values.

I don't think that I can restrict implementations for specific `typenum` traits. Also it would have also been nice to have one generic function that can accept arbitrary vectors, but that would require at least type level integers + variadic function, which Rust doesn't have.

Because I used `typenum` to express n-dimensional vectors, it made it very easy to implement functions like dot

```Rust
pub fn dot(self, other: Self) -> T {
    self.data
        .into_iter()
        .zip(other.data.into_iter())
        .fold(T::zero(), |acc, (x, y)| acc + x * y)
}
```
I really like that Rust has pattern matching on tuples, I think this really adds to readability.

Now is probably a good time to show the initial definition of a `Vector`.

```Rust
#[derive(PartialEq, Eq, Copy, Debug)]
pub struct Vector<T: Float, N: ArrayLength<T>>
    where N::ArrayType: Copy
{
    pub data: GenericArray<T, N>,
}
```

I implemented copy on this type because this library is intended for game devs which means that the dimensions will probably not bigger than 4. I also don't think I can specialize structs at the moment to allow bigger vectors to be heap allocated. I also auto implemented PartialEq and Eq because I am lazy. Comparison should probably be implemented with `abs(a - b) < eps`.


Implementing operators was a bit more verbose in Rust compared to D. I used macros to make it less verbose.


```Rust
macro_rules! as_expr { ($e:expr) => {$e} }
macro_rules! impl_op_vec{
    ($trait_name: ident, $fn_name: ident, $op: tt) => {
        impl<T, N> $trait_name for Vector<T, N>
            where N::ArrayType: Copy,
                  N: ArrayLength<T>,
                  T: Float
        {
            type Output = Vector<T, N>;
            fn $fn_name(self, other: Self) -> Self::Output {
                unsafe {
                    let mut new_data: GenericArray<T, N> = mem::uninitialized();
                    let iter = self.data
                        .iter()
                        .zip(other.data.iter())
                        .map(|(a, b)| as_expr!( *a $op *b));
                    for (index, val) in iter.enumerate() {
                        new_data[index] = val;
                    }
                    Vector::<T, N> { data: new_data }
                }
            }
        }
    }
}

impl_op_vec!(Sub, sub, -);
impl_op_vec!(Add, add, +);
impl_op_vec!(Mul, mul, *);
impl_op_vec!(Div, div, /);

macro_rules! impl_op_vec_un{
    ($trait_name: ident, $fn_name: ident, $op: tt) => {
        impl<T, N> $trait_name<T> for Vector<T, N>
            where N::ArrayType: Copy,
                  N: ArrayLength<T>,
                  T: Float
        {
            type Output = Vector<T, N>;
            fn $fn_name(self, other: T) -> Self::Output {
                unsafe {
                    let mut new_data: GenericArray<T, N> = mem::uninitialized();
                    let iter = self.data
                        .iter()
                        .map(|a| as_expr!( *a $op other));
                    for (index, val) in iter.enumerate() {
                        new_data[index] = val;
                    }
                    Vector::<T, N> { data: new_data }
                }
            }
        }
    }
}

impl_op_vec_un!(Mul, mul, *);
impl_op_vec_un!(Add, add, +);
impl_op_vec_un!(Sub, sub, -);
impl_op_vec_un!(Div, div, /);
```

I am not a Rust programmer so this macro is probably not as nice as it should be, but basically all I am doing is a simple text substitution. The only hiccup that I encountered was the substitution for the `operator`. For some reason I had to create a helper macro `as_expr`. Not sure why this was necessary, but I assume that this might be a bug?

In general I am not sure I like macros in Rust, because macros can accept almost arbitrary syntax. I think this makes macros very powerful but also painful for other people to use. As a client you probably always have to either read the documentation or look at the macro implementation yourself, so that you know how to invoke the macro correctly.

Also you might notice that I am using unsafe here. I basically do this almost anywhere because of `mem::unitialzed()`. The reason I am doing this is that I don't think that it is possible to collect into a fixed length array / GenericArray from an iterator.

The documentation engine is pretty nice in Rust, for example you can write
```Rust
/// Builds a `Vector<T, N >` from a `Vector<T, N-1>` with an additional value.
/// # Example
/// ```
/// use rla::vector::*;
/// let v = Vec3f::from_one_less(Vec2f::new(&[1.0, 2.0]), 3.0);
/// assert!(v == Vec3f::new(&[1.0, 2.0, 3.0]));
/// ```
pub fn from_one_less(first: Vector<T, Sub1<N>>, val: T) -> Vector<T, N>{..}
```

and it will parse the comments as markdown. It also recognizes codeblocks, compiles and executes them.

I currently haven't implemented vector swizzling nor  individual member access like
```Rust
v.x = 25;
```

The problem is that I don't know how I would implement this in an ergonomic and clean way. For example in D you can have this

```Rust

auto x = v.x;
v.x = 24;
```

where `x` is a overloaded function `ref T x(){..}` and `void x(T val){}`, although I have implemented it a bit differently in D. I am posting the D implementation because I don't think a lot of Rust programmers really know how metaprogramming looks like.

[Implementation](https://github.com/BreezeEngine/breeze/blob/master/source/breeze/math/vector.d#L67+L86)

Feel free to skip this part if you are not interested in D.
```D
struct Vector(T, size_t _dimension){
    private enum vectorCords = "xyzw";
    ...
    ref auto opDispatch(string op)() inout
    if(op.length is 1){
        import std.string: indexOf;
        import std.algorithm.iteration: map;
        enum index = vectorCords.indexOf(op);
        return data[index];
    }
    auto opDispatch(string op)() const
    if(op.length > 1 && op.length <= dimension){
        import std.string: indexOf;
        import std.algorithm.iteration: map;
        import std.range: array;
        import std.algorithm.mutation: copy;
        import std.algorithm.searching: count;
        static immutable indices = op.map!(c => vectorCords.indexOf(c)).array;
        static assert(indices[].count(-1) == 0, "Combination of " ~op~" does not exist.");
        T[op.length] _data;
        indices.map!(i => data[i]).copy(_data[]);
        return Vector!(T,op.length)(_data);
    }
    ...
}
```

For example `v.x` here returns a value but `v.xy` returns a Vec2. It is also possible to do `v.xyzzyzyy` which would return a Vec8 but it is probably not really practical.

The code works like this:
`private enum vectorCords = "xyzw";` is a compile time string. opDispatch receives the string behind the dot for an object at compile time. For example calling `v.xyz`, opDispatch would receive the string `xyz` at compile time. You then map this string at compile time to the index from `vectorCords`. For this example it would map `"xyz"` to `[0, 1, 2]` at compile time. Then you simply look up the values and return the vector. The Vector will have the dimension of the length of the received
string which in this case is `xyz`, so it will return a Vector3.

Let us get back to Rust. Implementing this will be probably a bit harder. I will probably create 4 traits with a macro. `x` , `xy`, `xyz` and `xyzw`. `xy` will inherit from `x`, `xyz` from `xy` an so on. I will then create default implementations with a macro for all possible combinations. And then I will implement the traits manually for Vector2 - Vector4. I really wish that Rust would get a bit more metaprogramming support in the future.

Matrices are implemented in a similar fashion.
```Rust
type Mat4x4<T> = Matrix<T, U4, U4>;
type Mat3x3<T> = Matrix<T, U3, U3>;
type Mat3x2<T> = Matrix<T, U3, U2>;
type Mat2x3<T> = Matrix<T, U2, U3>;
type Mat2x2<T> = Matrix<T, U2, U2>;

type Mat3x2f = Mat3x2<f32>;
type Mat2x3f = Mat2x3<f32>;
type Mat2x2f = Mat2x2<f32>;
```
But implementing matrices was much harder than a simple vector. For example let us implement matrix multiplication. In case you are not familiar it looks like this
[Matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication)
```
N, M, N1, M1 > 0
N == N1
Matrix<N, M> * Matrix<N1, M1> = Matrix<M, N1>
```

```Rust
impl<T, N, M> Matrix<T, N, M>
    where ...
{
    fn mul<N1>(&self, other: &Matrix<T, N1, N>) -> Matrix<T, M, N1>{...}
}
```
Without type constrains this looks rather elegant. We implement `mul` only on matrices that can be multiplied together, and then we return the correct matrix.

My only gripe with this approach is that I don't think it is possible to return a user defined error.

For example a client might try to multiply `Matrix<f32, 3, 2> * Matrix<f32, 3, 2>` which is not possible. It would be nice to output a custom error message to the user, something like this

```Rust
Error: Tried to multiply Matrix<f32, 3, 2>,  Matrix<f32, 3, 2>, but the dimesions of
Matrix<f32, 3, 2>,  Matrix<f32, 3, 2>  don't match.
            ^`````````````````````~^
```

This is possible in D but I don't think something like this can currently be implemented in Rust. Another occurrence would be `identity`.

An [identity matrix](https://en.wikipedia.org/wiki/Identity_matrix) can only implemented on a matrix of type `Matrix<T, N, N>`.
```Rust
impl<T, N> Matrix<T, N, N>
    where ...
{
    fn identity() -> Matrix<T, N, N> {
        ...
    }
}
```

It then would be nice to also output a custom error message if the user wants to call `identity` on a non-square matrix.

If we look at this code again, we see that I have left out the type constrains. I did this on purpose because they look quite hilariously verbose.

```Rust
impl<T, N, M> Matrix<T, N, M>
{
    fn mul<N1>(&self, other: &Matrix<T, N1, N>) -> Matrix<T, M, N1>{...}
}
```

But it was a good exercise to see how good Rust's error messages really are.

```Rust
// funny stuff is happening here
impl<T, N, M> Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T> + ArrayLength<Vector<T, M>>,
          M: ArrayLength<T> + ArrayLength<Vector<T, N>>,
          <N as ArrayLength<Vector<T, M>>>::ArrayType: Copy,
          <M as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
          <N as ArrayLength<T>>::ArrayType: Copy,
          <M as ArrayLength<T>>::ArrayType: Copy
{
    fn mul<N1>(&self, other: &Matrix<T, N1, N>) -> Matrix<T, M, N1>
        where N1: ArrayLength<T> + ArrayLength<Vector<T, M>> + ArrayLength<Vector<T, N>>,
              <N1 as ArrayLength<T>>::ArrayType: Copy,
              <N1 as ArrayLength<Vector<T, M>>>::ArrayType: Copy,
              <N1 as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
              N: ArrayLength<Vector<T, N1>>,
              <N as ArrayLength<Vector<T, N1>>>::ArrayType: Copy,
              Vector<T, N>: Copy
    {
        unsafe {
            let mut new_matrix: Matrix<T, M, N1> = mem::uninitialized();
            let other_transposed: Matrix<T, N, N1> = other.transpose();
            for j in 0..N1::to_usize() {
                for i in 0..M::to_usize() {
                    new_matrix.data[j].data[i] = self.data[j].dot(other_transposed.data[i]);
                }
            }
            new_matrix
        }
    }
}
```
Obviously this looks horrendous and I am not even sure I had to write it like that. It was really annoying to repeatedly specify the copy constrain.

The only good part was that it was super trivial to write. I did 0 thinking about any constrains, I just looked at the error messages and copy pasted the constrain. I did this until Rust stopped complaining. I was really surprised that how easy this was and it would probably not be too unreasonable to think that the constrains could possibly be generated with some external tool.

My experience with cargo and crates.io was flawless so far. I didn't run into any issue at all. I really like how intuitive it was to get up and running. Publishing a crate was also pretty simple, you can find it [here](https://crates.io/crates/rla). Please note that the library is completely experimental and will probably never be finished.

It was also possible to add a .git repository inside a cargo.toml, which I had to do with my task pool library. More on that in part 2.

Also cargo allows you to directly install binaries into .cargo. For example you can install cargo watch like this `cargo install cargo-watch`.

This concludes Part 1, in Part 2 I will continue with my task pool library.
