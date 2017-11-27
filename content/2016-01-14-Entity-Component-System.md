+++
date        = "2016-01-14T11:27:27-04:00"
title       = "The general design of my flawed compile time entity component system in C++14"
tags        = [ "ecs", "c++" ]
+++

A lot of people are talking about entity component systems and how they could potentially improve your code base. Most of the articles are theoretical and it is hard to find examples that actually convey some advantages. 

There are actually a lot of entity component systems available for many different programming languages, but most seem to sacrifice performance for expressiveness. 

But I am curious and I really like the general idea of an entity component system but I don't want to just accept the opinion of other game developers. It's time to get some actual experience and I have implemented a very small entity component system in C++14 with template metaprogramming.

This post will be about the general design of my entity component system.

## Features

The core features of my entity component system are:

```cpp
Contiguous memory layout
Components are grouped at compile time
Compile time filtering of components
Components are always added in the correct group automatically
```

## Overview


```cpp
struct world{
  //component_group 'moveable_object'
  std::vector<position> moveable_object_position;
  std::vector<velocity> moveable_object_velocity;

  //component_group 'entity'
  std::vector<position> entity_position;
  std::vector<velocity> entity_velocity;
  std::vector<name> entity_name;
  ...
};
```

This is the core of the entity component system, we group components together at compile time. The layout is called `SoA` (structures of arrays) and a specific object is referred to by id.

For example instead of having 
```cpp
struct moveable_object{
  velocity vel;
  position pos;
};
std::vector<moveable_object> mv_objects;
```
we have
```cpp
std::vector<position> moveable_object_position;
std::vector<velocity> moveable_object_velocity;
```
And I refer to `moveable_object_position[index], moveable_object_velocity[index]` as an object in this post.

Each `component_group` has a list of constrains, for example an `entity` needs to have a `position`, `velocity` and `name` component. The advantage of this approach is that filtering is essentially free, because we can just iterate over `entity_position`,`entity_velocity` and`entity_name` at the same time. Another advantage is that we don't have to worry about the memory layout as much, because we group everything at compile time.

It is also possible to filter `component_groups` based on the components they own. For example if we want to iterate over every `object` that has a `position` and `velocity` component, we would have to iterate over `entity_position`, `entity_velocity` and `moveable_object_position`, `moveable_object_velocity`.

Obviously maintaining something like this by hand is extremely tedious. This is where metaprogramming will help us to convey our meaning. The following code snippet is a small example from my experimental entity component system.

```cpp
struct position {
  float x, y;
};
struct velocity {
  float x, y;
};
struct name {
  std::string name;
};

struct print_name {
  template <class World> void update(World& w) {
    w.template update<name>([](auto& name) { print(name.name); });
  }
};
struct print_pos_vel {
  template <class World> void update(World& w) {
    w.template update<position, velocity>([](auto& pos, auto& vel) {
      print(pos.x, pos.y);
      print(vel.x, vel.y);
    });
  }
};
auto make_monster(float x, float y, std::string n) {
  return hana::make_tuple(position{ x, y }, velocity{ 0, 0 }, name{ n });
}

template <class... Ts>
using component_group_handle = ecs::core::component_group<
    breeze::util::container::handle_container, Ts...>;

int main() {
  using moveable_object = component_group_handle<position, velocity>;
  using entity = component_group_handle<position, velocity, name>;
  using cg = ecs::core::component_groups<moveable_object, entity>;
  auto w = ecs::core::world<cg>{};
  auto sg = ecs::core::make_systems_group(w, print_name{}, print_pos_vel{});
  w.add(make_monster(10, 5, "Monster3"));
  w.add(make_monster(11, 5, "Monster2"));
  w.add(position{ 42, 24 }, velocity{ 5, 5 }, name{ "Monster1" });

  // moveable_object
  w.add(position{ 0, 0 }, velocity{ 10, 5 });
  sg.update();
}
```

Don't worry if you don't understand what is going on, I'll try to explain everything you need to know in the following section.


```cpp
struct print_pos_vel {
  template <class World> void update(World& w) {
    w.template update<position, velocity>([](auto& pos, auto& vel) {
      print(pos.x, pos.y);
      print(vel.x, vel.y);
    });
  }
};
```

This is a simple system, which filters out every `component_group`that does not have a `position` and `velocity` component, it then extracts the correct components for every `component_group` that we are interested in. We then zip the iterators for `position` and `velocity` together and we repeat this for every `component_group` that satisfies the `position` and `velocity` constrain. We are then left with a tuple of zipped iterators which we
concatenate into one big iterator. After that we can iterate over every `position` and `velocity` component and print it to the console.

The only thing that is done at runtime is accessing the iterator for every component and concatenate them.

```cpp
using moveable_object = component_group_handle<position, velocity>;
using entity = component_group_handle<position, velocity, name>;
```

These are our `component_groups` which we have to declare once. Technically those could be inferred but I decided that it would serve as a good way to document all the different `component_groups` that are used by the entity component system.
```cpp
using cg = ecs::core::component_groups<moveable_object, entity>;
auto w = ecs::core::world<cg>{};
```

After declaring all the `component_group`'s that we want to use, we can finally create our `world` object.

The world object itself doesn't do much and its purpose is to store our data. 

```cpp
auto sg = ecs::core::make_systems_group(w, print_name{}, print_pos_vel{});
...
sg.update();
```

A `system_group` just groups different systems together and allows to call `update` on every system that has been registered at compile-time.

```cpp
// entity
w.add(position{ 42, 24 }, velocity{ 5, 5 }, name{ "Monster1" });

// moveable_object
w.add(position{ 0, 0 }, velocity{ 10, 5 });
```
Every `object` is added into the correct `component_group`. If you call add with `<position, velocity>` it will look at all the `component_group`'s and that were declared before and insert it into the correct one.
Because we have declared `using moveable_object = component_group_handle<position, velocity>;`  the components `w.add(position{ 0, 0 }, velocity{ 10, 5 });` will be inserted into the `component_group` `moveable_object`.

Deleting `objects` is also very simple. Because we have grouped everything at compile-time, we just have to look at the `component_group` that the object belongs to and delete it. To preserve contiguous elements we swap the last element with the element that we want to delete and then we simply remove the last element.

Most entity component system allow to add and remove components from `objects` at runtime. If we want to take a `moveable_object` and add a `name` component, it would become an `entity`.

We would need to move the components from the `component_group` `moveable_object` to the `component_group` `entity` and add then simply add the `name` component.


## Dealbreaker

The biggest problem is the compilation time. Every query of update `w.template update<T0,T1,...TN>` costs around 2 seconds on my machine, that is just unacceptable.

You can find the code [here](https://github.com/BreezeEngine/breeze/blob/master/src/ecs/core/core.hpp) and [here](https://github.com/BreezeEngine/breeze/blob/master/examples/ecs/main.cpp)

Please note that the code is in a highly experimental state and will most likely not even compile on your machine.

## What have I learned?
I really love metaprogramming in C++ with [hana](https://github.com/boostorg/hana). It is expressive, concise and doesn't look odd if you are familiar with functional conecpts.

Compile errors are horrifying and tools for compile-time debugging/profiling are pretty much non existent.

Compile times can quickly blow up and it's hard to track down where and why they are blowing up.

I don't think the entity component system that I envision is practical to create in C++ and I will most likely abandon it. I am not quite ready the abandon the idea and I will try to create a similar entity component system in `D`.
