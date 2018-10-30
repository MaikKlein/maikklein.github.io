+++
date        = 2016-06-23
title       = "Comparison between Rust and D"
tags        = [ "Rust"]
+++

I will try to compare `Rust` and `D` as objectively as possible. I will mostly focus the language parts that can actually be compared.

*Rust currently is at version 1.9 and D is at version 2.071.0.*


### Move and copy semantics
Rust moves by default or copies if the `Copy` trait is implemented.

```Rust
fn test(v: Vec<i32>){
}
fn main() {
    let v = vec![1, 2, 3];
    test(v); // v has moved into test
    // v can not be used
}
```

Explicit `copies` are done in Rust with the `Clone` trait.


```Rust
fn test(v: Vec<i32>){
}
fn main() {
    let v = vec![1, 2, 3];
    test(v.clone()); // the cloned vec has moved into test
    // v can still be used here
}
```

D does a shallow copy by default

```D
import std.container: Array;

void test(Array!int arr){
    arr.insert(4);
}
void main()
{
    auto arr = Array!int(1, 2, 3);
    test(arr); // only does a shallow copy
    // Array contains now elements 1, 2, 3, 4
}
```

Explicit copying has to be implemented manually. `Array` exposes a `dup` method that can be used to duplicate an array.

```D
import std.container: Array;

void test(Array!int arr){
    arr.insert(4);
}
void main()
{
    auto arr = Array!int(1, 2, 3);
    test(arr.dup);
    //arr still has 3 elements
}
```

Moves have to be done explicitly with `move`.

```D
import std.container: Array;
import std.algorithm.mutation: move;

void test(Array!int arr){
    arr.insert(4);
}
void main()
{
    auto arr = Array!int(1, 2, 3);
    test(arr.move);
    // arr can still be used but is empty
}
```

Moved values in D can still be used because they are reset to their default state.

It is also possible to reject lvalues for specific functions.

```D
import std.container: Array;
import std.algorithm.mutation: move;

@disable void test(ref Array!int arr);
void test(Array!int arr){
    arr.insert(4);
}
void main()
{
    auto arr = Array!int(1, 2, 3);
    // test(arr); does not accept an lvalue
    test(arr.move); // still works
}
```

### Mutability

Types in Rust are immutable by default.
```Rust
fn main() {
    let v = vec![1, 2, 3];
    //v.push(1); // does not work
    let mut v1 = vec![1, 2, 3];
    v1.push(1);
}
```
D requires the `immutable` keyword.
```D
void main()
{
    immutable i = 42;
}
```
D does not allow interior mutability as far as I know, which means it is not possible to create an immutable `Array`. Interior mutability in Rust can be implemented with [Cell](https://doc.rust-lang.org/std/cell/).

D has another keyword `const` to restrict exterior mutability.

```
void main()
{
   const arr = Array!int(1, 2, 3);
   //arr.insert(1); does not compile
}
```

From the [D spec](https://dlang.org/spec/const3.html#const_and_immutable)

>immutable applies to data that cannot change. Immutable data values, once constructed, remain the same for the duration of the program's execution. Immutable data can be placed in ROM (Read Only Memory) or in memory pages marked by the hardware as read only. Since immutable data does not change, it enables many opportunities for program optimization, and has applications in functional style programming.

<!-- -->
>const applies to data that cannot be changed by the const reference to that data. It may, however, be changed by another reference to that same data. Const finds applications in passing data through interfaces that promise not to modify them.

<!-- -->
>Both immutable and const are transitive, which means that any data reachable through an immutable reference is also immutable, and likewise for const.

### Struct initialization

D as well as Rust can field initialize structs.

```D
struct Foo{
     int i;
}

void main()
{
    Foo f = {i: 42};
}
```


``` Rust
struct Foo{
    i: i32
}
fn main() {
    let f = Foo{i: 42};
}
```

D also has constructors

```D
struct Foo{
     int i;
     this(int _i){
         i = _i;
     }
}

void main()
{
    auto f = Foo(42);
}
```

while Rust uses functions to do object construction.

```Rust
struct Foo{
    i: i32
}
impl Foo{
    fn new(i: i32) -> Foo{
        Foo{i: i}
    }
}
fn main() {
    let f = Foo::new(42);
}
```

### Methods and UFCS

Methods in D are implement inside the type.

```D
struct Foo{
     int i;
     this(int _i){
         i = _i;
     }

     void print(){
         import std.stdio: writeln;
         writeln("Foo ", i);
     }
}

void main()
{
    auto f = Foo(42);
    f.print();
}
```
Rust implements methods outside of the type and inside a `impl` block.
```Rust
struct Foo{
    i: i32
}
impl Foo{
    fn new(i: i32) -> Foo{
        Foo{i: i}
    }
    fn print(&self){
        println!("Foo {}", self.i);
    }
}
fn main() {
    let f = Foo::new(42);
    f.print();
}
```

Also Rust doesn't implicitly capture `this / self`.

Universal function call syntax allows to call print In Rust like this:

```Rust
fn main() {
    let f = Foo::new(42);
    Foo::print(&f);
}
```
In D functions can be called like methods
```D
struct Foo{
     int i;
     this(int _i){
         i = _i;
     }
}

void print(ref Foo f){
    import std.stdio: writeln;
    writeln("Foo ", f.i);
}

void main()
{
    auto f = Foo(42);
    // both works
    f.print();
    print(f);
}
```

The advantage of `impl` blocks is that they reduce verbosity for constrains.

```Rust
impl<T> Bar<T>
    where: T: Copy
{
    fn something(&self, val: T) -> T{...}
}
```
It is also possible to have multiple `impl` for different constrains. As far as I know this is not possible in D, every function needs its own constrains.

A small advantage of D is that it is possible to call those functions directly, like normal functions.
```D
auto zippedRange = zip(r1, r2);
auto zippedRange = r1.zip(r2);
```

While in Rust this will be a bit more explicit.

```Rust
let iter = Iterator::zip(iter1, iter2);
//or
let iter = iter1.zip(iter2);
```
### Compile times
Subjectively Rust compiles slower than D, but it is really hard to get some objective data. I will post the results of compiling racer, rustfmt, DCD and dfmt. I am aware that those numbers are not really representable but it is the best that I can do.

Also note that those a full release builds and not incremental debug builds.

```
➜  bin time cargo install rustfmt
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling winapi v0.2.7
   Compiling rustc-serialize v0.3.19
   Compiling bitflags v0.5.0
   Compiling utf8-ranges v0.1.3
   Compiling log v0.3.6
   Compiling getopts v0.2.14
   Compiling unicode-segmentation v0.1.2
   Compiling strings v0.0.1
   Compiling regex-syntax v0.3.3
   Compiling winapi-build v0.1.1
   Compiling unicode-xid v0.0.3
   Compiling kernel32-sys v0.2.2
   Compiling diff v0.1.9
   Compiling term v0.4.4
   Compiling term v0.2.14
   Compiling libc v0.2.12
   Compiling memchr v0.1.11
   Compiling aho-corasick v0.5.2
   Compiling thread-id v2.0.0
   Compiling thread_local v0.2.6
   Compiling regex v0.1.71
   Compiling syntex_syntax v0.32.0
   Compiling toml v0.1.30
   Compiling env_logger v0.3.3
   Compiling rustfmt v0.5.0
  Installing /home/maik/.cargo/bin/rustfmt
  Installing /home/maik/.cargo/bin/cargo-fmt
cargo install rustfmt  382.58s user 1.49s system 130% cpu 4:54.72 total
```

```
➜  bin time cargo install racer
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling unicode-xid v0.0.3
   Compiling ansi_term v0.7.4
   Compiling libc v0.2.12
   Compiling unicode-width v0.1.3
   Compiling rustc-serialize v0.3.19
   Compiling regex-syntax v0.3.3
   Compiling winapi-build v0.1.1
   Compiling winapi v0.2.7
   Compiling kernel32-sys v0.2.2
   Compiling strsim v0.4.1
   Compiling term v0.2.14
   Compiling thread-id v2.0.0
   Compiling typed-arena v1.1.0
   Compiling utf8-ranges v0.1.3
   Compiling bitflags v0.5.0
   Compiling vec_map v0.6.0
   Compiling clap v2.2.6
   Compiling log v0.3.6
   Compiling memchr v0.1.11
   Compiling aho-corasick v0.5.2
   Compiling thread_local v0.2.6
   Compiling regex v0.1.71
   Compiling toml v0.1.30
   Compiling syntex_syntax v0.32.0
   Compiling env_logger v0.3.3
   Compiling racer v1.2.10
  Installing /home/maik/.cargo/bin/racer
cargo install racer  495.08s user 2.05s system 143% cpu 5:47.45 total
```

```
➜  DCD git:(master) time make -j4
find: ‘containers/experimental_allocator/src/std/experimental/allocator/’: No such file or directory
git log -1 --format="%H" > githash.txt
mkdir -p bin
mkdir -p bin
dmd src/common/constants.d src/common/socket.d src/common/dcd_version.d src/common/messages.d src/server/autocomplete.d src/server/server.d dsymbol/src/dsymbol/import_.d dsymbol/src/dsymbol/symbol.d dsymbol/src/dsymbol/cache_entry.d dsymbol/src/dsymbol/string_interning.d dsymbol/src/dsymbol/semantic.d dsymbol/src/dsymbol/builtin/symbols.d dsymbol/src/dsymbol/builtin/names.d dsymbol/src/dsymbol/deferred.d dsymbol/src/dsymbol/modulecache.d dsymbol/src/dsymbol/scope_.d dsymbol/src/dsymbol/type_lookup.d dsymbol/src/dsymbol/conversion/first.d dsymbol/src/dsymbol/conversion/package.d dsymbol/src/dsymbol/conversion/second.d libdparse/src/dparse/ast.d libdparse/src/dparse/entities.d libdparse/src/dparse/lexer.d libdparse/src/dparse/parser.d libdparse/src/dparse/formatter.d libdparse/src/dparse/rollback_allocator.d libdparse/src/dparse/stack_buffer.d libdparse/src/std/experimental/lexer.d  containers/src/containers/dynamicarray.d containers/src/containers/ttree.d containers/src/containers/unrolledlist.d containers/src/containers/openhashset.d containers/src/containers/hashset.d containers/src/containers/internal/hash.d containers/src/containers/internal/node.d containers/src/containers/internal/storage_type.d containers/src/containers/internal/element_type.d containers/src/containers/internal/backwards.d containers/src/containers/slist.d msgpack-d/src/msgpack/exception.d msgpack-d/src/msgpack/attribute.d msgpack-d/src/msgpack/package.d msgpack-d/src/msgpack/register.d msgpack-d/src/msgpack/streaming_unpacker.d msgpack-d/src/msgpack/buffer.d msgpack-d/src/msgpack/common.d msgpack-d/src/msgpack/value.d msgpack-d/src/msgpack/unpacker.d msgpack-d/src/msgpack/packer.d -Icontainers/src -Imsgpack-d/src -Ilibdparse/src -Idsymbol/src -Icontainers/experimental_allocator/src -J. -wi -O -release -inline -ofbin/dcd-server
dmd src/common/constants.d src/common/socket.d src/common/dcd_version.d src/common/messages.d src/client/client.d msgpack-d/src/msgpack/exception.d msgpack-d/src/msgpack/attribute.d msgpack-d/src/msgpack/package.d msgpack-d/src/msgpack/register.d msgpack-d/src/msgpack/streaming_unpacker.d msgpack-d/src/msgpack/buffer.d msgpack-d/src/msgpack/common.d msgpack-d/src/msgpack/value.d msgpack-d/src/msgpack/unpacker.d msgpack-d/src/msgpack/packer.d -Imsgpack-d/src -Imsgpack-d/src -Icontainers/experimental_allocator/src -J. -inline -O -wi -ofbin/dcd-client
make -j4  42.62s user 2.44s system 104% cpu 43.173 total
```

```
➜  dfmt git:(master) ✗ time dub build -f -b release
Performing "release" build using dmd for x86_64.
experimental_allocator 2.70.0-b1: building configuration "library"...
libdparse 0.7.0-alpha9: building configuration "library"...
dfmt 0.5.0-beta3+commit.25.ge3bf269: building configuration "application"...
Linking...
dub build -f -b release  40.78s user 0.56s system 99% cpu 41.360 total
```

### Dynamic / static dispatch, constrains and extending functionality

In Rust it is possible to extend a type using traits.

```
struct Foo {
    i: i32,
}

impl Foo {
    fn new(i: i32) -> Foo {
        Foo { i: i }
    }
}

trait Print {
    fn print(&self);
}

impl Print for Foo {
    fn print(&self) {
        println!("Foo {}", self.i);
    }
}

impl Print for i32 {
    fn print(&self) {
        println!("i32 {}", self);
    }
}

fn main() {
    let f = Foo::new(42);
    f.print();
    Print::print(&f);

    42.print();
    Print::print(&42);
}
```

The same thing is possible in D with overloaded functions.

```
struct Foo{
     int i;
     this(int _i){
         i = _i;
     }
}

void print(ref Foo f){
    import std.stdio: writeln;
    writeln("Foo ", f.i);
}

void print(int i){
    import std.stdio: writeln;
    writeln("int ", i);
}

void main()
{
    auto f = Foo(42);
    f.print();
    print(f);

    42.print();
    print(42);
}
```

The advantage of traits is that you can use them to constrain types at compile and they allow dynamic dispatch

```
fn call_print_static<T: Print>(t: &T){
    t.print();
}

fn call_print_dynamic(p: &Print){
    p.print();
}

fn main() {
    let f = Foo::new(42);
    call_print_static(&f);
    call_print_static(&42);

    call_print_dynamic(&f as &Print);
    call_print_dynamic(&42 as &Print);
}
```
This can roughly be expressed in D like this: 
```
interface Print{
    void print();
}

class Foo: Print{
     int i;
     this(int _i){
         i = _i;
     }
     void print(){
         import std.stdio: writeln;
         writeln("Foo ", i);
     }
}

void call_print_dynamic(Print p){
    p.print();
}

void call_print_static(T)(T t){
    t.print();
}

void main()
{
    //f is a pointer
    auto f = new Foo(42);
    call_print_dynamic(f);
    call_print_static(f);
}
```

Dynamic dispatch with interfaces can only really be used with classes. Classes are by default references types which means if there is a type `Foo` it implicitly is type `Foo*`. While it is possible allocate classes on the heap / stack or some memory region, they inherently have an indirection.

Also `call_print_static` is duck typed, which means that the function expects every `T` to be callable with `.print`. This can easily result in ugly error messages.

D doesn't really have a way to automatically constrain types like Rust.

Duck typing still has a few advantages. It is easy get something working, because you don't have to specify every constrain in advance. It is also possible to generate some very specific error messages.

```
void call_print_static(T)(T t){
    static assert(!is(T == class), T.stringof ~ " is a class and can not be used.");
    t.print();
}

void main()
{
    //f is a pointer
    auto f = new Foo(42);
    call_print_dynamic(f);
    call_print_static(f); // error
}
/*
source/app.d(21,5): Error: static assert  "Foo is a class and can not be used."
source/app.d(30,22):        instantiated from here: call_print_static!(Foo)
*/
```

### Type inference
In Rust it is possible to write:

```Rust
fn test() -> Result<i32, String> {
    Ok(42)
}
```

This works because Rust knows that the return type is `Result<i32, String>` and that `Ok<i32>` is part of it. You can not write this in D as far as I know.

The D approach could look like this

```D
alias SomeResult = Result!(int, string);
SomeResult test(){
    return SomeResult.ok(42);
}
```

### Explicit implicit conversion/coercions

```Rust
use std::ops::Deref;

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T>{
    fn new(val: T) -> Wrapper<T> {
        Wrapper{value: val}
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &T{
        &self.value
    }
}

struct Foo {
    i: i32,
}

impl Foo {
    fn new(i: i32) -> Foo {
        Foo { i: i }
    }
}

trait Print {
    fn print(&self);
}

impl Print for Foo {
    fn print(&self) {
        println!("Foo {}", self.i);
    }
}

fn main() {
    let f = Wrapper::new(Foo::new(42));
    f.print();
}
```

>Here’s the rule: If you have a type U, and it implements Deref<Target=T>, values of &U will automatically coerce to a &T [quote](https://doc.rust-lang.org/book/deref-coercions.html)

This is also the reason that you can pass a `String` into a function that expects a `&str`.

```D
struct Wrapper(T){
    T value;
    this(T _value){
        value = _value;
    }
    alias value this;
}

interface Print{
    void print();
}

class Foo: Print{
     int i;
     this(int _i){
         i = _i;
     }
     void print(){
         import std.stdio: writeln;
         writeln("Foo ", i);
     }
}

void main()
{
    //f is a pointer
    auto f = Wrapper!Foo(new Foo(42));
    f.print();
}
```
In D calling `f.print()` is then the same thing as `f.value.print()`. This means `Wrapper<T>` can be passed to anything that expects a `T`.

[Proxy](https://dlang.org/phobos/std_typecons.html#.Proxy) can be used to forbid the conversions and only forward the methods to the new type.

### Unused generics

In D it is perfectly legal to create types with unused generics:

```D
struct Foo(A, B){}
```

Rust doesn't allow this by default and requires phantom data.

```Rust
use std::marker::PhantomData;
struct Foo<A, B>{
    _m_a: PhantomData<A>,
    _m_b: PhantomData<B>,
}
```

### Variadics

Rust doesn't have type level variadics and has to resort to macros. A commonly used variadic macro is `vec![...]`.

```D
// duck typing
void variadicPrint(Args...)(Args args){
    // compile time loop
    foreach(ref arg; args){
        arg.print();
    }
}

void main()
{
    auto wrapperF = Wrapper!Foo(new Foo(42));
    auto f = new Foo(42);
    variadicPrint(f);
    //or
    variadicPrint(f, f, wrapperF, f);
}
```

Because `variadicPrint` is duck typed, anything that has a `print` method, can be passed into it. Variadics also allow `Tuple` to be implemented as a library in D.

### Pattern matching

Rust natively supports algebraic data types which can be matched on.

```Rust
fn main() {
    let o = Some(5);
    match o {
        Some(i) => println!("{}", i),
        None => println!("None")
    }
}
```

While D has algebaric data types (adt) as a library inside phobos, they have a significant overhead and can not be used at compile time. Luckily adt's can be implemented in around 30 lines of code.

```D
alias Test = Algebraic!(uint, float);
uint i = 5;
float f = 5.0f;
auto b = Test(f);
b.match!(
    (uint i) => writeln("uint, ", i),
    (float f) => writeln("float ", f),
);
```

Rust allows type deconstruction

```Rust
fn main() {
    let t = (1, "Test");
    let (a, b) = t;
}
```

While something similar can be implemented in D, it will not be as usable.

### Type level values

Rust doesn't really have `type level values`, the closest thing would probably be `typenum`. See my [blog post](https://maikklein.github.io/post/impression-rust/) about using `typenum` for a vector / matrix library.

`Type level values` are most commonly used as `type level integers`. They allow the user to express a vector math library like this

```D
alias Vec3f = Vector<float, 3>;
```
D isn't limited to integrals, you can basically pass any type into a template at compile time.
```D
struct Worker{
    string name;
    uint id;
}

string level(Worker worker)(){
    static if(worker.id < 10){
        return "Grunt";
    }
    else{
        return "Boss";
    }
}
void main()
{
    // done at compile time
    enum string grunt = level!(Worker("Tom", 5));
    enum string boss = level!(Worker("Jeff", 20));
}
```

### Metaprogramming

Rust has 3 features that allows metaprogramming, macros, traits and compiler plugins.

The main purpose of macros is to reduce duplicated code, but they also allow custom syntax like this

```Rust
for e in recurrence!(f[i]: f64 = 1.0 ... f[i-1] * i as f64).take(10) {
    println!("{}", e)
}
```
[link](https://danielkeep.github.io/tlborm/book/pim-README.html)

Traits allow implementations to be implemented for a specific range of types, allow types to be constrained and allow for dynamic dispatch.

Compiler plugins are unstable can operate on the AST and can potentially execute arbitrary code at compile time. An example would be
[libhoare](https://github.com/nrc/libhoare)
```Rust
#[precond="x > 0"]
#[postcond="result > 1"]
fn foo(x: int) -> int {
    let y = 45 / x;
    y + 1
}
```

While D natively has design by contract, I don't believe this could be implemented as nicely in D. The closest thing I could imagine would be by using `UDA` and or mixins. But as far as I know it is not possible to directly manipulate the AST in D.

D has a full arsenal of metaprogramming tools. Type level values, variadics, type level computations, static if, mixins, templates, CTFE, static reflection. The blog post would become too long to showcase every feature but I already made a few blog posts that use some of those features. [SoA](https://maikklein.github.io/post/soa-d/), [TypeObject ala Boost hana](http://maikklein.github.io/2016/03/01/metaprogramming-typeobject/), [Strings to types](https://maikklein.github.io/post/2015-11-14-Converting-strings-to-types/).

CTFE in D is more limited than compiler plugins. It does not allow to execute arbitrary code like connecting to a database at compile time, as it would be a security risk.

### Standard library, manual memory management and ownership semantics

*Before we start let me tell you that it is technically possible to completely avoid the garbage collection in D, but there are currently a few problems.*

[Rust std](https://doc.rust-lang.org/std/) and [D phobos](https://dlang.org/phobos/index.html)

In my opinion a good standard library is very import for a language. The biggest reason for that is every library will use the standard library to some degree.

In Rust most libraries are already using `Box`, `Rc`, `Arc`, `Result`, `Option`. In D there currently is `Unique` and `RefCounted` which still use the GC. `Nullable` is similar to `Optional` but it implements auto deref with `alias this` and could therefore cause subtle problems.

Ownership can be modelled in D by disabling the copy constructor. A type without a copy constructor must be moved which is similar to Rust. Currently types without a copy constructor can not reasonably be used with the standard library in D as you would have to explicitly call `move` on them.

This means that you would have to recreate everything in the std that copies in their implementation, which is a lot. Then there are other smaller problems, like how do you move variadic arguments?

```D
void foo(Args)(Args args){
    bar(args); // how do you pass variadics?
}
```

The way I currently implement this is by looking at the type, if it has no copy constructor I call move, otherwise I copy. I generate a compile time string that looks like this `"arg[0], arg[1].move, arg[2].move, arg[3]"`.

Basically what I am trying to say is that ownership semantics are not as refined as in Rust at the moment and avoiding the GC with smart pointers will be a lot of work.

To avoid the Gc in D one can use the [allocator](https://dlang.org/phobos/std_experimental_allocator.html) library that ships with D. It handles all the low level stuff and exposes generic and composable allocators.

Rust currently does not have any allocators but it seems to come into `nighly` soon.


### Conclusion

I am fairly certain that I forgot to cover a few topics but this should give you a rough overview.
