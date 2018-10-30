+++
date        = 2016-11-27
title       = "First steps in Nom: Parsing pseudo GLSL"
tags        = [ "Rust"]
+++

I am currently working on my rendering engine and I always wanted to streamline my shader pipeline. I want to detect `.glsl` files, parse them, extract the type information and generate Rust bindings inside the build script.

GLSL can look like this:

```GLSL
#version 400
layout(location = 0) uniform vec4 color;
layout(location = 1) uniform mat4 mvp;

layout(location = 0) in vec4 pos;
in vec2 uv;

layout(location = 0) out vec3 test;
void main(){
    //...
}
```

Here we have two `uniform` variables of type `vec4` and `mat4`, a `vec4` and a `vec2` as input and a `vec3` as output for a vertex shader.
Before today I had never really written any parser besides for CSV or OBJ and I was always scared of it because I know how complex they can get.
These are my first steps in Nom.

We are going to parse the `GLSL` code from above.
I started by defining an `enum`.
```Rust
#[derive(Debug)]
pub enum Glsl {
    Version(u32),
    Input(Option<u32>, String, String),
    Output(Option<u32>, String, String),
    Uniform(Option<u32>, String, String),
}
```

I just treat `Input`, `Output` and `Uniform` the same for now. There are edge cases that I just ignore for now for example we will not parse uniforms that are directly initialized nor array types.

We start by parsing the `Version`:
```Rust
named!(glsl_version<u32>,
    do_parse!(
        tag!("#version") >>
        opt!(space) >>
        number: map_res!(
            digit,
            std::str::from_utf8
        ) >>
        opt!(multispace) >>
        (number.parse::<u32>().unwrap())
    )
);
```

`glsl_version<u32>` will create a function with the name `glsl_version` and a return type of `u32`. We then look for a specific string that matches `#version`. It follows by 0 or more spaces which we express with `opt!(space)`, then we extract n characters that are digits into a variable called `number`. `multispace` recognizes spaces, tabs, carriage returns and line feeds. After that we parse the `number` into a `u32`. I call `.unwrap()` here because it shouldn't fail.

I will do something really hacky that you should probably never do in production code but it helps us to get started.
```Rust
named!(glsl_alt<Option<Glsl>>,
        alt!(
              glsl_version => { |n| Some(Glsl::Version(n)) }
            | take!(1) => { |_| None }
        )
);
```
`alt!` is a conditional parser. It will try to execute the parser in order. If `glsl_version` fails we execute the `take!(1)` parser which consumes 1 byte and returns `None`. This is actually not the right place for `glsl_version` as the `#version` specifier should only occur at the top, but I will let it stay there for now.

This parser only executes once which in the case of `#version` would be enough but we still need to parse `in`, `out` and `uniform`.
```Rust  
named!(parse_glsl<&[u8], Vec<Option<Glsl>> >, many0!(glsl_alt));
```
`many0` will execute the parser repeatedly and will write the results into a `Vec`.
```Rust
layout(location = 0) uniform vec4 color;
```
First we will write a parser for the optional layout specifier.
```Rust
named!(glsl_location<u32>,
    do_parse!(
        tag!("layout") >>
        opt!(space) >>
        tag!("(") >>
        opt!(space) >>
        tag!("location") >>
        opt!(space) >>
        tag!("=") >>
        opt!(space) >>
        location: map_res!(
            digit,
            std::str::from_utf8
        ) >>
        opt!(space) >>
        tag!(")") >>
        opt!(space) >>
        (location.parse::<u32>().unwrap())
    )
);
```
I am sure this could be written more elegantly but it should do the trick. It is very similar to the version parser. Because I pretend that `in`, `out` and `uniform` are the same we will create a macro that generates a parser to avoid code duplication.
```Rust
macro_rules! glsl_gen(
    ($name: ident, $i: expr) => (
        named!($name<(Option<u32>, String, String)>,
            do_parse!(
                location: opt!(glsl_location) >>
                tag!($i) >>
                opt!(space) >>
                type_name: map_res!(
                    alphanumeric,
                    std::str::from_utf8
                ) >>
                opt!(space) >>
                name: map_res!(
                    alphanumeric,
                    std::str::from_utf8
                ) >>
                opt!(space) >>
                char!(';') >>
                opt!(multispace) >>
                (location,
                 FromStr::from_str(type_name).unwrap(),
                 FromStr::from_str(name).unwrap())
            )
        );
    )
);

glsl_gen!(glsl_in, "in");
glsl_gen!(glsl_out, "out");
//Note: Doesn't parse if the uniform has a default initialization
glsl_gen!(glsl_uniform, "uniform");
```
First we use the `glsl_location` parser that we just created and write the value into the variable `location`. `location` is of type `Option<u32>`. After that we basically do the same thing as we did in `glsl_version` and `glsl_location`. The only difference is that we return a tuple of type `(Option<u32>, String, String)`.

Now we can update `glsl_alt`.
```Rust
named!(glsl_alt<Option<Glsl>>,
        alt!(
              glsl_version => { |n| Some(Glsl::Version(n)) }
            | glsl_in => { |(loc, ty, name)| Some(Glsl::Input(loc, ty, name)) }
            | glsl_out => { |(loc, ty, name)| Some(Glsl::Output(loc, ty, name)) }
            | glsl_uniform => { |(loc, ty, name)| Some(Glsl::Uniform(loc, ty, name)) }
            | take!(1) => { |_| None }
        )
);
```
I hope you can see why I have written `glsl_alt` that way, it made it easy to get started but it comes with some problems. Every byte that fails to parse will add a `None` into a `Vec`.
```Rust
fn main() {
    let s = "
        #version 400
        layout(location = 0) uniform vec4 color;
        layout(location = 1) uniform mat4 mvp;

        layout(location = 0) in vec4 pos;
        in vec2 uv;

        layout(location = 0) out vec3 test;
        void main(){
            //...
        }
    ";
    if let IResult::Done(_, o) = parse_glsl(s.as_bytes()){
        let v: Vec<Glsl> = o.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
        println!("{:?}", v);
    }
}
```
For now I am just going to filter all those `None`'s.
```Rust
[Version(400),
 Uniform(Some(0), "vec4", "color"),
 Uniform(Some(1), "mat4", "mvp"),
 Input(Some(0), "vec4", "pos"),
 Input(None, "vec2", "uv"),
 Output(Some(0), "vec3", "test")]
```
For the first day of using `Nom` I call this a success, it was much easier than I thought. Obviously I completely ignored errors and edge cases and tons of other stuff but this is work for another day.
