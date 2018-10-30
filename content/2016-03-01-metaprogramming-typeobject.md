+++
date = 2016-03-01
tags = []
title = "Metaprogramming with type objects in D"
+++

I am going to introduce you to `Type Objects` and why they might be useful in combination with metaprogramming.
This blog post is heavily inspired by [Boost Hana](https://github.com/boostorg/hana).

Let us start with an example. If we would want to implement an `Algebraic Data Type` we would need to find out the maximal size of the types it should contain. In D that is pretty easy
```d
import std.meta: AliasSeq;
import std.traits: Largest;
alias Types = AliasSeq!(int, float, char, double);
enum maxSize = Largest!(Types).sizeof;
writeln(maxSize);//8 bytes
```


Largest is implemented like this
```d
template Largest(T...) if(T.length >= 1)
{
    static if (T.length == 1)
    {
        alias Largest = T[0];
    }
    else static if (T.length == 2)
    {
        static if(T[0].sizeof >= T[1].sizeof)
        {
            alias Largest = T[0];
        }
        else
        {
            alias Largest = T[1];
        }
    }
    else
    {
        alias Largest = Largest!(Largest!(T[0 .. $/2]), Largest!(T[$/2 .. $]));
    }
}
```

But if `Largest` wouldn't be available, it would be quite annoying to always create a new template. Alternatively we could implement `Largest` like this

```d
import std.meta: AliasSeq, staticMap;
import std.traits: Largest;
import std.algorithm.comparison: max;
alias Types = AliasSeq!(int, float, char, double);
enum size(T) = T.sizeof;
enum maxSize = max(staticMap!(size, Types));
writeln(maxSize);//8 byte
```

Not much worse because we can reuse the standard library but we also had to create a new template to convert types into sizes. This is a general pattern in metaprogramming. If you want to do type level metaprogramming you have to use templates but wouldn't it be nice if we could use ordinary functions?


Instead of doing type computations with templates we will create `Type Objects`.

```d
import std.traits: isInstanceOf;
enum isType(T) = isInstanceOf!(Type, T);
struct Type(T){
    alias type = T;
    string toString()
    {
        return "Type!("~T.stringof~")";
    }
}
```

```d
enum t  = Type!(int)();
enum t1 = Type!(string)();
```

You may wonder why we do this. The answer is simple, we now can use `Types` like ordinary objects. Let us create a simple `equals` function that checks if two `Types` are actually the same.
```d
enum equals(A,B)(Type!A, Type!B){
    return is(A == B);
}
enum t  = Type!(int)();
enum t1 = Type!(string)();

writeln(t.equals(t1)); // false
writeln(equals(t, t1)); // false
writeln(equals(Type!int(), Type!int())); // true
```
Types itself are not that interesting, let us create a `TypeTuple` that can hold any number of `Types`.
```d
enum isTypeTuple(T) = isInstanceOf!(TypeTuple, T);

struct TypeTuple(Types...){
    import std.meta: allSatisfy;
    static assert(allSatisfy!(isType, Types), "Variadic parameters need to be of type 'Type!'");
    Types expand;
    alias expand this;
    string toString()
    {
        import std.range;
        string[] s;
        foreach(t; expand){
            s~= t.toString();
        }
        return "TypeTuple!(" ~ s.join(", ") ~")";
    }
}
enum typeTuple(Types...)(Types){
    return TypeTuple!Types();
}
```
By the way you may wonder why we created a `toString` method here. This is because `D` sometimes doesn't like to print types that were generated and it might not print the actual type and inserts some pseudo symbols like `F!int` instead of `Type!int`.

We can use `TypeTuple` like this:
```d
enum types = TypeTuple!(Type!int, Type!char, Type!float, Type!double)();
//or
enum types = typeTuple(Type!int, Type!char, Type!float, Type!double);
```
It is just a bit of boilerplate which we can easily avoid with a helper function
```d
enum tupleFromTypes(Ts...)(){
    import std.meta: staticMap;
    return TypeTuple!(staticMap!(Type, Ts))();
}
enum types = tupleFromTypes!(int,double, int,float);
```
Now we can start to implement some nice metafunctions. We start by implementing `filter`.
```d
enum filter(alias f, Tup)(Tup){
    static assert(isTypeTuple!(Tup), tup.stringof~" is not a TypeTuple.");
    enum tup = Tup();
    static if(tup.length == 0){
        return typeTuple();
    }
    else static if(f(tup[0])){
        return typeTuple(tup[0], filter!(f)(typeTuple(tup[1..$])).expand);
    }
    else{
        return filter!(f)(typeTuple(tup[1..$]));
    }
}
```
We can now use it like this:

```d
enum types = tupleFromTypes!(int, double, string, float);
enum biggerThan4 = filter!(t => t.type.sizeof > 4)(types);
writeln(biggerThan4);// TypeTuple!(Type!(double), Type!(string))
```

The code above filters our `TypeTuple` with an ordinary lambda function. The resulting `Types` must be bigger than 4 bytes. We can also implement `map`
```d
enum map(alias f, Tup)(Tup){
    static assert(isTypeTuple!(Tup), tup.stringof~" is not a TypeTuple.");
    enum tup = Tup();
    static if(tup.length == 0){
        return typeTuple!();
    }
    else{
        return typeTuple(f(tup[0]), map!(f)(typeTuple(tup[1..$])).expand);
    }
}
```
```d
enum types = tupleFromTypes!(int, double, string, float);
enum onlyInts = map!(t => Type!int())(types);
writeln(onlyInts);// TypeTuple!(Type!(int), Type!(int), Type!(int), Type!(int))
```
We replace every `Type` with `Type!int` which is probably not that useful. We could also replace only types that are actually bigger than 4 bytes with `Type!int`.
```d
enum types = tupleFromTypes!(int, double, string, float);
enum smallerThan5 = map!((t){
    static if(t.type.sizeof > 4){
        return Type!int();
    }
    else{
        return t;
    }
})(types);// TypeTuple!(Type!(int), Type!(int), Type!(int), Type!(float))
writeln(smallerThan5);
```
The possibilities are almost endless and you can use it like any other function. Just remember that it needs to be executed at compile time which means you have to use `static if` instead of an ordinary `if`.
```d
enum indexOf(T,Tup)(T, Tup){
    static assert(isTypeTuple!(Tup), tup.stringof~" is not a TypeTuple.");
    static assert(isType!(T), T.stringof~" is not a Type.");
    enum t = T();
    enum tup = Tup();
    foreach(index, type; tup.expand){
        if(type.equals(t)){
            return index;
        }
    }
    return -1;
}
enum types = tupleFromTypes!(int, double, string, float);
enum index = indexOf(Type!string(), types);
writeln(index);// 2

```
And last but not least, we can implement quicksort for types.
```d
enum partition(alias f, Tup)(Tup){
    enum tup = Tup();
    return partitionImpl!(f)(tup, typeTuple(), typeTuple());
}

enum partitionImpl(alias f, Tup, TupLeft, TupRight)(Tup, TupLeft, TupRight){
    import std.typecons: tuple;
    enum tup = Tup();
    enum l = TupLeft();
    enum r = TupRight();

    static if(tup.length == 0){
        return tuple(l, r);
    }
    else{
        static if(f(tup[0])){
            return partitionImpl!(f)(typeTuple(tup[1..$]), typeTuple(tup[0], l.expand), typeTuple(r.expand));
        }
        else{
            return partitionImpl!(f)(typeTuple(tup[1..$]), typeTuple(l.expand), typeTuple(tup[0], r.expand));
        }

    }
}

enum sort(alias f,Tup)(Tup){
    enum tup = Tup();
    static if(tup.length == 0){
        return typeTuple();
    }
    else static if(tup.length == 1){
        return typeTuple(tup[0]);
    }
    else{
        enum middle= tup[0];
        enum t = partition!(t => f(t, middle))(typeTuple(tup[1..$]));
        enum left = t[0];
        enum right = t[1];
        return typeTuple(left.expand, middle, right.expand);
    }
}
```
I needed to create another helper function `partition` which just splits a `TypeTuple` into two `TypeTuples` based on a predicate.
```d
enum types = tupleFromTypes!(int, double, string, float);
enum sortedTypes = sort!((t1, t2) => t1.type.sizeof > t2.type.sizeof)(types);
writeln(sortedTypes);// TypeTuple!(Type!(string), Type!(double), Type!(int), Type!(float))
```
Now it is also trivial to get the maximum size
```d
enum types = tupleFromTypes!(int, double, string, float);
enum sortedTypes = sort!((t1, t2) => t1.type.sizeof > t2.type.sizeof)(types);
enum maxSize = sortedTypes[0].sizeof;
writeln(maxSize);// 8 bytes
```

`Type Objects` allow metaprogramming to look like normal functional programming just with types.
