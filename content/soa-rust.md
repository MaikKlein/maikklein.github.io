+++
date        = 2017-01-03
title       = "SoA in Rust with Macros 1.1"
tags        = [ "Rust", "Macros1.1", "SoA"]
+++

*Disclaimer*: This is just a proof of concept.

## SoA
```Rust
#![feature(proc_macro)]
#[macro_use]
extern crate soa_derive;

#[derive(Debug)]
struct Vec2{
    x: f32,
    y: f32
}

impl Vec2{
    pub fn new() -> Self{
        Vec2{
            x: 0.0,
            y: 0.0
        }
    }
}

#[derive(SoA)]
struct GameObject {
    pos: Vec2,
    vel: Vec2,
    health: f32,
    // Other fields . . .
}

fn main() {
    let mut soa = GameObjectSoA::new();
    let game_object = GameObject {
        pos: Vec2::new(),
        vel: Vec2::new(),
        health: 42.0,
    };
    soa.push(game_object);
    println!("{:?}", soa);
}
```
Let us have a look at the generated code.

```Rust
#[derive(Debug)]
struct GameObjectSoA {
    pub pos: Vec<Vec2>,
    pub vel: Vec<Vec2>,
    pub health: Vec<f32>,
}
impl GameObjectSoA {
    pub fn new() -> Self {
        GameObjectSoA {
            pos: Vec::new(),
            vel: Vec::new(),
            health: Vec::new(),
        }
    }
    pub fn push(&mut self, value: GameObject) {
        let GameObject { pos: pos, vel: vel, health: health } = value;
        self.pos.push(pos);
        self.vel.push(vel);
        self.health.push(health);
    }
}
```
`soa_derive` essentially turns a struct of fields into a struct of arrays.

```Rust
// aos = array of structures
let mut aos = Vec::<GameObject>::new();
// soa = structure of arrays
let mut soa = GameObjectSoA::new();
```

You might ask yourself, "Why is this useful?". The answer is mainly for performance. Often when you want to iterate over a `Vec<GameObject>`, you don't actually care about every field. For example you might just want to adjust the `health` of every `GameObject`. For AoS that means that you will needlessly load a lot of data into your cache that you actually never use. Of course AoS is still useful for data that you want to access together for example `Vec<Vec2>`.


The nice thing about `soa_derive` is that the usage is very similar to AoS.
```Rust
// SoA
let mut soa = GameObjectSoA::new();
let game_object = GameObject {
    pos: Vec2::new(),
    vel: Vec2::new(),
    health: 42.0,
};
soa.push(game_object);
```
```Rust
// AoS
let mut aos = Vec::<GameObject>::new();
let game_object = GameObject {
    pos: Vec2::new(),
    vel: Vec2::new(),
    health: 42.0,
};
aos.push(game_object);
```
You can find the code [here](https://github.com/MaikKlein/soa_derive). It is nothing more than a proof of concept.

## Macros 1.1

Overall I really like Macros 1.1 but there are a few things that are a bit awkward. For example generating the push method:
```Rust
pub fn push(&mut self, value: GameObject) {
    let GameObject { pos: pos, vel: vel, health: health } = value;
    self.pos.push(pos);
    self.vel.push(vel);
    self.health.push(health);
}
```
And specifically this line
```Rust
let GameObject { pos: pos, vel: vel, health: health } = value;
```
I think the limitations of `quote!` are that you can only interpolate something that implements `ToTokens` and you can only use it one time.

The following code does not compile
```Rust
let fields: Vec<Field> = ...;
let field_idents: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();
quote!{
    impl #soa_ident {
        pub fn push(&mut self, value: #ident){
            let #ident{#(#field_idents: #field_idents, )*} = value;
            #(
               self.#field_idents.push(#field_idents);
            )*
        }
    }
}
```

I had to write it like this
```Rust
let field_idents: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();
let push_self: Vec<quote::Tokens> = fields.iter()
    .map(|f| {
        let field = f.ident.clone().unwrap();
        quote!{
            self.#field.push(#field);
        }
    })
    .collect();
let deconstruct_list: Vec<quote::Tokens> = fields.iter()
    .map(|f| {
        let field = f.ident.clone().unwrap();
        quote!{
            #field: #field
        }
    })
    .collect();

quote!{
    impl #soa_ident {
        pub fn push(&mut self, value: #ident){
            let #ident{#(#deconstruct_list, )*} = value;
            #(
                #push_self
            )*
        }
    }
}
```
Essentially I had to create a new loop that would exactly output the thing that I wanted.

What I really wanted to write would be something like this
```Rust
// Pseudo code
// For readability I didn't handle the option case.
let fields: Vec<Field> = ...;
quote!{
    impl #soa_ident {
        pub fn push(&mut self, value: #ident){
            let #ident{#(#[fields.ident]: #[fields.ident], )*} = value;
            #(
               self.#[fields.ident].push(#[fields.ident]);
            )*
        }
    }
}
```
Instead of directly interpolating on values that implement `ToTokens`, only the "expressions" would have to require `ToTokens`. Here the expression would be #[field.ident]. I am not sure if that could be implemented but it would essentially get rid of all the temporary `Vec`'s that you have to create to get the correct output.

What I really like this that the generated code is just a string which you can just print to the console. Of course it is barely readable because everything will be on the same line but you can just format the string with `rustfmt`.

```Rust
#[proc_macro_derive(SoA)]
pub fn soa_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = gen_soa_derive(&ast);
    // Displays the generated code
    println!("{}", gen);
    gen.parse().unwrap()
}
```

And if you are curious this is how the push method would look in `D`.
[Blog post](https://maikklein.github.io/post/soa-d/)
```D
void insertBack(T t){
    if(length == size) grow;
    foreach(index, _; Types){
        containers[index][length] = __traits(getMember, t, MemberNames[index]);
    }
    length = length + 1;
}
```

### Update1:
There is actually a workaround for accessing the same iterator multiple times in a `quote!`.[Source](https://www.reddit.com/r/rust/comments/5lrb9y/soa_in_rust_with_macros_11/dbxwfy7/)

```Rust
let field_idents_: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();
let field_idents = &field_idents_;
let field_idents1= &field_idents_;

quote!{
    #[derive(Debug)]
    struct #soa_ident {
        #(
            #vec_fields,
        )*
    }
    impl #soa_ident {
        pub fn new() -> Self {
            #soa_ident {
                #(
                    #field_idents : Vec::new(),
                )*
            }
        }

        pub fn push(&mut self, value: #ident){
            let #ident{#(#field_idents: #field_idents1, )*} = value;
            #(
                self.#field_idents.push(#field_idents1);
            )*
        }
    }
}
```
