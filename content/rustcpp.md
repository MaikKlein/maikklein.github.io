+++
date        = "2017-11-29"
title       = "Rust cpp"
tags        = [ "Rust"]
draft = true
+++

* [Mutability](#mutability)
* [Traits](#traits)

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
[Playground]()

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

To reiterate on the meaning of 'compile time' in this context: The AST has to be constructed at compile time and `ExprPrinter` is essentially a zero cost abstraction. And with zero cost abstraction I mean that the resulting code would be no different from hand written print statements.

You can have a look at the generated assembly at [godbolt](https://godbolt.org/g/nR55r3).

The same concept is used for `Iterators`, `Futures` etc.

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
