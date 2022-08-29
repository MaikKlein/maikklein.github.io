+++
title= "Unreal Rust"
date        = 2022-08-28
+++




![unrealrustDirkremix](https://user-images.githubusercontent.com/1994306/187069523-1690b143-f42f-4db7-8512-0ac041b363bd.png)
# Introduction

"What if I could write a game in Rust, but use Unreal as a renderer?". After a bit of thinking I came to the conclusion that exposing the Unreal renderer to Rust via C ffi was way more work than I was willing to do. But what if I could just build on top of Unreal instead? I could just move actors (Unreal gameobjects) around with Rust. That seemed much more managable, so I sat down and hacked something up.

After a week I exposed a few functions that allowed me to get inputs, and set and get the position of an actor. With this I could finally move around a character inside Unreal 🥳.

{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187089570-557a4578-afc8-4063-b18d-3e2c2495354e.mp4")}}

While exciting, it was quite boring to watch. What if I could play animations? I investigated how Unreal drives animations. And it this example the character here is already rigged with animations, and those animations are driven by an `AnimationBlueprint`. All I had to do is pipe in the velocity that the character is running at, and the `AnimationBlueprint` would to the rest. I just exposed an ffi function `GetRustVelocity` and my character  was running.


{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187089573-3fa0583f-707b-4b1d-8842-ff8980a23297.mp4")}}

For me that was quite magical. With almost no work, I could move a character around with Rust in Unreal. At that time I also had proper hot reloading, so I could easily make changes while playing in the editor. Another realization was that I could not crash the edtior from within Rust at all, that made experimenting much more fun. But more on that later.

But I wanted to do more than just moving around. I wanted to play sounds, do physics, have 3d pathfinding, spawn particles, create prefabs, do networking. This made me realize that I actually did not want to use Unreal just as a renderer, I wanted to use the whole engine. Why implement it myself if I can just expose a few functions?

And `unreal-rust` was born.

(I also recorded a bunch of videos during development and created a [playlist on youtube](https://www.youtube.com/playlist?list=PLps1NSMUeqzicmTej83z-n1J383u1UVq1))

# What is `unreal-rust`?

TL;DR `unreal-rust` allows you to write games with Unreal Engine in Rust.

`unreal-rust` is an opinionated Rust integration for Unreal. Rust cares about ownership, mutability and lifetimes. Mapping Unreal concepts to Rust one to one would only cause a headache.
Instead `unreal-rust` will be written on top of the Unreal `AActor` and expose its API in a Rust friendly way.

The first big change is that `unreal-rust` will use an Entity Component System (ECS). For `unreal-rust` I decided to use [bevy](https://bevyengine.org/) instead of rolling my own which allows me to focus more on the Unreal part. I might want to fork `bevy-ecs` in the future to adapt it my needs, but I am very happy with it so far. The folks at [bevy](https://bevyengine.org/) have done a wonderful work making the ECS user friendly.

## Editor components and reflection

You can just add the `Component` derive, give it a unique UUID/GUID (_which your editor of choice can generate_) and register it with `register_components`. And if you want it to show up in the editor, you can just mark it with `#[reflect(editor)]`.

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

`Blueprint` is Unreal's visual scripting system and it is heavily used in the engine. You can drive animations, materials, particles, sound, gameplay through it. If I wanted `unreal-rust` to become something real, I had to support blueprints somehow.

I added a new node called `GetComponent(Rust)`, which gives you access to all of your components in Rust.

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

We can now access the `MovementComponent` inside an animation blueprint to drive all of the animations. This is everything that is required to make the player character run, jump, and glide. The rest is handled inside the animation blueprint itself.

## Example
```rust
#[derive(Debug, Component)]
#[uuid = "b6addc7d-03b1-4b06-9328-f26c71997ee6"]
#[reflect(editor)]
pub struct PlaySoundOnImpactComponent {
    pub sound: USound,
}
```
```rust
fn register_hit_events(mut query: Query<(&mut ActorComponent,
                                         Added<PlaySoundOnImpactComponent>)>) {
    for (mut actor, added) in &mut query {
        if added {
            actor.register_on_hit();
        }
    }
}
```

```rust
fn play_sound_on_hit(
    api: Res<UnrealApi>,
    mut events: EventReader<ActorHitEvent>,
    query: Query<(&TransformComponent, &PlaySoundOnImpactComponent)>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if event.normal_impulse.length() <= PlaySoundOnImpactComponent::MINIMUM_FORCE {
            continue;
        }

        if let Some(&entity) = api.actor_to_entity.get(&event.self_actor) {
            if let Ok((trans, sound)) = query.get(entity) {
                play_sound_at_location(
                    sound.sound,
                    trans.position,
                    trans.rotation,
                    &SoundSettings::default(),
                )
            }
            commands.add(Despawn { entity });
        }
    }
}
```

```
Begin Frame -> Rust tick -> Unreal tick ->  End Frame
```


## Hot reloading

{{webmvideo(src="https://user-images.githubusercontent.com/1994306/187090400-19812611-f907-434d-88cd-363e4c3903e7.mp4")}}


# How does it work?

We need to set up a communication scheme between Rust and Unreal, and we are going to use c ffi.

We will compile our Rust code into a C compatible dll via `crate-type = ["cdylib"]`, and then use dlopen inside Unreal to load our Rust code.


Let's set up some data structures:

* `UnrealBindings`: This will contain function pointers to Unreal and we will store this on the Rust side. This allows us to call into Unreal from within Rust, like getting the position of an actor.
* `RustBindings`: This will contain function pointers to Rust and we will store this in Unreal. Sometimes Unreal has to call into Rust, eg call the `Tick` function of Rust, or getting reflection information out of Rust.

First we will look at `UnrealBindings`.

```rust
pub type GetSpatialDataFn = extern "C" fn(
    actor: *const AActorOpaque,
    position: &mut Vector3,
    rotation: &mut Quaternion,
    scale: &mut Vector3,
);

extern "C" {
    pub fn GetSpatialData(
        actor: *const AActorOpaque,
        position: &mut Vector3,
        rotation: &mut Quaternion,
        scale: &mut Vector3,
    );
}

#[repr(C)]
pub struct ActorFns {
    pub get_spatial_data: GetSpatialDataFn,
    ...
}

#[repr(C)]
pub struct UnrealBindings {
    pub actor_fns: ActorFns,
    ...
}
```

We define an extern function `GetSpatialData` and the respective `GetSpatialDataFn` function pointer. With the help of [cbindgen](https://github.com/eqrion/cbindgen) we can translate the code above into a c header `Bindings.h`.

Now we hop into Unreal and implement `GetSpatialData`

```cpp
void GetSpatialData(const AActorOpaque* actor,
                    Vector3* position,
                    Quaternion* rotation,
                    Vector3* scale)
{
    const auto Transform = ToAActor(actor)->GetTransform();
    *position = ToVector3(Transform.GetTranslation());
    *rotation = ToQuaternion(Transform.GetRotation());
    *scale = ToVector3(Transform.GetScale3D());
}
```

Then we simply fill out the `UnrealBindings` struct with the function pointer to `GetSpatialData`.

```cpp
ActorFns actor_fns = {};
actor_fns.get_spatial_data = &GetSpatialData;
...

UnrealBindings b = {};
b.actor_fns = actor_fns;
...

```

Now we have initialized `UnrealBindings` with some function pointers, but we need a way to get this struct into Rust. Going back to Rust we are going to define an entry point.


```rust
pub type EntryUnrealBindingsFn =
    unsafe extern "C" fn(bindings: UnrealBindings, rust_bindings: *mut RustBindings) -> u32;

#[no_mangle]
pub unsafe extern "C" fn register_unreal_bindings(
    bindings: UnrealBindings,
    rust_bindings: *mut RustBindings,
) -> u32 {
    ...
}

```

In Unreal we simply retrieve the `register_unreal_bindings` entry point

```Cpp
void* LocalBindings = FPlatformProcess::GetDllExport(LocalHandle, TEXT("register_unreal_bindings\0"));

```

And then we pass the `UnrealBindings` into `register_unreal_bindings`. The `RustBindings` will follow the exact same procedure.

```rust
pub type TickFn = unsafe extern "C" fn(dt: f32) -> ResultCode;

#[repr(C)]
pub struct RustBindings {
    pub tick: TickFn,
    ...
}

```

We create a `RustBindings` struct will all the function pointers we want, we implement the function in Rust and pass them into `RustBindings`. Now we simply give Unreal those bindings inside 
