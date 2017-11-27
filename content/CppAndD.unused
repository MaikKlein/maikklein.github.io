+++
date        = "2016-03-01"
title       = "A comparison between C++ and D"
tags        = [ "D", "c++" ]
+++

# General

* D has `modules` which results in faster build times compared to C++. C++ might get modules after C++17. Clang and MSVC also have experimental support for `modules`.

* D supports local imports which makes it easy to move functions into different files.

~~~cpp
void foo(){
    import std.stdio: writeln;
    writeln("foo");
}
~~~


* D has integrated `unittests` and can be written directly in the source file that you want to test. This means you can write `unittests` directly under the function that you are testing.

* `class` and `struct` have a different meaning in C++ and D. In C++ they almost identical, just with different visibility defaults. In D structs can not use inheritance or interfaces. While classes can have inheritance and interfaces but lack support for deterministic destructors. A full list of differences can be found [here](https://dlang.org/spec/struct.html). Classes in D are usually dynamically allocated, but they can also be allocated on the stack. Classes are by default reference types. If `T` is a class then it implicitly is `T*`.

* D comes with a default GC while C++ is GC free. D can also be used without a GC but there are a few inconveniences. First the standard library(phobos) in D is not move aware. This means you can not have a `std::vector<std::unique_ptr>` in phobos. It is possible to write your own containers that are move aware which means it is possible to have an array or vector with unique pointers.

* Moving in C++ is just an rvalue cast while in D it really moves. In C++ you would write a function `template<class T> void foo(T&& t){}`, it captures `t` as `T&&` if `t` is an rvalue or it captures `t` as `T&` if `t` is an lvalue. You then use `std::move` with a `move constructor` to move your objects and `std::forward` to perfectly forward your objects. In D you would create two functions `void foo(T)(ref T t){}` which always captures lvalues by references and `void foo(T)(T t){}` which only captures rvalues. As far as I know moving in D is also not exception safe. D moves objects with a bitwise copy, this means you are not allowed to have internal pointers.

* `structs` in D don't have a default constructor because every struct needs exception free default construction and this must be known at compile time. But it is possible to initalize structs with custom values.

~~~cpp
struct Foo{
  int i = 42;
}
~~~

* Variables in D are always initialized unless explicity told not to `Foo f = void;`. Initialization rules are bit more complicated in C++ and depend on the context.


* D as well as C++ can disable default construction, copy construction and copy assignment. Note that in D it is still possible to call `T.init` even if the default constructor is disabled.

* In C++ local references can escape the scope while in D they can't.

* The allocator in C++ is a template argument (at least in the STL) while in D it can be changed at runtime. This means that you can have different allocators inside an array. You can find more information [here](https://dlang.org/phobos/std_experimental_allocator.html). The allocator in D is still experimental.

* C++ needs explicit function specifiers such as `noexcept` `const` `constexpr` while in D they are inferred if the function/method is a template. This means that `void foo(T)(...)nothrow, const, pure, @nogc{}` can just be `void foo(T)(...)`.

* Functions and methods in D can be called without parenthesis if they have no arguments. `void foo(){}; foo;`

* C++ as well as D have anonymous functions. C++: `[](auto a, auto b){ return a + b;}` , D: `(a, b) => a + b` or `(a, b){return a + b;}`. As far as I know capturing other variables requires the GC in D. In C++ you can explicitly capture variables by copy, ref or move. Lambda functions in D can not return references. C++17 will also make lambda functions available with `constexpr`. Lambda functions can also be used at compile time in D.

* Unlike in C++ the order of declarations doesn't matter in D.

* D has built in documentation comments. In C++ you have to use an external tool such as doxygen.

* D has `alias this` which makes composition of types without inheritance really easy.

~~~cpp
struct Foo{
  Bar bar;
  alias bar this;
}
~~~

The code above forwards all methods and members from `Bar` to `Foo` and makes `Foo` implicitly convertible to `Bar`. The implicit conversion can be removed by using `Proxy` instead of alias this.

* Operator of overloading in C++ `Foo Foo::operator+(Foo const& foo){}`. Operator overloading in D `Foo opBinary(string op)(in Foo f)`. This allows the [mixin macros pattern](http://wiki.dlang.org/Mixin_Macros_Pattern) which can remove a lot of boilerplate code for you.

* D has universal function call syntax (ufcs). This just means that functions can also be called like methods `foo(bar)` or `bar.foo()`. This is similar to extension methods in C#. C++17 might also get some form of ufcs.

* C++ has `user defined literals` like `1_seconds`. D doesn't have this feature but it can be emualted with ufcs `1.seconds`.

* Conditonal compilation uses the pre-processor in C++ `#if, #elif, #else, and #endif Directives`. In D it is `version(YourKeyword){...}`.

* Exceptions in D can be allocated by the GC with `new` but you can also allocate the storage for an exception yourself.

* C++ has `namespaces` and are used like this `namespace Foo{ namespace Bar{ namespace Baz{..}}}`. D uses modules with a file structure. To get `foo.bar.baz` you can create `baz.d` inside the `bar` folder and `bar` inside the `foo` folder.

* Globals in D are only thread local by default unless they are immutable. To get thread safe global access you would mark the global variable as `shared`. To get the same global variables as in C++ you would use `__gshared`.

* `const` in D is [transitive](https://dlang.org/const-faq.html#transitive-const). In D and C++ it is `undefined behaviour` to cast away the `const` and modify the object.

# Meta programming

* It is possible to pass almost any value to a template in D. C++ is limited to integrals, chars and addresses of functions and symbols.

* Template instantiations are done in C++ with `<Foo,Bar>` and in D with `!(Foo,bar)`.

* D has no fold expressions like C++ `foo(f(args)...);`, although they can be implemented as a library.

* D can evaluate almost any function at compile time with very few restrictions. One restriction would be casting see [answer](http://stackoverflow.com/a/35701007/944430) for more information where in C++ those functions need to be marked as `constexpr` and have many restrictions such as no allocations.

* C++ can have multiple variadic templates `template<class... As, class... Bs>` but they need to be inferred. This is not possible in D but you can have templates of templates `contains!SomeTypes.all!SomeOtherTypes`.

* D can generate strings at compile time and compile them at compile time with `mixin`, this is not possible in C++.

* C++ has macros while D does not. `mixins` are capable of replacing macros.

* D has static reflection/introspection, this feature might come to C++ after C++17. This can currently be emulated to some extent with `libclang`.

* D also has `user defined attributes` `@Encrypted string name;` which can be used by D's static introspection. C++17 gets `user defined attributes` in the form of `[[YourKeyWord]]`

* D can print any type at runtime with `writeln(SomeType.stringof)` or `writeln(typeof(somevar).stringof)`. Anything that is available at compile time can be printed at compile time with `pragma(msg, SomeType)`. This is very useful for debugging meta programs. C++ can only do this with some compiler hacks as far as I know.

* It is possible to pass `symbols` to templates in D. `template Foo(alias someSymbol)` This just means that it is possible to pass anything that is available at compile time into `Foo`. Examples would be other templates, functions, lambdas, constants etc. This is similar to Lisp.

* D has `static if`. It is possible to use template specialization in C++ to achieve something similar.

* *Unsure, need more benchmarks* Compile times for meta programming seems to be roughly equivalent between C++ and D.

* In D it is possible to iterate over variadic types with a foreach loop. In C++ this can be achieved as a library, for example with [Boost hana](https://github.com/boostorg/hana).
~~~d
foreach(index, type; VaradicTypes){
}
~~~

* Variadics in D are automatically expanded while in C++ you expand them explicitly with fold expressions.

* Variadics in D can be sliced `VariadicTypes[1 .. $]` this creates a variadic type list without the first type.
