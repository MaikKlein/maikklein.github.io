+++
date        = 2016-06-22
title       = "First impression of Rust after two years - Part 2"
tags        = [ "Rust"]
+++


# First impression of Rust after two years - Part 2

As promised this is part 2 of [part 1](https://maikklein.github.io/post/impression-rust/).

I created a small task based library in Rust based on naughty dogs gdc talk. The main purpose was to get a feeling of Rust and not the create a full blown library.

Here is how you would use it.

```Rust
fn main() {
    let res = TaskPool::submit(|| {
        println!("Before long running task");
        let r = TaskPool::submit(|| {
            std::thread::sleep(Duration::from_secs(10));
            return 42;
        });
        // Waits for the long running task to complete, does not block other tasks!
        println!("After long running task {}", r.await());
        42
    });
    for i in 0..20 {
        TaskPool::submit(move || println!("Another Task {}", i));
    }
    println!("{}", res.await());
}
```

The output can look like this

```
TASK 0
TASK 1
Before long task
TASK 2
TASK 3
TASK 4
TASK 9
TASK 5
TASK 10
TASK 19
TASK 6
TASK 11
TASK 17
TASK 18
TASK 7
TASK 8
TASK 12
TASK 13
TASK 14
TASK 15
TASK 16
After long task 42
42
```

The main feature is that tasks will never block other tasks. In a task system it would be very bad for a task to wait for other tasks to complete because you basically lose the benefit of a whole core/thread. Instead I am using a fiber / coroutine to yield the context.

I am currently using a fork of [coroutine-rs](https://github.com/rustcc/coroutine-rs). Also as you can see, the API is currently relying on a global but immutable TaskPool. The reason for this what that I had to explicitly wrap `TaskPool` inside an `Arc` and clone it every time I wanted to create a task in another task.

```Rust
fn main() {
    let taskpool = Arc::new(TaskPool::new(3));
    let taskpool1 = taskpool.clone();
    let res = taskpool.submit(|| {
        taskpool1.submit(|| println!("..."));
        42
    });
}
```

I am just not the biggest fan of global variables. Currently a TaskPool is immutable and I just randomly distribute tasks to `ThreadLocalQueue`. This is not really a good scheduler but it works. A better approach would probably to do all scheduling on the main thread.

Also if a tasks gets assigned to a `ThreadLocalQueue` it will stay in this queue until it is done. Once a task is assigned to a `ThreadLocalQueue` it will create a coroutine. The reason for this is performance because I don't want to worry about sharing fibers across threads and synchronizing them.

`Corountine-rs` currently does not support of getting the currently active coroutine. I have implemented it using a thread local variable.

```Rust
thread_local!(static FIBER: Cell<*mut coroutine::asymmetric::Handle> = Cell::new(std::ptr::null_mut()));
```

```Rust
for t in self.work.iter_mut() {
    FIBER.with(|f| {
        f.set(t);
    });
    t.next();
    FIBER.with(|f| {
        f.set(std::ptr::null_mut());
    });
}
```
The gist of it is that before I execute a coroutine, I put a pointer to it on the `tls`, execute it and null the ptr again. I have no idea if doing that is even legal.

I am doing it this way because I then always know if I am currently inside a task or not, without needing to explicitly pass the coroutine.

```Rust
struct Future<T> {
    receiver: Receiver<T>,
}
impl<T> Future<T> {
    fn new(receiver: Receiver<T>) -> Self {
        Future { receiver: receiver }
    }

    fn await(&self) -> T {
        let mut fiber = FIBER.with(|f| {
            return f.get();
        });
        let is_fiber = fiber != std::ptr::null_mut();
        if is_fiber {
            loop {
                let r = self.receiver.try_recv();
                if r.is_ok() {
                    return r.unwrap();
                }
                unsafe {
                    (*fiber).resume(0);
                }
            }
        } else {
            return self.receiver.recv().unwrap();
        }
    }
}
```
Obviously the whole library is implemented in a super hacky way but this allows me to block on the main thread and reschedule inside a task. Rescheduling works by testing if `try_recv` actually returns something, if it doesn't, I yield the context.

Now I want to talk about the standard library. I am coming from D where I had to recreate almost anything myself that Rust ships by default because I wanted to avoid the GC. That meant recreating Box, Rc, Arc, Vec, Optional, Result etc and basically every container, because none of the container in the phobos (std) is move aware.

My main goal is to create a game engine from scratch, while I enjoy working really low level, recreating a big part of the standard library was not as much fun as I thought.

Coming from D it felt really good to have a standard library that works without a GC. The biggest problem of creating a lot of stuff from scratch is that I am the only user. If I encounter some problem, I can't really post my code anywhere if it contains my custom smart-ptr or Optional/Result. Also using other libraries is probably a no go because they will most likely use the standard library which doesn't work with my move aware types.

So while it is completely possible to implement all those constructs in D, it felt a bit awkward to create everything manually. This makes me appreciate Rust's standard library much more.

Implementing the task library was relatively painless, the only thing that was missing were coroutines / fibers in the std, which were available as a 3rd party library.

## Conclusions:

*Note: I base my conclusions on Rust 1.9 stable and I do not include features that may or may not come in the future. I know this may offend some people but it is too hard to judge Rust from the perspective of an outsider. I just don't have enough information when which features might become available in stable Rust as there doesn't seem to be publicly available roadmap.*

#### What I like:

* Traits with dynamic / static dispatch
* Useful and well documented standard library
* Good standard documentation engine
* A sort of standardized style guide. I really like that all libraries have an almost identical syntactical style.
* Move semantics are really well done
* Default immutability and the concept of interior mutability
* No constructors
* Memory safety and no race conditions in safe Rust
* One standard open source compiler based on LLVM
* Useful error messages most of the time
* Solid base for a good ecosystem with cargo
* Implementation lives outside a type
* Good type inference
* Explicit error handling like in Haskell
* Pattern matching, deconstructing patterns like `let (a, b) = ...;`

#### What I dislike:

* No variadics
* No type level integers
* Metaprogramming in general is lacking, also unsure where Rust is heading in regards to metaprogramming.
* Compiler sometimes can not reason about lifetimes / mutability, which requires workarounds.
* Compile times seem relatively long compared to D
* No custom allocators
* No constexpr or compile time evaluation
* No public roadmap
* Immature tools like `racer` (I don't mean to offend anyone here, I appreciate the effort that is put into those tools)

My overall impression is that I think Rust has a very good core language but it still misses some advanced features.
