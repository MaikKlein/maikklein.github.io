+++
date        = "2017-11-23"
title       = "Enum"
tags        = [ "Rust"]
+++

# Common enum data

```Rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InstanceDef<'tcx> {
    Item(DefId),
    Intrinsic(DefId),

    /// <fn() as FnTrait>::call_*
    /// def-id is FnTrait::call_*
    FnPtrShim(DefId, Ty<'tcx>),

    /// <Trait as Trait>::fn
    Virtual(DefId, usize),

    /// <[mut closure] as FnOnce>::call_once
    ClosureOnceShim { call_once: DefId },

    /// drop_in_place::<T>; None for empty drop glue.
    DropGlue(DefId, Option<Ty<'tcx>>),

    ///`<T as Clone>::clone` shim.
    CloneShim(DefId, Ty<'tcx>),
}
```

This code is from the rust compiler but I have seen this pattern multiple times. As you can see every variant contains a `DefId`. If you need to get access the `DefId` you have to match on every variant.

```Rust
impl<'tcx> InstanceDef<'tcx> {
    #[inline]
    pub fn def_id(&self) -> DefId {
        match *self {
            InstanceDef::Item(def_id) |
            InstanceDef::FnPtrShim(def_id, _) |
            InstanceDef::Virtual(def_id, _) |
            InstanceDef::Intrinsic(def_id, ) |
            InstanceDef::ClosureOnceShim { call_once: def_id } |
            InstanceDef::DropGlue(def_id, _) |
            InstanceDef::CloneShim(def_id, _) => def_id
        }
    }
```

It would be easier to maintain if the enum is wrapped in a struct.

```Rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InstanceDefKind<'tcx> {
    Item,
    Intrinsic,

    /// <fn() as FnTrait>::call_*
    /// def-id is FnTrait::call_*
    FnPtrShim(Ty<'tcx>),

    /// <Trait as Trait>::fn
    Virtual(usize),

    /// <[mut closure] as FnOnce>::call_once
    ClosureOnceShim,

    /// drop_in_place::<T>; None for empty drop glue.
    DropGlue(Option<Ty<'tcx>>),

    ///`<T as Clone>::clone` shim.
    CloneShim(Ty<'tcx>),
}

pub struct InstanceDef{
    def_id: DefId,
    kind: InstanceDefIdKind
}
```

I often wrap my enums in a struct if I am not sure if I need to add some data to every single member, because if I do it gernally is trivial to add.

# Struct variants

```Rust
#[derive(PartialEq, Clone, Debug)]
pub enum Message {
    MetaEvent {
        delta_time: u32,
        event: MetaEvent,
        data: Vec<u8>,
    },
    MidiEvent { delta_time: u32, event: MidiEvent },
    SysExEvent {
        delta_time: u32,
        event: SysExEvent,
        data: Vec<u8>,
    },
    TrackChange,
}
```

There are a few problems with struct variants. For example if I want to generate multiple `MidiEvent`, I would have to create a `Message`. If I then want to mutate those events I would need to match of a `Message::MidiEvent`.

Generially I prefer to create real structs.

```Rust
#[derive(PartialEq, Clone, Debug)]
pub struct MidiEvent { delta_time: u32, event: MidiEvent }
#[derive(PartialEq, Clone, Debug)]
pub enum Message {
...
    MidiEvent(MidiEvent)
...
}
```


Subjectively this also makes it easier to match on those variants.

```Rust
struct Test {
    i: u32,
}

enum Bar {
    Test(Test),
    Other,
}

enum Baz {
    Test { i: u32 },
    Other,
}

fn main() {
    let mut t = Bar::Test(Test { i: 42 });
    if let Bar::Test(ref mut test) = t {
        test.i = 24;
    }
    // vs
    let mut t = Baz::Test { i: 42 };
    if let Baz::Test { ref mut i } = t {
        *i = 24;
    }
}
```

But if you add a field to Test

```Rust
struct Test {
    i: u32,
    i2: u32,
}

enum Bar {
    Test(Test),
    Other,
}

enum Baz {
    Test { i: u32, i2: u32 },
    Other,
}

fn main() {
    let mut t = Bar::Test(Test { i: 42, i2: 100 });
    if let Bar::Test(ref mut test) = t {
        test.i = 24;
    }
    // vs
    let mut t = Baz::Test { i: 42, i2: 100 };
    if let Baz::Test { ref mut i } = t {
        *i = 24;
    }
}
```
[playground](https://play.rust-lang.org/?gist=0e1814acd8d8776ed66cb981caaead9a&version=stable)

```Rust
error[E0027]: pattern does not mention field `i2`
  --> src/main.rs:23:12
   |
23 |     if let Baz::Test { ref mut i } = t {
   |            ^^^^^^^^^^^^^^^^^^^^^^^ missing field `i2`
```

So when you match on an struct variants it is probably a good idea, to add `, ..`.

```Rust
    if let Baz::Test { ref mut i, .. } = t {
        *i = 24;
    }
```


```Rust
struct Test {
    i: u32,
    i2: u32,
}

enum Bar {
    Test(Test),
    Other,
}

enum Baz {
    Test { i: u32, i2: u32 },
    Other,
}

fn main() {
    let mut t = Bar::Test(Test { i: 42, i2: 100 });
    if let Bar::Test(ref mut test) = t {
        test.i = 24;
        test.i2 = 50;
    }
    // vs
    let mut t = Baz::Test { i: 42, i2: 100 };
    if let Baz::Test {
        ref mut i,
        ref mut i2,
        ..
    } = t
    {
        *i = 24;
        *i2 = 50;
    }
}
```
In the first case we only had to change one line, `test.i2 = 50;`. But in the struct variant case, we had to also name the variable.

```Rust
    if let Baz::Test {
        ref mut i,
        ref mut i2,
        ..
    } = t
    {
        *i = 24;
        *i2 = 50;
    }
```

Struct variants also feel inconsistent when they are mixed with tuple variants. From the [RFC](https://github.com/nox/rust-rfcs/blob/master/text/0418-struct-variants.md)

```Rust
enum Foo {
    Foo,
    Bar(int, String),
    Baz { a: int, b: String }
}
```

Or a more real life example

```Rust
pub enum WindowEvent {
    Resized(u32, u32),
    Moved(i32, i32),
    Closed,
    DroppedFile(PathBuf),
    HoveredFile(PathBuf),
    HoveredFileCancelled,
    ReceivedCharacter(char),
    Focused(bool),
    KeyboardInput {
        device_id: DeviceId,
        input: KeyboardInput,
    },
    MouseMoved {
        device_id: DeviceId,
        position: (f64, f64),
    },
    MouseEntered {
        device_id: DeviceId,
    },
    MouseLeft {
        device_id: DeviceId,
    },
    MouseWheel {
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },
    MouseInput {
        device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    },
    TouchpadPressure {
        device_id: DeviceId,
        pressure: f32,
        stage: i64,
    },
    AxisMotion {
        device_id: DeviceId,
        axis: AxisId,
        value: f64,
    },
    Refresh,
    Touch(Touch),
}
```

I never really know if I have to match with `Foo::Bar(bar)` or `Foo::Bar{ some_name, .. }`. It often feels very inconsistent. I can see the benefit of having struct variants, because you can define them inside the enum and they all will derive the same traits, but I still generally avoid them.
