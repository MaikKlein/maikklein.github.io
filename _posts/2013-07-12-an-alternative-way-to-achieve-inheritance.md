---
layout: post
title: "An Alternative Way To Achieve Inheritance"
description: ""
category: 
tags: []
---

{% highlight rust %}

struct UsefulPrinter<'self>{
    smiley: ~str,
    print_fn_first: &'self fn(),
    print_fn_second: &'self fn()
}
impl<'self> UsefulPrinter<'self> {
    fn print(&self){
       (self.print_fn_first)();
       (self.print_fn_second)();
       println(" "+ self.smiley);
    }
}


struct HelloWorldPrinter<'self>{
    printer: UsefulPrinter<'self>
}
impl<'self> HelloWorldPrinter<'self> {
    fn new() -> HelloWorldPrinter<'self>{
    let print_fn_first = || print("Hello ");
    let print_fn_second = || print("World");
    
    let printer = UsefulPrinter{smiley: ~":D",
                                print_fn_first: print_fn_first,
                                print_fn_second: print_fn_second};

    HelloWorldPrinter{printer: printer}

    }
}

struct IamSoTiredPrinter<'self>{
    printer: UsefulPrinter<'self>
}
impl<'self> IamSoTiredPrinter<'self> {
    fn new() -> IamSoTiredPrinter<'self>{
    let print_fn_first = || print("Leave me alone.... ");
    let print_fn_second = || print("I am so tired.");
    
    let printer = UsefulPrinter{smiley: ~":(",
                                print_fn_first: print_fn_first,
                                print_fn_second: print_fn_second};

    IamSoTiredPrinter{printer: printer}

    }
}
fn main() {
    let hwp = HelloWorldPrinter::new();
    hwp.printer.print();

    let istp = IamSoTiredPrinter::new();
    istp.printer.print();
}

/* 
output:
Hello World :D
Leave me alone.... I am so tired. :(
*/



{% endhighlight %}
