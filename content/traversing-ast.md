+++
date        = 2017-08-20
title       = "AST traversal and code generation"
tags        = [ "Rust"]
+++
[Previous blog post](https://maikklein.github.io/post/shading-language-part1/) 

In this blog post I will mainly talk about AST traversal and code generation and the problems that I am currently facing.

What is AST traversal? 

```Rust
#[derive(Debug)]
enum Expr<'a> {
    Num(i32),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
}

fn fold_expr<T, Unary: Fn(i32) -> T, Add: Fn(T, T) -> T, Mul: Fn(T, T) -> T>(
    unary: &Unary,
    add: &Add,
    mul: &Mul,
    e: &Expr,
) -> T {
    let rec = |e: &Expr| fold_expr(unary, add, mul, e);
    match e {
        &Expr::Num(i) => unary(i),
        &Expr::Add(l, r) => add(rec(l), rec(r)),
        &Expr::Mul(l, r) => mul(rec(l), rec(r)),
    }
}

fn main() {
    let eval = |e: &Expr| fold_expr(&|a| a, &|a, b| a + b, &|a, b| a * b, e);
    let e = Expr::Add(&Expr::Num(5), &Expr::Mul(&Expr::Num(10), &Expr::Num(5)));
    println!("{:?}", eval(&e));
}
```
[Playground](https://play.rust-lang.org/?gist=5f24062414040aca65ae33a3e1dfbc06&version=nightly)

Here we create a small AST, and then we `fold` it into a single value. I translated this directly from some Haskell code that I found a few days ago. While this is not completely idiomatic Rust, it didn't translate too badly. An alternative to this traversal is traversal with the visitor pattern. This is how the visitor pattern looks currently in Rust

[visit.rs](https://github.com/rust-lang/rust/blob/master/src/libsyntax/visit.rs#L52)
```Rust
pub trait Visitor<'ast>: Sized {
    ...
    fn visit_local(&mut self, l: &'ast Local) { walk_local(self, l) }
    fn visit_mod(&mut self, m: &'ast Mod, _s: Span, _attrs: &[Attribute], _n: NodeId) {
        walk_mod(self, m);
    }
    ...
}
pub fn walk_mod<'a, V: Visitor<'a>>(visitor: &mut V, module: &'a Mod) {
    walk_list!(visitor, visit_item, &module.items);
}

pub fn walk_local<'a, V: Visitor<'a>>(visitor: &mut V, local: &'a Local) {
    for attr in local.attrs.iter() {
        visitor.visit_attribute(attr);
    }
    visitor.visit_pat(&local.pat);
    walk_list!(visitor, visit_ty, &local.ty);
    walk_list!(visitor, visit_expr, &local.init);
}


```

Every function inside the trait has a default implementation with a walk function. This means that if you want to implement your own `Visitor` you only have to overwrite the functions that you are interested in. And when you want to overwrite a function, you can reuse the walk functions. I think this is actually a really nice pattern but I had a few practical problems with it.


```Rust
fn visit_assign(&mut self, stmt: &'a Stmt<'a>, assign: &'a Assign<'a>, data: &Self::Data) {
    walk_assign(self, stmt, assign, data);
    let spirv_expr = self.ctx.spirv_expr.get(&assign.expr.node_id).expect("expr");
    let var_def = self.ctx
        .scopes
        .get_var_def(&stmt.node_id, &assign.ident)
        .expect("no var def");
    let spirv_var = self.ctx.spirv_var.get(&var_def.node_id).expect(
        "no spirv var",
    );
    self.ctx.builder.store(
        spirv_var.var_id,
        *spirv_expr,
        None,
        &[],
    );
}
pub fn walk_assign<'a, V: Visitor<'a>>(
    visitor: &mut V,
    stmt: &'a Stmt<'a>,
    assign: &'a Assign<'a>,
    data: &V::Data,
) {
    visitor.visit_expr(&assign.expr, data);
}
```

This is code from from my spirv compiler. Essentially it generates spirv for assignments `a = 4;`. On the left side is a variable with a name, which we need to find. On the right side is an expression which need to evaluate before we can generate the code for the assignment. This is why I call `walk_assign` at the top. The problem is that this visitor can not return values directly. I currently put the results in a `VecMap` which is like a hash map, but more efficient for numbers that are close together.

```Rust
walk_assign(self, stmt, assign, data);
let spirv_expr = self.ctx.spirv_expr.get(&assign.expr.node_id).expect("expr");
```

There are a few problems, first we have to write the result into some container which means that the lookup could fail and it makes multi-threading more painful than it needs to be. 

```Rust
extern crate rayon;

#[derive(Debug)]
pub enum Expr<'a> {
    Num(i32),
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Mul(&'a Expr<'a>, &'a Expr<'a>),
}

pub trait Visitor<'a>: Sized + Sync {
    type R: Send;
    fn visit_expr(&self, e: &'a Expr<'a>) -> Self::R {
        walk_expr(self, e)
    }
    fn visit_mul(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R;
    fn visit_add(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R;
    fn visit_num(&self, i: i32) -> Self::R;
}

pub fn walk_expr<'a, V: Visitor<'a>>(v: &V, e: &'a Expr<'a>) -> V::R {
    // Maybe use join here?
    let mut r: Option<V::R> = None;
    rayon::scope(|scope| {
        scope.spawn(|_| {
            r = Some(match e {
                &Expr::Add(l, r) => v.visit_add(l, r),
                &Expr::Mul(l, r) => v.visit_mul(l, r),
                &Expr::Num(i) => v.visit_num(i),
            })
        });
    });
    r.unwrap()
}

pub fn walk_add<'a, V: Visitor<'a>>(v: &V, l: &'a Expr<'a>, r: &'a Expr<'a>) -> (V::R, V::R) {
    rayon::join(|| v.visit_expr(l), || v.visit_expr(r))
}

pub fn walk_mul<'a, V: Visitor<'a>>(v: &V, l: &'a Expr<'a>, r: &'a Expr<'a>) -> (V::R, V::R) {
    rayon::join(|| v.visit_expr(l), || v.visit_expr(r))
}

pub struct FoldVisitor;

pub fn fold_expr<'a, 'b>(e: &'a Expr<'b>) -> i32 {
    let f = FoldVisitor {};
    FoldVisitor::visit_expr(&f, e)
}

impl<'a> Visitor<'a> for FoldVisitor {
    type R = i32;
    fn visit_mul(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R {
        let (l, r) = walk_add(self, l, r);
        l * r
    }

    fn visit_add(&self, l: &'a Expr<'a>, r: &'a Expr<'a>) -> Self::R {
        let (l, r) = walk_add(self, l, r);
        l + r
    }

    fn visit_num(&self, i: i32) -> Self::R {
        i
    }
}

fn main() {
    let e = Expr::Mul(&Expr::Num(5), &Expr::Add(&Expr::Num(10), &Expr::Num(5)));
    println!("{:?}", fold_expr(&e));
}
```

Essentially two things changed. The visitor is now borrowed immutably, every function now has an explicit return and most visit functions have no default implementation anymore. Also the walk functions can now be paralellized. I decided to use rayon because it easy to use and it almost does what I want.

For me it is important that the compiler is designed from the ground up to support multi-threading, as it would most likely not be easy to add it later.

The main problem with this example is that it is too simple. 

Consider this let statement.

```Rust
let a: Bar = foo();
```

Before we can typecheck this expression we need to find the correct function `foo` as there could be many more functions named `foo` in different scopes. Then we need to extract the return type, which is a user defined type. This means that we also have to find that type in the correct scope. Also `Bar` can be defined after `foo`.

```Rust
fn foo() -> Bar{
    Bar{}
}
struct Bar; 
```

Because of this, we need to collect information about the AST before we can do type checking. The same is true for code generation

```Rust
let a = 4;
let b = a;
```

We first create a new variable and store the value `4` in `a`, then we create variable `b` and load the value of `a` and store it in `b`. This is the equivalent SPIR-V code.

```Rust
 %a = OpVariable %_ptr_Function_uint Function
      OpStore %a %uint_4
 %b = OpVariable %_ptr_Function_uint Function
%20 = OpLoad %uint %a
      OpStore %b %20
```

This means that when we traverse the AST and generate code for `a`, we need to reuse the symbols that we generated previously. To me this seems like an inherently mutable problem (at least in Rust). I am not sure if and immutable solution would be a viable approach in Rust. I am thinking of using a [concurrent hashmap](https://docs.rs/chashmap/2.2.0/chashmap/) unless I can think of a better solution.

The other problem is the code generation itself. I currently have one mutable builder object that can record instructions.

```Rust
self.ctx.builder.store(
    spirv_var.var_id,
    *spirv_expr,
    None,
    &[],
);
```

Of course I can not use this in multi-threaded code. I could wrap in a `Mutex` but that would completely kill the performance as this object is accessed in every node inside the AST. One solution that I am thinking of is to create a separate builder object in every traversal, record commands and return the object. Then I could stitch those objects together when I traverse the AST from the bottom to the top. The caveat is that this would introduce many small allocations and Rust currently doesn't really have good solution for memory management with user defined allocators.

Traversing the AST turned out to be harder than I thought but I think the visitor pattern with explicit returns will result in cleaner and better code.
