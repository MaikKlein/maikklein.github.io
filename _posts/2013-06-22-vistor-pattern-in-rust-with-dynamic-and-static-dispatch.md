---
layout: post
title: "Vistor Pattern in Rust with dynamic and static dispatch"
description: ""
category: 
tags: []
---

{% highlight rust %}



trait Visitor<T>{
	fn visit(&self, t: T);
}
struct PrintVisitor;
struct PrintReverseVisitor;

impl Visitor<~str> for PrintVisitor {
	fn visit(&self, s: ~str){
		println(s);
	}
}

impl Visitor<~str> for PrintReverseVisitor {
	fn visit(&self, s: ~str){
		let x: ~[char] = s.rev_iter().collect();
		println( std::str::from_chars(x));
	}
}


struct SomeObjectWithStaticDispatch;
impl<V: Visitor<~str>> SomeObjectWithStaticDispatch {
	fn do_sth<'r>(&self, v: &'r V , s: ~str){
		v.visit(s);
	}
}
struct SomeObjectWithDynamicDispatch;

impl SomeObjectWithDynamicDispatch {
	fn do_sth<'r>(&self, v: &'r Visitor<~str>, s: ~str){
		v.visit(s);
	}
}
fn main(){

let v = &PrintVisitor;
let v1 = &PrintReverseVisitor;

let o_static = SomeObjectWithStaticDispatch;
let o_dynamic = SomeObjectWithDynamicDispatch;

o_static.do_sth(v, ~"Hello");
o_static.do_sth(v1, ~"Hello");

o_dynamic.do_sth(v as &Visitor<~str>, ~"Hello");
o_dynamic.do_sth(v1 as &Visitor<~str>, ~"Hello");
}



{% endhighlight %}
