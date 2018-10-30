+++
date        = 2017-11-29
title       = "Rust cpp"
tags        = [ "Rust"]
draft = true
+++



# Move semantics

## Typesafe statemachines

```Rust
use std::time::SystemTime;

#[derive(Debug)]
struct Unauthorized;

#[derive(Debug)]
struct Authorized {
    authorized_since: SystemTime,
}

#[derive(Debug)]
struct User<State> {
    name: String,
    state: State,
}

impl User<Unauthorized> {
    fn new<S: Into<String>>(name: S) -> Self {
        User {
            name: name.into(),
            state: Unauthorized,
        }
    }

    fn login(self, pwd: &str) -> Result<User<Authorized>, User<Unauthorized>> {
        Ok(User {
            name: self.name,
            state: Authorized {
                authorized_since: SystemTime::now(),
            },
        })
    }
}

impl User<Authorized> {
    fn logout(self) -> Result<User<Unauthorized>, User<Authorized>> {
        Ok(User {
            name: self.name,
            state: Unauthorized,
        })
    }
}

fn main() {
    // Only has the method `login`.
    let user = User::<Unauthorized>::new("Foo");
    // `login` consumes `user` and produces an User<Authorized>
    let auth_user = user.login("bar").expect("Unable to login");
    let unauth_user = auth_user.logout().expect("Unable to logout");
}
```
[Playground](https://play.rust-lang.org/?gist=244366be4b367528769e26c6ce7a850a&version=stable)

# Mutability

## Variables
```Rust
let mut i = 42;
i = 24;
```
## Methods


```Rust
struct Foo {
    n: u32,
}

impl Foo {
    fn mutate(&mut self, n: u32) {
        self.n = n;
    }
}

fn main() {}

```
[Playground](https://play.rust-lang.org/?gist=9c069bab752ec4432899ba360641be1b&version=stable)

You can only mutate fields with `&mut self`. If you do not want mutability do borrow with `&self`.

## Borrows (References) inside structs

```Rust
struct Foo<'a, T:'a> {
    t: &'a T
}
```
Foo contains an immutable borrow to T (`&T`). To borrow mutabily you can simply borrow mutably with `t: &'a mut T`. But what if you want to abstract over mutability?

```Rust
struct Foo<T> {
    t: T,
}

impl<'a, T> Foo<&'a T> {
    fn foo(&self) -> &T{
        self.t
    }
}

impl<'a, T> Foo<&'a mut T> {
    fn foo_mut(&mut self) -> &mut T {
        self.t
    }
}

fn main() {}
```
[Playground](https://play.rust-lang.org/?gist=a458e8fa0f7c78dfe9004717c7876f8c&version=stable)

## Abstracting over mutability
As far as I know there is no good way to generally abstract over mutability at the moment.

```Rust
struct Foo<T> {
    t: T,
}

impl<T> Foo<T> {
    fn get(&self) -> &T {
        &self.t
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.t
    }
}

fn main() {}
```
[Playground](https://play.rust-lang.org/?gist=198c84897915ab009bcc361e97e22217&version=stable)

Mutability often leads to code duplication, but in simple cases like these it is not as bad.


```Rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Chunks<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            None
        } else {
            let chunksz = cmp::min(self.v.len(), self.size);
            let (fst, snd) = self.v.split_at(chunksz);
            self.v = snd;
            Some(fst)
        }
    }
    ...
```
[Link](https://github.com/rust-lang/rust/blob/77ab3a1d5ff69c0cb3eb85a75ef734eaf7429f1b/src/libcore/slice/mod.rs#L2144)

```Rust
impl<'a, T> Iterator for ChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.v.is_empty() {
            None
        } else {
            let sz = cmp::min(self.v.len(), self.chunk_size);
            let tmp = mem::replace(&mut self.v, &mut []);
            let (head, tail) = tmp.split_at_mut(sz);
            self.v = tail;
            Some(head)
        }
    }
    ...
```
[Link](https://github.com/rust-lang/rust/blob/77ab3a1d5ff69c0cb3eb85a75ef734eaf7429f1b/src/libcore/slice/mod.rs#L2242)

Both implementations are almost identical, but are still implemented separately. Often this is the case because of methods that are named `get` or `get_mut`, `split` or `split_mut`.

```Rust
macro_rules! make_mir_visitor {
    ($visitor_trait_name:ident, $($mutability:ident)*) => {
        pub trait $visitor_trait_name<'tcx> {
            // Override these, and call `self.super_xxx` to revert back to the
            // default behavior.

            fn visit_mir(&mut self, mir: & $($mutability)* Mir<'tcx>) {
                self.super_mir(mir);
            }

            fn visit_basic_block_data(&mut self,
                                      block: BasicBlock,
                                      data: & $($mutability)* BasicBlockData<'tcx>) {
                self.super_basic_block_data(block, data);
            }
            ...
```
[Link](https://github.com/rust-lang/rust/blob/77ab3a1d5ff69c0cb3eb85a75ef734eaf7429f1b/src/librustc/mir/visit.rs#L80)
```Rust
make_mir_visitor!(Visitor,);
make_mir_visitor!(MutVisitor,mut);
```

Sometimes macros are used to avoid code duplication. In this case the macro will define two separate traits `Visitor` and `MutVisitor`.

## Mutation by ownership

```Rust
fn mutate_by_ownership(mut v: Vec<u32>) -> Vec<u32> {
    v.push(1);
    v
}

fn mutate_by_ownership_explicit(v: Vec<u32>) -> Vec<u32> {
    let mut v = v;
    v.push(1);
    v
}

fn main() {
    let v = vec![1, 2, 3];
    let v1 = mutate_by_ownership(v);
    // v is not accessible anymore
    let v2 = mutate_by_ownership_explicit(v1);
    // v1 is not accessible anymore
}
```
[Playground](https://play.rust-lang.org/?gist=407790d00c72d9166eeb6017c3584348&version=stable)

Variables can be redeclared as mutable.

# Traits

## Extend functionality

```Rust
trait Say {
    fn say(&self);
}

impl Say for u32 {
    fn say(&self) {
        println!("I am an u32 {}", *self);
    }
}

fn main() {
    1.say();
}
```
[Playground](https://play.rust-lang.org/?gist=5b2cbc9bdb945b0654b7da65f576a9c4&version=stable)
{{ playpen(path="extend-trait.rs") }}

This will implement a method `say()` on an `u32`.

## Implement traits for other traits

```Rust
use std::fmt::Debug;
trait PrintDebug {
    fn print_debug(&self);
}

impl<T: Debug> PrintDebug for T {
    fn print_debug(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    1.print_debug();
    "Hello".print_debug();
    vec![1, 2, 3, 4].print_debug();
}
```
[Playground](https://play.rust-lang.org/?gist=8b3857944524aaf929dbaed6694706b2&version=stable)


## Add functionality to generics with traits

```Rust
trait ToMeters {
    fn to_meters(self) -> Meters;
}

#[derive(Debug, Copy, Clone)]
struct Meters(f32);

impl ToMeters for Meters {
    fn to_meters(self) -> Meters {
        self
    }
}

#[derive(Debug, Copy, Clone)]
struct Centimeters(f32);

impl ToMeters for Centimeters {
    fn to_meters(self) -> Meters {
        Meters(self.0 / 100.0)
    }
}

fn print_meters<M: ToMeters>(m: M) {
    println!("{:?}", m.to_meters());
}

fn main() {
    let cm = Centimeters(250.0);
    let m = Meters(2.5);
    print_meters(cm); // Meters(2.5)
    print_meters(m); // Meters(2.5)
}
```
[Playground](https://play.rust-lang.org/?gist=589fc01f3aaffa960a839be2758d8942&version=stable)

Here `print_meters` accepts every type that implements `ToMeters`. This will automatically convert units like centimeters to meters, without user intervention. This can cause binary bloat. Imagine that you have many different unit types and every time you call `print_meters` it will monomorphize the function parameters to a concrete type like `fn print_meters(m: Centimeters){..}`. 

Now I imagine a `big_fn` that would be very slow to compile.
```Rust
fn big_fn<M: ToMeters>(m: ToMeters) {
    // crazy things are happening here
}
```

There are two problems with this approach:

1. If `big_fn` is inside a library it will be compiled in user code, because no monomorphized version exists. This adds compile time for the user.

2. Compile times might go up because `big_fn` might be generated multiple times, for every type that the user uses.

The work around is to monomorphize the function explicitly.

```Rust
trait ToMeters {
    fn to_meters(self) -> Meters;
}

#[derive(Debug, Copy, Clone)]
struct Meters(f32);

impl ToMeters for Meters {
    fn to_meters(self) -> Meters {
        self
    }
}

#[derive(Debug, Copy, Clone)]
struct Centimeters(f32);

impl ToMeters for Centimeters {
    fn to_meters(self) -> Meters {
        Meters(self.0 / 100.0)
    }
}

fn big_fn<M: ToMeters>(m: M) {
    big_fn_impl(m.to_meters());
}

fn big_fn_impl(m: Meters) {
    // every thing is happening here
}

fn main() {
    let cm = Centimeters(250.0);
    let m = Meters(2.5);
    big_fn(cm);
}

```

Everything happens inside `big_fn_impl` and it only has a concrete type as a parameter. This means that `big_fn_impl` will be compiled only once inside the library, and only the calls to `big_fn` will be generated in user code.

This can drastically reduce compile times in certain cases, but it shifts work to the programmer. 

*Note*: Incremental compilation should be able to reduce build times without user intervention.

## Static polymorhism / static dispatch

[Ast based visitor in C#](https://en.wikipedia.org/wiki/Visitor_pattern#Dynamic_Visitor)

```Rust
struct Literal(pub f64);

struct Addition<L: Expr, R: Expr> {
    pub l: L,
    pub r: R,
}

impl<L, R> Addition<L, R>
where
    L: Expr,
    R: Expr,
{
    pub fn new(l: L, r: R) -> Self {
        Addition { l, r }
    }
}

impl<L, R> Expr for Addition<L, R>
where
    L: Expr,
    R: Expr,
{
    fn accept<Visitor: ExprVisitor>(&self, v: &Visitor) {
        v.visit_addition(self);
    }
}

impl Expr for Literal {
    fn accept<Visitor: ExprVisitor>(&self, v: &Visitor) {
        v.visit_literal(self);
    }
}

struct ExprPrinter;
impl ExprVisitor for ExprPrinter {
    fn visit_literal(&self, lit: &Literal) {
        print!("{}", lit.0);
    }
    fn visit_addition<L: Expr, R: Expr>(&self, add: &Addition<L, R>) {
        print!("(");
        add.l.accept(self);
        print!("+");
        add.r.accept(self);
        print!(")");
    }
}

fn main() {
    let printer = ExprPrinter {};
    let e = Addition::new(Addition::new(Literal(1.0), Literal(2.0)), Literal(3.0));
    e.accept(&printer);
}
```
[Playground](https://play.rust-lang.org/?gist=3a337ef9d5b450362f48c828860f2f18&version=stable)

I translated the C# version straight into Rust and therefor is not completely idomatic Rust. 

The biggest difference compared to the C# version is that this is a compile time visitor. It might not be the best example because an AST is usually not known at compile time but I thought that most people would be familiar with some common design patterns like the visitor pattern.

To reiterate on the meaning of 'compile time' in this context: The AST has to be constructed at compile time and `ExprPrinter` is essentially a zero cost abstraction. And with zero cost abstraction I mean that the code above compiles down to a single print statement.

You can have a look at the generated assembly at [godbolt](https://godbolt.org/g/nR55r3).

The same concept is used for `Iterators`, `Futures` etc.

## Returning traits
```Rust
#![feature(conservative_impl_trait)]
use std::iter::Iterator;

fn some_iter() -> impl Iterator<Item = u32> {
    (1..).map(|i| i).filter(|i| i % 2 == 0)
}

fn main() {}
```
[Playground](https://play.rust-lang.org/?gist=772bb5eba0f1a0a69d76a95d32f1c144&version=nightly)

Static dispatch is very straight forward in Rust but it can be hard to name those types explicitly. For example the type for the iterator above would roughly be `Filter<Map<RangeFrom<u32>, fn(u32) -> u32>, fn(&u32) -> bool>`. Instead of naming the return type explicitly, the `impl trait` can be used `impl Iterator<Item = u32>`. This is not yet available on stable.

## Dynamic dispatch
```Rust
pub trait Calculate {
    fn calculate(&self, u32) -> u32;
}

pub struct Square;
pub struct Factorial;

impl Calculate for Square {
    fn calculate(&self, n: u32) -> u32 {
        n * n
    }
}

impl Calculate for Factorial {
    fn calculate(&self, n: u32) -> u32 {
        (1..).take(n as usize).product()
    }
}

pub fn useless_sum<'a>(v: &[Box<Calculate>]) -> u32 {
    (0..5)
        .flat_map(|n| v.iter().map(move |calc| calc.calculate(n)))
        .sum()
}

fn main() {
    let v: Vec<Box<Calculate>> = vec![Box::new(Square), Box::new(Factorial)];
    let sum = useless_sum(&v);
    println!("{}", sum);
}
```
[Playground](https://play.rust-lang.org/?gist=1abee467a3f3f53b379154edabf4f195&version=stable) / [Godbolt](https://godbolt.org/g/TLsxnQ)

Dynamic dispatch can be achieved with traits but it causes the compiler to miss a few optimizations. An alternative to dynamic dispatch with traits is to use enums.

```Rust
pub trait Calculate {
    fn calculate(&self, u32) -> u32;
}

pub struct Square;
pub struct Factorial;

impl Calculate for Square {
    fn calculate(&self, n: u32) -> u32 {
        n * n
    }
}

impl Calculate for Factorial {
    fn calculate(&self, n: u32) -> u32 {
        (1..).take(n as usize).product()
    }
}

pub enum Calc {
    Square(Square),
    Factorial(Factorial),
}

impl Calc {
    pub fn calculate(&self, n: u32) -> u32 {
        use Calc::*;
        match *self {
            Square(ref sq) => sq.calculate(n),
            Factorial(ref f) => f.calculate(n),
        }
    }
}

pub fn useless_sum(v: &[Calc]) -> u32 {
    (0..5)
        .flat_map(|n| v.iter().map(move |calc| calc.calculate(n)))
        .sum()
}

fn main() {
    let v = vec![Calc::Square(Square), Calc::Factorial(Factorial)];
    let sum = useless_sum(&v);
    println!("{}", sum);
}
```

[Playground](https://play.rust-lang.org/?gist=cbd305f4432e34250e25420222ed2bca&version=stable) / [Godbolt](https://godbolt.org/g/D3ubyc)

Enums are transparent to the compiler and therefor might result in more performant code. The downside is that the variants have to be known.


## Generic arrays and Deref

```Rust
use std::ops::{Deref, DerefMut};

pub trait Unsigned<T>: Copy {
    type Array;
}

#[derive(Copy, Clone)]
pub struct U1;
#[derive(Copy, Clone)]
pub struct U2;
#[derive(Copy, Clone)]
pub struct U3;
//...

impl<T> Unsigned<T> for U1 {
    type Array = [T; 1];
}

impl<T> Unsigned<T> for U2 {
    type Array = [T; 2];
}

impl<T> Unsigned<T> for U3 {
    type Array = [T; 3];
}
//...

#[derive(Copy, Clone, Debug)]
pub struct VecN<T, U: Unsigned<T>> {
    data: U::Array,
}

impl<T, U> VecN<T, U>
where
    U: Unsigned<T>,
{
    pub fn from_array(data: U::Array) -> Self {
        VecN { data }
    }
}

impl<T> VecN<T, U3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        VecN { data: [x, y, z] }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Deref for VecN<T, U3> {
    type Target = CVec3<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for VecN<T, U3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

pub type Vec3f = VecN<f32, U3>;

fn main() {
    let v = Vec3f::from_array([1.0, 2.0, 3.0]);
    let mut v = Vec3f::new(1.0, 2.0, 3.0);
    let v1 = v;
    // Field access comes from Deref<Target = StaticVec3<T>>
    let x = v.x;
    // Mutable access through DerefMut
    v.x = 42.0;
}
```
[Playground](https://play.rust-lang.org/?gist=b841f0ce73be1bcbb639765f43a08200&version=stable)

At the time of writing this blog post, the [const generics rfc](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) hasn't yet made it to nightly yet. This introduces a hacky workaround to have const generics for arrays on stable. 

Because those vectors now have an array as their member, field access through `.x` `.y` `.z` is not possible anymore. The obvious solution would be to introduces methods such as `fn x(&self) -> &T` and `fn x(&mut self)-> &mut T`.

The alternative is to define a new struct `CVec3` and then to implement `Deref` and `DerefMut`.

## Methods can silently override trait methods

```Rust
pub trait Foo {
    fn hello(&self);
}

pub mod some_module {
    pub struct Bar;
}

use some_module::Bar;

impl Foo for Bar {
    fn hello(&self) {
        println!("Hello Foo");
    }
}

fn main() {
    let b = Bar;
    b.hello(); // prints `Hello Foo`
}

```
[Playground](https://play.rust-lang.org/?gist=3fc3587c23dffd4b809ec69b85e83537&version=stable)

If `Bar` also adds a method called `hello` in the future, then this method will be used instead of the trait method.

```Rust
pub trait Foo {
    fn hello(&self);
}

pub mod some_module {
    pub struct Bar;
    impl Bar {
        pub fn hello(&self) {
            println!("Hello Bar");
        }
    }
}

use some_module::Bar;

impl Foo for Bar {
    fn hello(&self) {
        println!("Hello Foo");
    }
}

fn main() {
    let b = Bar;
    b.hello(); // prints `Hello Bar`
}
```
[Playground](https://play.rust-lang.org/?gist=58c188c3db6d66873fdd8a09570cf013&version=stable)

This is not a problem in generic code 

```Rust
fn foo<F: Foo>(f: F){...}
```


## Adding methods to a trait can be a breaking change

```Rust
pub trait Foo {
    fn hello(&self);
}

pub struct Bar;

pub mod some_module {
    pub trait Baz {}
}

use some_module::Baz;

impl Foo for Bar {
    fn hello(&self) {
        println!("Hello Foo");
    }
}

impl Baz for Bar {}

fn main() {
    let b = Bar;
    b.hello();
}

```
[Playground](https://play.rust-lang.org/?gist=11b2e422805940971a6321dded55dce0&version=stable)

If `Baz` also exposes a `hello` method, then those two methods will be in conflict.
```Rust
pub trait Foo {
    fn hello(&self);
}

pub struct Bar;

pub mod some_module {
    pub trait Baz {
        fn hello(&self){
            // Err
        }
    }
}

use some_module::Baz;

impl Foo for Bar {
    fn hello(&self) {
        println!("Hello Foo");
    }
}

impl Baz for Bar {}

fn main() {
    let b = Bar;
    b.hello();
}
```
[Playground](https://play.rust-lang.org/?gist=39cffc2b1d941119430258119a492ee2&version=stable)

```Rust
error[E0034]: multiple applicable items in scope
  --> src/main.rs:27:7
   |
27 |     b.hello();
   |       ^^^^^ multiple `hello` found
   |
note: candidate #1 is defined in an impl of the trait `Foo` for the type `Bar`
  --> src/main.rs:18:5
   |
18 | /     fn hello(&self) {
19 | |         println!("Hello Foo");
20 | |     }
   | |_____^
note: candidate #2 is defined in an impl of the trait `some_module::Baz` for the type `Bar`
  --> src/main.rs:9:9
   |
9  | /         fn hello(&self){
10 | |             // Err
11 | |         }
   | |_________^
```

Trait methods can be named explicitly to disambiguate the function calls.

```Rust
fn main() {
    let b = Bar;
    Foo::hello(&b);
}
```
# Lifetimes

## Structs

```Rust
#[derive(Copy, Clone)]
struct Foo<'a> {
    s: &'a str,
}

impl<'a> Foo<'a> {
    fn print_str(self) {
        print!("{}", self.s);
    }
}

fn main() {}
```
[Playground](https://play.rust-lang.org/?gist=c4a036efe34b240a6ad9674f042024a2&version=stable)

Borrows inside structs and impl blocks require an explicit lifetime. Sometimes this can make refactoring more cumbersome. Imagine that you start of with the following code.

```Rust
struct Foo {
    s: String,
}

impl Foo {
    fn print_str(&self) {
        print!("{}", self.s);
    }
}

fn main() {}
```
[Playground](https://play.rust-lang.org/?gist=d342f14442eb0e02c0d4ca449be023b4&version=stable)

But you realize that you really wanted to use a `&str` instead of a `String`. This means that you have to explicitly add lifetime annotations to every impl block and to types that can not infer the lifetime. [This RFC](https://github.com/rust-lang/rfcs/blob/master/text/2115-argument-lifetimes.md#impl-blocks-and-lifetimes) will be a improvement.

# Conventions

```Rust
struct snake_case;
impl snake_case {
    fn PascalCase(){}
}

fn main() {}
```

[Playground](https://play.rust-lang.org/?gist=8a4562198d490f89342766c5c0735ec2&version=stable)

Warnings:

```Rust
warning: type `snake_case` should have a camel case name such as `SnakeCase`
 --> src/main.rs:1:1
  |
1 | struct snake_case;
  | ^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(non_camel_case_types)] on by default

warning: method `PascalCase` should have a snake case name such as `pascal_case`
 --> src/main.rs:3:5
  |
3 |     fn PascalCase(){}
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(non_snake_case)] on by default

```

# Enum

## Fizzbuzz

```Rust
#[derive(Debug, Copy, Clone)]
pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl FizzBuzz {
    pub fn from_number(n: u32) -> FizzBuzz {
        use FizzBuzz::*;
        match (n % 3 == 0, n % 5 == 0) {
            (true, false) => Fizz,
            (false, true) => Buzz,
            (true, true) => FizzBuzz,
            _ => Number(n),
        }
    }
}

fn main() {
    (1..100)
        .map(FizzBuzz::from_number)
        .for_each(|f| println!("{:?}", f));
}
```
[Playground](https://play.rust-lang.org/?gist=10fc46c8d1998e56746a9a24d802ad7b&version=stable)
{{ playpen(path="fizzbuzz.rs") }}

# Codegen
