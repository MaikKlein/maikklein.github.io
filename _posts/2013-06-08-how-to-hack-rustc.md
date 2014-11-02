---
layout: post
title: "How to hack rustc"
description: ""
category: 
tags: []
---

We are going to hack rustc. Our goal is to extend the functionality of rustc so that we can write "rustc --printString".

*Note this tutorial assumes that you are using linux*

Open the following file:
{% highlight rust %}
/rust/src/librustc/driver/driver.rs
{% endhighlight %}

Now search for the function

{% highlight rust %}
pub fn optgroups() -> ~[getopts::groups::OptGroup] {
{% endhighlight %}
This is the main function where we will pattern match the input. Add the following line 
{% highlight rust %}
  optflag("",  "printString", "Prints a string"),
{% endhighlight %}
Your function should look like this
{% highlight rust %}
pub fn optgroups() -> ~[getopts::groups::OptGroup] {
 ~[
  optflag("",  "printString", "Prints a string"),
  optflag("",  "bin", "Compile an executable crate (default)"),
  optflag("c", "",    "Compile and assemble, but do not link"),
  ....
{% endhighlight %}

Now you have to open
{% highlight rust %}
/rust/src/librustc/rustc.rc
{% endhighlight %}

and search for the following function

{% highlight rust %}
pub fn run_compiler(args: &~[~str], demitter: diagnostic::Emitter) {
{% endhighlight %}

Now you can pattern match the argument like this
{% highlight rust %}
if opt_present(matches, "printString") {
        io::println("Rustc says: Hello");  
        return;
}
{% endhighlight %}
Your function should look like this
{% highlight rust %}
pub fn run_compiler(args: &~[~str], demitter: diagnostic::Emitter) {
    // Don't display log spew by default. Can override with RUST_LOG.
    ::core::logging::console_off();

    let mut args = /*bad*/copy *args;
    let binary = @args.shift();

    if args.is_empty() { usage(*binary); return; }

    let matches =
        &match getopts::groups::getopts(args, optgroups()) {
          Ok(m) => m,
          Err(f) => {
            early_error(demitter, getopts::fail_str(f));
          }
        };

    if opt_present(matches, "h") || opt_present(matches, "help") {
        usage(*binary);
        return;
    }
    if opt_present(matches, "printString") {
        io::println("Rustc says: Hello");  
        return;
    }
    ....
{% endhighlight %}


Now save and go to your main rust folder and type
{% highlight rust %}
make rustc-stage1
{% endhighlight %}


Goto 
{% highlight rust %}
/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
{% endhighlight %}

And now you can call your flag like this

{% highlight rust %}
./rustc --printString
{% endhighlight %}

And it will hopefully output the following:

{% highlight rust %}
Rustc says: Hello
{% endhighlight %}






