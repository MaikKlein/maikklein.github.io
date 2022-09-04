+++
title= "Announcing Unreal Rust"
date        = 2022-09-04
+++

![unrealrustDirkremix](https://user-images.githubusercontent.com/1994306/187069523-1690b143-f42f-4db7-8512-0ac041b363bd.png)

# Links

* [unreal rust github](https://github.com/MaikKlein/unreal-rust)
* [Devlog on Youtube](https://www.youtube.com/playlist?list=PLps1NSMUeqzicmTej83z-n1J383u1UVq1)

# Introduction

A few months ago I asked myself "What if I could write a game in Rust, but use Unreal as a renderer?". After a bit of thinking I came to the conclusion that exposing the Unreal renderer to Rust via C ffi was way more work than I was willing to do. But what if I could just build on top of Unreal instead? I could just move actors (Unreal gameobjects) around with Rust. That seemed much more manageable, so I sat down and hacked something up.

After a week I exposed a few functions that allowed me to get inputs, set and get the position of an actor. With this I could finally move a character around in Unreal 🥳.

{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187089570-557a4578-afc8-4063-b18d-3e2c2495354e.mp4")}}

While exciting, it was quite boring to watch. What if I could play animations?

I investigated how Unreal drives animations. And in this example the character here is already rigged and has animations. Those animations are driven by an `AnimationBlueprint`. All I had to do is pipe in the velocity that the character is running at, and the `AnimationBlueprint` would to the rest. I just exposed an ffi function `GetRustVelocity` and my character was running.


{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187089573-3fa0583f-707b-4b1d-8842-ff8980a23297.mp4")}}

For me that was quite magical. With almost no work, I could move a character around with Rust in Unreal.

But I wanted to do more than just moving around. I wanted to play sounds, do physics, have 3d pathfinding, spawn particles, create prefabs, do networking. This made me realize that I actually did not want to use Unreal just as a renderer, I wanted to use the whole engine. Why implement it myself if I can just expose a few functions?

And `unreal-rust` was born.

# What is `unreal-rust`?

TL;DR `unreal-rust` allows you to write games with Unreal Engine in Rust.

`unreal-rust` is an opinionated Rust integration for Unreal. Rust cares about ownership, mutability and lifetimes. Mapping Unreal concepts to Rust 1 to 1 would only cause a headache.
Instead `unreal-rust` will be written on top of the Unreal `AActor` and expose its API in a Rust friendly way.

The first big change is that `unreal-rust` will use an Entity Component System (ECS). For `unreal-rust` I decided to use [bevy](https://bevyengine.org/) instead of rolling my own. I am just a single developer and I have to pick my battles. Writing and maintaining an ECS would distract me from doing actual work. The folks at [bevy](https://bevyengine.org/) have done a wonderful job of making the ECS user friendly 🥰.

I want to deeply integrate Rust into Unreal and everything should be accessible. You can add Rust `Components` to `AActor` in the editor. For example you can add a `CharacterConfigComponent` to the `PlayerActor`, which you can then access from within Rust. This allows to configure your character from Unreal without needing to touch any code.
Additionally you can access Rust `Components` from within Blueprint. This allows you to drive Animation blueprints, or pass data into your UI.

Rust communicates with Unreal through C FFI. Its a bit out of scope for this blog post but I did a small write up to explain how it works in the [unreal rust wiki](https://github.com/MaikKlein/unreal-rust/wiki/FFI).

## Editor components and reflection

To make your `Components` visible in the editor/blueprint all you have to do is to add the `Component` derive, give it a unique UUID/GUID (_which your editor of choice can generate_) and register it with `register_components`. And if you want it to show up in the editor, you can just mark it with `#[reflect(editor)]`.

```rust
#[derive(Debug, Component)]
#[uuid = "16ca6de6-7a30-412d-8bef-4ee96e18a101"]
#[reflect(editor)]
pub struct CharacterConfigComponent {
    pub max_movement_speed: f32,
    pub gravity_dir: Vec3,
    pub gravity_strength: f32,
    pub max_walkable_slope: f32,
    pub step_size: f32,
    pub walk_offset: f32,
    pub jump_velocity: f32,
    pub max_gliding_downwards_speed: f32,
    pub gliding_gravity_scale: f32,
    pub ground_offset: f32,
}

...

impl Plugin for MovementPlugin {
    fn build(&self, module: &mut Module) {
        register_components! {
            CharacterConfigComponent,
            => module
        };
    }
}

```

The component should be immediately visible in the editor and you can add it to any actor you want. Right now sadly it only supports a few primitive types like `Vec3`, `Quat`, `f32`, `bool`, and some assets like `UClass` and `USound`. I have plans to extend it to user defined structs, hashmaps, arrays etc and also add custom drawers like having a slider for an `f32` instead of a input box.

{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187069753-ff61adf4-3b0c-4d0b-bcd9-530b92bac863.webm")}}

Now you can add any `Component` to an actor in the editor. This essentially gives you a way to have prefabs. You can create a `PlayerActor`, give it the components you want, configure it and when you place it in a level `unreal-rust` will automatically register it and add the components to the actual Rust `Entity`.

## Blueprint

`Blueprint` is Unreal's visual scripting system and it is heavily used in the engine. You can drive animations, materials, particles, sound, gameplay through it. It is important that `unreal-rust` can be used in Blueprint as well.

I added a new node called `GetComponent(Rust)`, which gives you access to all of your Rust components.

{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187077333-c8fb34e6-899e-4e65-9809-cb72c815b7a7.webm")}}


In the video above I expose a `MovementComponent` which just contains a few fields like `velocity`, `is_falling` etc and is written to by the movement system.
```rust
#[derive(Default, Debug, Component)]
#[uuid = "fc8bd668-fc0a-4ab7-8b3d-f0f22bb539e2"]
pub struct MovementComponent {
    pub velocity: Vec3,
    pub is_falling: bool,
    pub is_flying: bool,
    pub view: Quat,
}
```

We can now access the `MovementComponent` inside the animation blueprint and drive all of the animations. This is everything that is required to make the player character run, jump, and glide. The rest is handled inside the animation blueprint itself.

## Hot reloading

This allows you to see your code changes in real time without needing to restart the editor.

{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187090400-19812611-f907-434d-88cd-363e4c3903e7.mp4")}}

## Experimentation without crashing the editor

C++ has one big flaw in Unreal. You can easily crash the editor, either by triggering an assert or accessing a nullptr. This makes it quite difficult to experiment with the engine, especially if you are new. In Rust however crashes or `panic` are well defined and can be caught. That means if you ever unwrap an `Option::None` or do out of bounds access, `unreal-rust` will simply catch the panic, exit play mode and log the error to the console. It will never crash the editor.

# What is the current state?

The current state is `jank`. Almost nothing is implemented properly and you should not use `unreal-rust` in a real project yet. I will list some of the issues here.

## No stable versioning / persitence


Lets say you wrote the following component:
```rust
#[derive(Default, Component)]
#[uuid = "fc8bd668-fc0a-4ab7-8b3d-f0f22bb539e2"]
pub struct MovementComponent {
    pub gravity: f32,
}
```

Now you add this component to an actor in the editor and save it. But a few months later you realize that a simple float for the gravity is not enough and you want a direction as well. So you change the type.


```rust
#[derive(Default, Component)]
#[uuid = "fc8bd668-fc0a-4ab7-8b3d-f0f22bb539e2"]
pub struct MovementComponent {
    pub gravity: Vec3,
}
```

But now all the actors in your editor have the gravity as a `f32`, but they should become a `Vec3` instead. How do you convert those?

Next someone in your project renames `gravity` to `gravity_dir`. How do you tell the editor that this field was renamed?

Next you decide to split gravity into direction and strength
```rust
#[derive(Default, Component)]
#[uuid = "fc8bd668-fc0a-4ab7-8b3d-f0f22bb539e2"]
pub struct MovementComponent {
    pub gravity_strength: f32,
    pub gravity_dir: Vec3,
}
```

At which point do we automatically add the new `gravity_strength` field to all actors in the editor? We could do it during hot reload. You add the field, compile the code and then inside the editor we will update all of the `MovementComponents`.
But after a few minutes to decided that was a bad idea and revert the code again, and remove the `gravity_strength` field. But all of the `MovementComponents` in the editor will still have the `gravity_strength` field in them. We need a way to remove them, or you might store too much unnecessary data in your editor components.

Also the components can be accessed within blueprint:

![](https://user-images.githubusercontent.com/1994306/188315246-0ada8989-4388-443c-9c9c-2027f91f459e.png)

What if we were to remove the `is_falling` field here? Do we remove the connection from `is_falling` to `is in air?`? If we do this might break all of your bluprints, and you might have to recreate the connection in the future. If we keep the connection, the blueprint will fail to compile.

I could go on, but hopefully you get the gist. None of this is particularly difficult to solve, but it requires a bit of engineering. And before that is solved, I do not recommend to use `unreal-rust` in a real project.

I am loosely aware that Unreal Engine has similar issues and I still need to investigate how Unreal handles those cases itself.

## No serialization

Currently serialization is not implemented at all.

Usually the way to implement hot reloading is:

* Compile the code, and create a new dll
* Detect the change and initiate hot reloading
* Serialize the current gamestate
* Load the newly compiled dll
* And then restore the gamestate with the newly loaded dll

Because there is no serialization, we simply throw away the gamestate when loading the new dll.

I could simply use [serde](https://serde.rs) and call it a day. But serde does add quite a bit of compile time. I could use `bevy_reflect` and or `mini_serde` but I lose some of the niceties of serde like renaming fields and doing data upgrades.

TL;DR: I just haven't made up my mind yet of how serialization should work in `unreal-rust` and what problems it needs to solve. Losing gamestate during hot reload is not much of a blocker for myself.

## Other

* The character controller is pure jank
* A lot of missing APIs like pathfinding, networking etc
* Editor components are hacked on top of the unreal property system
* ...

# What's next?

While there are still a lot of problems, I do want to make `unreal-rust` into some thing real, it will just take a bit of time. There is an infinite todo list but the next big thing will be samples. I want to heavily drive this project through real world samples where I try to create some game mechanics like the Inscryption card game, God of War axe throwing etc.

There are a couple of reason for that:

* This allows me to prioritize which APIs I need to expose next
* I will quickly the discover pain points and can address them early
* It will make it easier for people to learn `unreal-rust` from those samples

# Closing words

If you want to keep up to date with the project you can follow me or [twitter](https://twitter.com/MaikKlein_DEV), or follow the project on [github](https://github.com/MaikKlein/unreal-rust).
If you want to try it out, I added some instruction [here](https://github.com/MaikKlein/unreal-rust#-getting-started).

Quick thank you 🥰 to:

* [kenney](https://kenney.nl/), [quaternius](https://www.patreon.com/quaternius) for providing amazing CC0 assets
* [bevy](https://bevyengine.org/) for providing the ECS
* [Dirk](https://twitter.com/noshbar) for improving Keith the C++ rat in the banner.

# Future blog posts

* Multithreading: Can we do it?
* Persistence, data upgrades
* Hybrid ECS
* Unreal <-> Rust FFI layer
* ...

# Discussions

* https://twitter.com/MaikKlein_DEV/status/1566474392342757377
* https://maikklein.github.io/unreal-rust-1/
