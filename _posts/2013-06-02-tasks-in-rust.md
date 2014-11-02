---
layout: post
title: "Tasks in Rust"
description: ""
category: 
tags: []
---

{% highlight rust %}

use std::comm::{stream, Chan,Port};

fn main() {
  
// creates 100 tasks
let chans = do vec::from_fn(100) |tasknumber| {
    let (port, chan) = stream::<int>();
    do spawn {
        let value = port.recv();
        print(fmt!("I am task # %? -> %? \n",tasknumber,value));
    }
    chan
};
// send 42 to all tasks
for chans.each |chan| {
  chan.send(42);
  
}
/* Prints:
*printing order is arbitrary*
I am task # 23 -> 42 
I am task # 31 -> 42 
I am task # 47 -> 42 
I am task # 71 -> 42 
I am task # 75 -> 42 
I am task # 83 -> 42 
I am task # 99 -> 42 
I am task # 35 -> 42
...
*/


}

{% endhighlight %}


