---
layout: post
title: "Lifetimes explained"
description: ""
category: 
tags: []
---


{% highlight rust %}

{% endhighlight %}

So you want to learn about lifetimes in Rust? Okay let's give it a try then. We are starting by creating a new struct let's call it 

{% highlight rust %}
struct Foo{
    name: ~str
}
{% endhighlight %}

Okay that was easy right? Hm.. but it isn't really useful right now. Let's change that, but before we do that we need to know how borrowing works.

{% highlight rust %}
struct Foo{
    name: ~str
}

fn main() {
    let foo = Foo{name: ~"Bar"};
    let ref_foo   = &foo;
    let ref_foo_2 = &foo;

    printfln!(foo); // prints {name: ~"Bar"}
    printfln!(ref_foo); // prints &{name: ~"Bar"}
    printfln!(ref_foo_2); // prints &{name: ~"Bar"}

}
{% endhighlight %}

So what is going on here? Well we created our new object `foo`, then we created two 'borrows' to `foo`. We called them `ref_foo` and `ref_foo_2`. You can see them as a pointer. `ref_foo` and `ref_foo_2` are currently pointing to `foo`. So what happens if we change `foo`?

{% highlight rust %}
struct Foo{
    name: ~str
}

fn main() {
    let mut foo = Foo{name: ~"Bar"};
    let ref_foo   = &foo;
    let ref_foo_2 = &foo;

    printfln!(foo); // prints {name: ~"Bar"}
    printfln!(ref_foo); // prints &{name: ~"Bar"}
    printfln!(ref_foo_2); // prints &{name: ~"Bar"}

    foo.name = ~"SomeOtherName";
    /*
    14:12 error: cannot assign to `foo.name` because it is borrowed
    foo.name = ~"SomeOtherName";
    ^~~~~~~~
    */
}
{% endhighlight %}

Oh.. that is unfortunate. We just got an error. What this error is telling us is that we are currently borrowing the variable `foo`. In other words `ref_foo` and `ref_foo_2` are currently pointing to `foo` and Rust doesn't let us modify `foo` while `foo` has pointers pointing on it. This is part of Rust's memory safety guarantee.

Okay what have we learned so far? 

1.) & acts like a pointer.
2.) We can not change borrowed variables.

Okay what if we would write something like this?

{% highlight rust %}
struct Foo{
    name: ~str
}

fn weird_function()-> &Foo {
    let foo = Foo{name: ~"Barrr"};
    &foo
}
/*
7:8 error: borrowed value does not live long enough
 &foo
*/
fn main() {
  
}
{% endhighlight %}

We are trying to return a borrowed Foo, but Rust doesn't let us do that! It tells us that the value doesn't live long enough. Okay so why is this happening? Let's have a closer look.

{% highlight rust %}
fn weird_function()-> &Foo {
    let foo = Foo{name: ~"Barrr"};
    &foo
}
/*
7:8 error: borrowed value does not live long enough
 &foo
*/
{% endhighlight %}

The problem with this function is that we are creating a new variable called `foo`. Then we are trying to return the borrowed value. Okay but now it gets a little bit weird. The function block ends and the local variable `foo` is freed. So we would return a pointer that points to no memory and that would be bad. 

Fortunately Rust is preventing this by telling us that the variable `foo` is not living long enough.  

Okay let's say our `struct Foo` isn't good enough anymore! We want another `struct Bar` and Foo should borrow it.

{% highlight rust %}
struct Foo{
    name: ~str,
    ref_bar: &Bar
}
struct Bar {
    name: ~str
}

fn main() {
 
}

/*
3:17 error: Illegal anonymous lifetime: anonymous lifetimes are not permitted here
ref_bar: &Bar
*/
{% endhighlight %}

Oh come on Rust, again? What is this? `Illegal anonymous lifetime`? 
Well if you think about it, this error message makes sense. Remember how Rust only let's us have borrows to valid memory? What if our 
`ref_bar` would get invalid in the life time of Foo?

We need a way to tell Rust "Hey Rust, please let ref_bar be valid as long as Foo is alive". So how would we do that?

With lifetimes!

{% highlight rust %}
struct Foo<'self>{
    name: ~str,
    ref_bar: &'self Bar
}
struct Bar {
    name: ~str
}

fn main() {
  
}
{% endhighlight %}

What is this weird symbol `'self`? This looks scary go away!
Okay basically `'self` denotes the life time of Foo. We are declaring lifetimes in `<..>`. Lifetimes are declared with `'+ any free keyword`. Examples would be `'r 'a 't 'fortytwo 'mountain` and so on. But 'self is a somewhat special case. `'self` is the only lifetime allowed in a Struct / Enum. 

*Note this is going to change soon. `'self` is considered to be a bug. You will be able to name your name your lifetimes in structs as you like.*

So we know how to declare lifetimes but how do we use them?

We are binding them to our borrows. This is done like `ref_bar: &'self Bar`. Now we can read this as "Our borrow has the lifetime of `'self`" Remember that we have declared 'self in Foo. So 'self means the life time of Foo.

Okay now let's look at more examples.

Let's say we want our struct Foo to return a borrow to it's name. Piece of cake right? Okay let's do it.

{% highlight rust %}
struct Foo{
    name: ~str,
}
impl Foo {
    fn get_ref_name(&self) -> &~str{
        &self.name
    }
}

fn main() {
}
/*
error: cannot infer an appropriate lifetime due to conflicting requirements
&self.name
*/
{% endhighlight %}

Pfffff again a life time error...
Now what are we trying to do here? We are returning a borrow to self.name right? this seems reasonable but why is Rust complaining about it? Okay let's think about what could happen.

{% highlight rust %}
let foo = Foo{name: ~"foo"};
let ref_name = foo.get_ref_name();
{% endhighlight %}

Okay what would happen to our variable ref_name if `foo` is freed? Right it would point to invalid memory! So again .. we need a way to tell rust that our variable `ref_name` is valid as long as `foo` is valid.


{% highlight rust %}
struct Foo{
    name: ~str,
}
impl Foo {
    fn get_ref_name<'r> (&'r self) -> &'r ~str{
        &self.name
    }
}

fn main() {
    let foo = Foo{name: ~"foo"};
    let ref_name = foo.get_ref_name();
    printfln!(ref_name); // prints &~"foo"
}
{% endhighlight %}

Okay let's analyze it again. First we are declaring our life time in `<..>` right? We are doing this like...
{% highlight rust %}
fn get_ref_name<'r> 
{% endhighlight %}

So now Rust knows that there is a lifetime. But how long does it live? Hm.. good question. I don't know and Rust also doesn't know. We need to bind it first. 

{% highlight rust %}
... &'r self ... 
{% endhighlight %}

By writing this Rust now knows that `'r` has the life time of Foo. Okay hm... but something is missing right? Now we need to tell Rust that our returning borrow also has the lifetime of `'r`
{% highlight rust %}
fn get_ref_name<'r> (&'r self) -> &'r ~str{
        &self.name
}
{% endhighlight %}

Okay that's it. We just told the returning borrow to have the lifetime of `'r`. Let's have a look at our previous code again.

{% highlight rust %}
let foo = Foo{name: ~"foo"};
let ref_name = foo.get_ref_name();
{% endhighlight %}

Rust now knows that ref_name is valid as long as foo is valid.
