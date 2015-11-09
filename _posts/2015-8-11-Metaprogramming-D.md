---
layout: post
title: "Metaprogramming in D - From a beginner's perspective"
description: ""
category: 
tags: []
---

{% highlight d %}
Disclaimer: I have only started learning D a few days ago and I wanted to document my experience.
{% endhighlight %}

Let us create a small gameplay framework that can update `GameObjects`. Sometimes you will see that `GameObjects` are stored in a vector using some kind of pointers.

{% highlight d %}
std::vector<GameObjects*>;
{% endhighlight %}

There are a few problems with this code, one problem would be that only the pointers are stored contiguously in memory. This could lead to a memory fragmentation problem which could be solved by using a custom allocator. Another problem is that only the pointers are prefetched and not the objects themselves which might result in a performance penalty.

We are going to address this problem in D with metaprogramming. Every `GameObject` will get it's own container.

{% highlight d %}
struct Engine{
  std::vector<Monster> monsters;
  std::vector<Player> players;
  ...
  void updateAll(){
    for(auto& player: players){
      player.update();
    }
    for(auto& monster: monsters){
      monster.update();
    }
    ...
  }
};
{% endhighlight %}
You could do this by hand, but it would be very tedious and error prone. This could be easily implemented in C++ but from now on we will make use of D's metaprogramming features. Don't worry if you don't understand every thing, I will go over everything in detail.
{% highlight d %}
class Engine(alias Container, GameObjects...){
  alias GameObjectsContainer = staticMap!(Container, GameObjects);
  GameObjectsContainer gameObjectsContainer;

  ref Container!T getContainer(T)(){
    enum indexOfT = IndexOf!(T, GameObjects);
    return gameObjectsContainer[indexOfT];
  }

  ref T getGameObject(T)(Handle!T handle){
    return getContainer!T()[handle.id];
  }

  Handle!T spawn(T)(T t){
    return Handle!T(getContainer!T().insertBack(t) - 1);
  }

  void updateAll(){
    foreach(container;gameObjectsContainer){
      foreach(gameobject;container){
        gameobject.update();
      }
    }
  }
}
struct Handle(T){
  size_t id;
  this(size_t id){
    this.id = id;
  }
}
{% endhighlight %}
It can be used like this:
{% highlight d %}
struct Monster{
  string name;
  this(string name){
    this.name = name;
  }
  void update(){
    writeln("Updating Monster: ", name);
  }
}
struct Player{
  string name;
  this(string name){
    this.name = name;
  }
  void update(){
    writeln("Updating Player: ", name);
  }
}
void main()
{
  alias GameObjects = AliasSeq!(Monster, Player);
  alias ArrayEngine = Engine!(Array, GameObjects);
  auto engine = new ArrayEngine;
  auto monster1Id = engine.spawn(Monster("Monster1"));
  auto player1Id = engine.spawn(Player("Player1"));
  auto monster2Id = engine.spawn(Monster("Monster2"));
  auto player2Id = engine.spawn(Player("Player2"));
  
  writeln(monster1Id);
  Monster monster1 = engine.getGameObject(monster1Id);
  engine.updateAll();

}
{% endhighlight %}
Output:
{% highlight d %}
Updating Monster: Monster1
Updating Monster: Monster2
Updating Player: Player1
Updating Player: Player2
{% endhighlight %}
I am aware that this code won't win any awards, I tried to keep it simple and short and don't go into specifics like invalidating the handles.

{% highlight d %}
class Engine(alias Container, GameObjects...){
  alias GameObjectsContainer = staticMap!(Container, GameObjects);
  GameObjectsContainer gameObjectsContainer;
{% endhighlight %}

This is the heart of our `Engine` class and it makes use of type-level metaprogramming. Everything in the parentheses are `templates` while `Container`is a symbol and `GameObjects...` is a variadic template.

{% highlight d %}
(alias Container, GameObjects...)

{% endhighlight %}
`Container` will be our type function which will transform a type into a container of that type. For example it will transform from `T to Container!T`. Note the `!` in D is a template invocation. In C++ you would use `<>`.
`map` is usually well know to functional programmers, it takes a list and a function and applies that function to every element of that list. `staticMap` is a `map` for types and type values.

{% highlight d %}
  alias GameObjectsContainer = staticMap!(Container, GameObjects);
{% endhighlight %}
If we call `staticMap` with `AliasSeq!(Monster, Player, Weapon)` as our list of types and `Container` as the type level function. The resulting type would be `AliasSeq!(Container!(Monster), Container!(Player), Container!(Weapon))`

D has a thin wrapper for variadic sequences called `AliasSeq`. If you are coming from C++ this would roughly be
{% highlight d%}
template<class ...Ts>
struct AliasSeq{};
{% endhighlight %}

We now have a small problem, we can not access `gameObjectsContainer` directly and we need to create a small helper function.
{% highlight d %}
ref Container!T getContainer(T)(){
  enum indexOfT = IndexOf!(T, GameObjects);
  return gameObjectsContainer[indexOfT];
}
{% endhighlight %}
`IndexOf` is also a metafunction that will take a type and a type sequence and will return the index to the first occurrence of the type in the type sequence. After we have obtained the index we are able to access the container that we want with `gameObjectsContainer[indexOfT]`

Spawning `GameObjects` is very simple.
{% highlight d %}
Handle!T spawn(T)(T t){
  return Handle!T(getContainer!T().insertBack(t) - 1);
}
{% endhighlight %}
We instantiate a `GameObject` and pass it to the spawn function. The spawn function knows the type of the `GameObject` which allows us to access the right container with `getContainer`. After that we just insert the `GameObject` at the end of the container. We then wrap the index in a `Handle!T`, which prevents accidental access of a wrong container.

{% highlight d %}
ref T getGameObject(T)(Handle!T handle){
  return getContainer!T()[handle.id];
}
{% endhighlight %}
Because a `Handle` stores the type of the `GameObject` we can just use the type to access the right container and retrieve the our `GameObject`.
{% highlight d %}
void updateAll(){
  foreach(container;gameObjectsContainer){
    foreach(gameobject;container){
      gameobject.update();
    }
  }
}
{% endhighlight %}
The first forloop loops over all containers at compile time, the second for loop will update every `GameObject` at run time.

We are now storing every `GameObject` in its own container which gives us several benefits. All `GameObjects` are stored contiguously in memory without any indirection and we have now a lot of type information. It's is now very simple to loop over all `Monsters`, or maybe you want to apply damage to nearby players.



Usually when I learn a new language I also try to come up with some strange ideas to see how the language will behave with not so common problems.

`Goal:` Create a function that takes a variable length of arguments and creates the sum of all integers and the sum of all floats.

Example:
{% highlight d %}
writeln(sumIntFloat(1, 2.0f, 3, 4.0f, 5.0f, 6, 7));
//Tuple!(int, float)(17, 11)
{% endhighlight %}

The first version of this function is rather elegant:
{% highlight d %}
enum isIntegral(alias i) = std.traits.isIntegral!(typeof(i));
enum isFloatingPoint(alias f) = std.traits.isFloatingPoint!(typeof(f));
Tuple!(int, float) sumIntFloat(Ts...)(Ts ts){
  int intSum = Filter!(isIntegral, ts)
    .only()
    .reduce!((a,b) => a + b);
  float floatSum = Filter!(isFloatingPoint, ts)
    .only()
    .reduce!((a,b) => a + b);
  return tuple(intSum, floatSum);
}
{% endhighlight %}

`isIntegral` will take a value and check with `typeof` if it is an integer. `Filter` is a metafuction that will filter our variadic sequence `Ts...`. Combining `Filter` with `isIntegral` will filter the sequence `Ts...` so that it will only contain integer values. `only()` transforms the sequence to a [range](http://dlang.org/phobos/std_range.html) which allows us to reuse functions from [std.algorithm](http://dlang.org/phobos/std_algorithm.html) like `reduce`.

I think it looks quite elegant but sadly `only()` will result in a copy. Let us see if we can improve on that.

Before we start, let us generalize our isIntegral and isFloatingPoint "functions".

{% highlight d %}
static template Is(T){
   enum SameAs(alias t) = is(T == typeof(t));
}
{% endhighlight %}
We can call it like this
{% highlight d %}
int i;
float f;
Is!float.SameAs(f);
Is!int.SamAs(i);
{% endhighlight %}

Sequences can be wrapped in an `AliasSeq` as we have seen before.

{% highlight d %}
alias IntFloatString = AliasSeq!(int, float, string);
{% endhighlight %}

But it is also possible to actually pass values into templates

{% highlight d %}
alias IntFloatString = AliasSeq!(1, 2.0f, "Hello World");
{% endhighlight %}

We need to create a new functon that takes a sequence and produces a value, just like `reduce`.

{% highlight d %}
template staticFold(alias Func, alias B, Ts...){
  static if(Ts.length == 0){
    alias staticFold = B;
  }
  else{
    alias staticFold = staticFold!(Func, Func!(B,Ts[0]), Ts[1..$]);
  }
}
{% endhighlight %}
Personally I think that the `[1..$]` is rather elegant and will simplify the code greatly compared to C++ where you have to great another template.

It can be used like this:
{% highlight d %}
template Sum(alias A, alias B){
  alias Sum = AliasSeq!(A + B);
}
template SumIntFloatV2(Ts...){
  alias IntSum   = staticFold!(Sum, 0, Filter!(Is!int.SameAs, Ts));
  alias FloatSum = staticFold!(Sum, 0.0f, Filter!(Is!float.SameAs, Ts));
}
void main()
{
  alias IntFloat = SumIntFloatV2!(1, 2.0f, 3, 4.0f, 5.0f, 6, 7);
  writeln(IntFloat.IntSum);
  writeln(IntFloat.FloatSum);
}
{% endhighlight %}

This doesn't look so bad either, but there are also a few problems. First, we have to write our `Sum` function as a template and the second problem is that we can only use this at compile time.

But one of D's features is to be able to evaluate almost any function at compile time so it should be possible to create a function that works at compile time as well as at run time.

This led to me ask a [question](http://stackoverflow.com/a/33588960/944430) on StackOverflow.

{% highlight d %}
struct Answer {
        int IntSum = 0;
        float FloatSum = 0.0;
}
Answer SumIntFloatV4(Ts...)(Ts ts) {
        Answer a;
        foreach(t; ts) {
                static if(is(typeof(t) == int))
                        a.IntSum += t;
                else static if(is(typeof(t) == float))
                        a.FloatSum += t;
        }

        return a;
}

void main() {
        // compile time
        pragma(msg, SumIntFloatV4(1,1.0,2,3,2.0,3.0));
        pragma(msg, SumIntFloatV4(1,1,2,3,1.0,2.0));
        // runtime
        import std.stdio;
        writeln(SumIntFloatV4(1,1.0,2,3,2.0,3.0));
        writeln(SumIntFloatV4(1,1,2,3,1.0,2.0));
}
{% endhighlight %}
I knew before hand that I could do it like this but in my opinion this looks much worse compared to version 1 and 2. I really wanted to reuse staticFold in my run time version but it didn't seem possible and I almost quit.

Luckily I went back to the drawing board and redefined what my goal was.

`Goal:` Generate an efficient function at compile time that can be called at run/compile time.

With a much better defined goal I was able to create a new version of `staticFold`.

{% highlight d %}
B fold(F, B, Ts...)(F f, B init, Ts ts){
  static if(ts.length == 0){
    return init;
  }
  else{
    return fold(f, f(init, ts[0]), ts[1..$]);
  }
}
{% endhighlight %}

{% highlight d %}
Tuple!(int, float) sumIntFloatV3(Ts...)(Ts ts){
  int intSum = fold((int a, int b) => a + b
                   ,0, Filter!(Is!int.SameAs, ts));
  float floatSum = fold((float a, float b) => a + b
                   ,0.0f, Filter!(Is!float.SameAs, ts));
  return tuple(intSum, floatSum);
}
void main()
{
  writeln(sumIntFloatV3(1, 2.0f, 3, 4.0f, 5.0f, 6, 7));
}
{% endhighlight %}
The nice thing about `fold` is that it can be called with almost anything.

{% highlight d %}
void main()
{
  writeln(sumIntFloatV3(AliasSeq!(1, 2.0f, 3.0f)));
  auto t = tuple(1, 2, 3, 4.0f, 5.0f);
  writeln(sumIntFloatV3(t.expand));
  int i = 5;
  int i2 = 10;
  float f = 5.0f;
  float f2 = 10.0f;
  writeln(sumIntFloatV3(i, f, i2, f2));
}
{% endhighlight %}
I have experimented a lot metaprogramming in various languages. For example macros, AST manipulation, two phase compilation with substitution and I think that template metaprogramming in D is rather elegant.
