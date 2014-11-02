---
layout: post
title: "Macro's in Rust"
description: ""
category: 
tags: []
---

{% highlight rust %}

macro_rules! add(
    ($sp:ident) => (
        $sp + $sp
    );
)
fn main() {
 
let s = ~"Hello World ";
println(fmt!("%?",add!(s))); // prints ~"Hello World Hello World "

let i = 10;
println(fmt!("%?",add!(i))); // prints 20

let v =  ~[1,2,3,4,5];
println(fmt!("%?",add!(v))); // prints ~[1, 2, 3, 4, 5, 1, 2, 3, 4, 5]

}

{% endhighlight %}

How cool is that?
