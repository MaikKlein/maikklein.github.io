---
layout: post
title: "Evaluating D for games"
description: ""
category: 
tags: []
---

{% highlight d %}
{% endhighlight %}

Disclaimer: I have absolutely no experience with D nor am I a good programmer so please take everything that I say with a grain of salt.

My current impression of D is that is a mixture of Java and C++. D seems to offer metaprogramming with templates, compiles to native code, but also seems to make heavy use of the GC. If those statements are true I do not know but I will take some time to evaluate D.

The first code that I wrote:

{% highlight d %}

import std.stdio;
import std.typecons;
class Foo(T){
public:
  T i;
}
class Bar{
public:
  int hello = 0;
}

void main()
{
  Foo!(Bar) f;
  int i = f.i.hello;
}
{% endhighlight %}

I wrote that code without any knowledge of D. Unfortunately if you run this code, you will get a segfault. I think that was my personal record of creating a segault in a new language.

The reason why it segfaults is because classes are heap allocated and f is a nullpointer. Of course that is completely my fault, but I am already a little bit annoyed that classes are by default heap allocated. 

It seems that D is differentiating between structs and classes. Structs are value types and live on the stack while classes are heap allocated. That already makes me somewhat uncomfortable because if I would use 3rd party libraries which would use classes it may use the GC in the background. I also assume that the majority of the D community will make use of the GC.

This makes me wonder how much of the standard library relies on the GC. 

I am trying to find an alternative to std::vector and it seems that would the std.container.Array

{% highlight d %}
import std.stdio;
import std.container;
void main()
{
  Array!int arr;
  arr.insertBack(10);
  writeln(arr);
}
{% endhighlight %}


If you print this something strange is happening. It will output 
{% highlight d %}

Array!int(RefCounted!(Payload, cast(RefCountedAutoInitialize)0)(RefCountedStore(20D9590)))

{% endhighlight %}
Of course it does not automatically convert arr to a string, I should have expected that but this is actually quite interesting. 

At this point I should probably mention that I am using [DCD] (https://github.com/Hackerpilot/DCD), an autocompletion engine for D. I can easily jump to the definition of Array which is quite awesome.

>Array type with deterministic control of memory. The memory allocated for the array is reclaimed as soon as possible; there is no reliance on the garbage collector. Array uses malloc and free for managing its own memory.

Okay but what is RefCounted?

>Defines a reference-counted object containing a T value as payload. RefCounted keeps track of all references of an object, and when the reference count goes down to zero, frees the underlying store. RefCounted uses malloc and free for operation.

My first thought was why does it need to be RefCounted in the first place? There also does not seem a way to customize the container in any way. It is not that bad, I expected to write my own custom containers anyway. At least now I know that it is possible to do manual memory management. 

RefCounted seems to be the equivalent of std::shared_ptr. I wonder if there is something similar to std::unqiue_ptr. 


>struct Unique(T);
Encapsulates unique ownership of a resource. Resource of type T is deleted at the end of the scope, unless it is transferred. The transfer can be explicit, by calling release, or implicit, when returning Unique from a function. The resource can be a polymorphic class object, in which case Unique behaves polymorphically too.

{% highlight d %}
  Unique!Foo f = new Foo;
{% endhighlight %}

D seems to be better than I thought. Out of curiosity I read though some parts of the standard library and I found something interesting.

>template scoped(T) if (is(T == class))
Allocates a class object right inside the current scope, therefore avoiding the overhead of new. This facility is unsafe; it is the responsibility of the user to not escape a reference to the object outside the scope

{% highlight d %}
import std.stdio;
import std.container;
import std.typecons;
class Foo{
  int i;
  this(){ 
    i = 0;
  }
}
void main()
{
  auto f = scoped!Foo();
  f.i = 10;
  writeln(f.i);
}
{% endhighlight %}

So it seems that it is possible to completely avoid the allocation of a class.

Let us see how we would achieve static dispatch in D.
{% highlight d %}
import std.stdio;

bool isRenderer(R)(){
  return is(R : OpenGL) ||
         is(R : DirectDraw);
}
struct OpenGL{
  void print() const{
    writeln("OpenGL");
  }
}
struct DirectDraw{
  void print() const{
    writeln("DirectDraw");
  }
}

void printRenderer(R)(const ref R r)
  if(isRenderer!R)
{
  r.print();
}

void main()
{
  OpenGL gl;
  DirectDraw dd;

  printRenderer(gl);
  printRenderer(dd);
}
{% endhighlight %}

This almost looks exactly like the C++. Note that the 'if' between the function definition and the function body is called a 'constrain'.

{% highlight d %}
returnType functionName(param) 
 if(condition)
{
  ...
}
{% endhighlight %}

I think it would basically translate to C++ like this
{% highlight d %}
template<typename R>
void printRenderer(const R &r){
  static_assert(!isRenderer::value,"Is not a valid renderer.");
  r.print();
}
{% endhighlight %}

The problem with this approach is that once we want to add a new renderer we would have to change existing code.
{% highlight d %}
bool isRenderer(R)(){
  return is(R : OpenGL)     ||
         is(R : DirectDraw) ||
         is(R : Metal);
}
{% endhighlight %}
Which would not be that bad if we only have to change code at one place, but maybe there is a better way? Let us see how CRTP looks in D.
{% highlight d %}
import std.stdio;
import std.typecons;

class Renderer(T){
  void print(){
    (cast(T)(this))._print();
  }
}
class OpenGL : Renderer!(OpenGL){
  void _print() const{
    writeln("OpenGL");
  }
}
class DirectDraw : Renderer!(DirectDraw){
  void _print() const{
    writeln("DirectDraw");
  }
}
void printRenderer(T)(Renderer!T r){
  r.print();
}

void main()
{
  auto gl = scoped!OpenGL();
  auto dd = scoped!DirectDraw();
  printRenderer(gl);
  printRenderer(dd);
}
{% endhighlight %}
Please note that we have to use classes because only classes support inheritance which is required to implement CRTP. Also note that we have to use the 'scoped' template in order to avoid the heap allocation.

CRTP has the advantage that I can extend existing code without changing a single line. Still, personally I think it is quite inconvenient to use. It forces you to cast your base class to a derived class like this
{% highlight d %}
    (cast(T)(this))._print();
{% endhighlight %}
And you would have to implement your methods with a different name. I used the convention '_methodName'. Personally I would choose the first version with static ducktyping + constrains because I think it looks more natural. 

D has a features that I have never seen in a another language before. D calls it alias this.
{% highlight d %}
import std.stdio;

struct Printer{
  string sentence;
  void print(){
    writeln(sentence);
  }
  this(string s){
    sentence = s; 
  }
}
struct HelloWorld1{
  Printer printer = Printer("Hello World 1");
}
struct HelloWorld2{
  Printer printer = Printer("Hello World 2");
  alias printer this;
}
struct HelloWorld3{
  Printer printer = Printer("Hello World 3");
  alias printer this;
  void print(){
    writeln("Error");
  }
}
void main()
{
  HelloWorld1 hello1;
  hello1.printer.print();//prints Hello World 1

  HelloWorld2 hello2;
  hello2.print();//prints Hello World 2

  HelloWorld3 hello3;
  hello3.print();//prints Error

}
{% endhighlight %}

Alias this could be a really nice feature, usually you would have to write 

{% highlight d %}
  hello1.printer.print();
{% endhighlight %}
if you are using composition. But with alias this you can just write 
{% highlight d %}
  hello2.print();
{% endhighlight %}
The problem is that members/functions are getting shadowed if they already exist in that struct (see HelloWorld3). I am not sure how useful it will be.

I have found a new keyword that could be very useful '@nogc'. It will guarantee at compile time that no function will use the GC. 
{% highlight d %}
@nogc{
  //your code  
}
{% endhighlight %}
Unfortunately this showed me how much of the standard library relies on the GC. Remember RefCounted, Unique and scoped? All of these helper functions rely on the GC. So I do not think that it is feasible anymore to completely disable the GC. Actually this is very strange, the RefCounted documentation clearly states

>RefCounted uses malloc and free for operation.

I assume that RefCounted must use some nongc function under the hood which prevents me from using it without the GC.

Before I go on I have to do some research of how the GC in D is actually implemented. Maybe the implementation is not that bad and if I only rely on a very small amount of allocations that might not even be noticeable.

It is actually very hard to find some details of how the GC works, but I found this quote from the official D website.

>The GC works by:

>Stopping all other threads than the thread currently trying to allocate GC memory.
>‘Hijacking’ the current thread for GC work.
>Scanning all ‘root’ memory ranges for pointers into GC allocated memory.
>Recursively scanning all allocated memory pointed to by roots looking for more pointers into GC allocated memory.
>Freeing all GC allocated memory that has no active pointers to it and do not need destructors to run.
>Queueing all unreachable memory that needs destructors to run.
>Resuming all other threads.
>Running destructors for all queued memory.
>Freeing any remaining unreachable memory.
>Returning the current thread to whatever work it was doing.

This line makes me very sad 

>Stopping all other threads than the thread currently trying to allocate GC memory

This may be a deal breaker for me. But I give it the benefit of the doubt. I am going to read though Unqiue and Refcounted and see what actually uses the GC. 
I read though 'Unique' and it is actually below 30 loc. It seems that the only 'function' that requires the GC is the destructor call '~this'.

> Error: @nogc function 'D main' cannot call non-@nogc function 'std.typecons.Unique!(Test).Unique.~this'

But I actually do not know how it even allocates, I think it still relies on the GC for that.


{% highlight d %}
struct Unique(T)
{
static if (is(T:Object))
    alias RefT = T;
else
    alias RefT = T*;

public:
/+ Doesn't work yet
    /**
    The safe constructor. It creates the resource and
    guarantees unique ownership of it (unless the constructor
    of $(D T) publishes aliases of $(D this)),
    */
    this(A...)(A args)
    {
        _p = new T(args);
    }
+/

    /**
    Constructor that takes an rvalue.
    It will ensure uniqueness, as long as the rvalue
    isn't just a view on an lvalue (e.g., a cast)
    Typical usage:
    ----
    Unique!(Foo) f = new Foo;
    ----
    */
    this(RefT p)
    {
        debug(Unique) writeln("Unique constructor with rvalue");
        _p = p;
    }
    /**
    Constructor that takes an lvalue. It nulls its source.
    The nulling will ensure uniqueness as long as there
    are no previous aliases to the source.
    */
    this(ref RefT p)
    {
        _p = p;
        debug(Unique) writeln("Unique constructor nulling source");
        p = null;
        assert(p is null);
    }
/+ Doesn't work yet
    /**
    Constructor that takes a Unique of a type that is convertible to our type:
    Disallow construction from lvalue (force the use of release on the source Unique)
    If the source is an rvalue, null its content, so the destrutctor doesn't delete it

    Typically used by the compiler to return $(D Unique) of derived type as $(D Unique)
    of base type.

    Example:
    ----
    Unique!(Base) create()
    {
        Unique!(Derived) d = new Derived;
        return d; // Implicit Derived->Base conversion
    }
    ----
    */
    this(U)(ref Unique!(U) u) = null;
    this(U)(Unique!(U) u)
    {
        _p = u._p;
        u._p = null;
    }
+/

    ~this()
    {
        debug(Unique) writeln("Unique destructor of ", (_p is null)? null: _p);
        delete _p;
        _p = null;
    }
    bool isEmpty() const
    {
        return _p is null;
    }
    /** Returns a unique rvalue. Nullifies the current contents */
    Unique release()
    {
        debug(Unique) writeln("Release");
        auto u = Unique(_p);
        assert(_p is null);
        debug(Unique) writeln("return from Release");
        return u;
    }
    /** Forwards member access to contents */
    RefT opDot() { return _p; }

/+ doesn't work yet!
    /**
    Postblit operator is undefined to prevent the cloning of $(D Unique) objects
    */
    this(this) = null;
 +/

private:
    RefT _p;
}
'
{% endhighlight %}
As you can see it steals the pointer and nulls the original one. This may cause it to be unique but it still relies on the GC for the allocation. At least that is what I think. Also there are scary amount of 'does not work yet' comments.

D seemed like a nice language for me and I did not cover all of D's features but it still seems to rely too much on the GC. I probably could reimplement many things in the standard library to not use the GC but I am not sure if want to do this right now. I probably will stay with C++.

I know many of you reading this article will think I am mad for dismissing D because of the GC and I accept your judgement. It may be completely unreasonable to avoid the GC at all costs but I don't want to work 2 years on my game just to realize in the end that the GC might become a problem that is really hard to fix.
