+++
date        = 2016-11-05
title       = "My first steps with Future-rs"
tags        = [ "Rust"]
+++

A few months ago I have written a small task system, it looks like this.

```Rust
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
println!("{}", res.await());
```
* It is possible to spawn tasks inside tasks
* Calling `.await()` on the main thread will block
* Calling `.await()` inside the task pool will reschedule the task

The task system was inspired by [Naughty Dog's Task system](http://www.gdcvault.com/play/1022186/Parallelizing-the-Naughty-Dog-Engine) but since then `future-rs` was released. Today I finally had time to test it out.

Instead of spawning tasks inside tasks you create `futures`.
```Rust
let some_future = futures::finished::<i32, ()>(42).map(|i| i + 42);
```
You can think of them as a finite state machine.
```Rust
let pool = CpuPool::new(3);
let some_future = futures::finished::<i32, ()>(42).map(|i| i + 42);
let cpu_future = pool.spawn(some_future);
println!("{}", cpu_future.wait().unwrap());
```
The biggest difference here is that submitting and creating work is completely separated.

`Futures` can be composed together
```Rust
fn future_test(id: i32) -> impl futures::Future<Item=i32, Error=()> {
    futures::finished::<i32, ()>(42)
        .map(move |i| {
            println!("1st map id {}  {:?}", id, thread_id::get());
            i + 1
        })
        .map(move |i| {
            println!("2nd map id {}  {:?}", id, thread_id::get());
            i + 2
        })
        .map(move |i| {
            println!("3rd map id {}  {:?}", id, thread_id::get());
            i + 3
        })
}
```
Though you probably want to make use of `impl trait` if you compose multiple futures together. Above I used the `thread-id` crate to see on which thread the future will execute.

I am currently writing a rendering engine in Vulkan and I need to record `CommandBuffers` on different threads. This means I have to figure out how I actually submit futures onto different threads.
```Rust
let c: Vec<_> = (0 .. 10).map(|i| future_test(i)).collect();
let r = pool.spawn(futures::collect(c));
println!("{:?}", r.wait());
```
This will print:
```Rust
1st map id 0  140444805625600
2nd map id 0  140444805625600
3rd map id 0  140444805625600
1st map id 1  140444805625600
2nd map id 1  140444805625600
3rd map id 1  140444805625600
1st map id 2  140444805625600
2nd map id 2  140444805625600
3rd map id 2  140444805625600
1st map id 3  140444805625600
2nd map id 3  140444805625600
3rd map id 3  140444805625600
1st map id 4  140444805625600
2nd map id 4  140444805625600
3rd map id 4  140444805625600
1st map id 5  140444805625600
2nd map id 5  140444805625600
3rd map id 5  140444805625600
1st map id 6  140444805625600
2nd map id 6  140444805625600
3rd map id 6  140444805625600
1st map id 7  140444805625600
2nd map id 7  140444805625600
3rd map id 7  140444805625600
1st map id 8  140444805625600
2nd map id 8  140444805625600
3rd map id 8  140444805625600
1st map id 9  140444805625600
2nd map id 9  140444805625600
3rd map id 9  140444805625600
Ok([48, 48, 48, 48, 48, 48, 48, 48, 48, 48])
```
You may notice that the thread id is always the same. This is because a `future` will currently execute only on 1 thread. This is not what I wanted to achieve.

If you want parallelism you should probably not submit one giant future but more smaller ones.
```Rust
let c: Vec<i32> =
    (0..10).map(|i| pool.spawn(future_test(i))).collect::<Vec<_>>().into_iter()
           .map(|f| f.wait().unwrap()).collect();
```
This code will print:
```Rust
1st map id 0  139959187011328
2nd map id 0  139959187011328
3rd map id 0  139959187011328
1st map id 1  139959184910080
2nd map id 1  139959184910080
3rd map id 1  139959184910080
1st map id 2  139959182808832
2nd map id 2  139959182808832
3rd map id 2  139959182808832
1st map id 3  139959187011328
2nd map id 3  139959187011328
3rd map id 3  139959187011328
1st map id 4  139959184910080
2nd map id 4  139959184910080
3rd map id 4  139959184910080
1st map id 5  139959182808832
2nd map id 5  139959182808832
3rd map id 5  139959182808832
1st map id 6  139959187011328
2nd map id 6  139959187011328
3rd map id 6  139959187011328
1st map id 7  139959184910080
2nd map id 7  139959184910080
3rd map id 7  139959184910080
1st map id 8  139959182808832
2nd map id 8  139959182808832
3rd map id 8  139959182808832
1st map id 9  139959187011328
2nd map id 9  139959187011328
3rd map id 9  139959187011328
```
The code creates 10 `Futures` from `future_test` and immediately spawns them with `pool.spawn(future_test(i))` which returns a `CpuFuture`. It then waits sequentially on the result and writes its result into a vector.

You might also notice that every `Future` from `future_test` will execute on the same thread.

I haven't spent too much time with `Future-rs` but it looks very promising. The next thing I will look into is how I can safely share stack references inside `Futures`. This was one part where I struggled with my `TaskPool` implementation and I will probably run into the same issues with `Future-rs` and `TaskPool` because `spawn` has `'static` lifetime requirements.

```Rust
fn spawn<F>(&self, f: F) -> CpuFuture<F::Item, F::Error>
where F: Future + Send + 'static, F::Item: Send + 'static, F::Error: Send + 'static
```

[Rayon](https://github.com/nikomatsakis/rayon) does seem to offer this
```Rust
/// Increment all values in slice.
fn increment_all(slice: &mut [i32]) {
    if slice.len() < 1000 {
        for p in slice { *p += 1; }
    } else {
        let mid_point = slice.len() / 2;
        let (left, right) = slice.split_at_mut(mid_point);
        rayon::join(|| increment_all(left), || increment_all(right));
    }
}
```

Luckily `futures-cpupool` is only a few 100 lines and therefore easy to make changes. I might have to remove the `Send` requirement from `.spawn` in order to have stack borrows. Mostly because it would be very bad if you create a future that has a borrow on the stack and you send it to a different thread. But it should be safe if it only gets send to the taskpool because you will get back another future which also doesn't implement `Send`. Then you could call `.wait()` in the destructor but I see I
am already getting ahead of myself.
