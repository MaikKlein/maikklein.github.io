+++
date        = 2016-12-29
title       = "Final progress report for Ash"
tags        = [ "Rust", "Vulkan", "Ash"]
+++

[Link to Ash](https://github.com/MaikKlein/ash)

# Custom allocators

I exposed custom allocators to every function that requires it. I decided to go with Optional<&vk::allocationCallbacks>

```Rust
unsafe fn create_instance(&self,
                          create_info: &vk::InstanceCreateInfo,
                          allocation_callbacks: Option<&vk::AllocationCallbacks>)
                          -> Result<Instance, InstanceError>;
```
Because allocators are unsafe, I also had to mark every function that can take an allocator as unsafe.

# Extension names
Every extension now exposes the Vulkan name as a `CStr`.

```Rust
use ash::extensions::{Swapchain, XlibSurface, Surface, DebugReport};
#[cfg(all(unix, not(target_os = "android")))]
fn extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr()
    ]
}
```

# Crossplatform examples

The examples should now work on Linux and Windows. Today I have finally installed Rust and the Vulkan SDK on my windows machine to test if the examples would run.

# Extension loading
I exposed all [extensions](https://github.com/MaikKlein/ash/tree/master/src/extensions) which meant that I could finally rewrite the loader. Previously I loaded every possible function that I exposed. As I explained in my previous post, this meant that some function pointers had to remain uninitialized because some extensions could not be loaded.

This meant that if you would try to access a function pointer that failed to load you would crash. I have spent time to read through several OpenGL / Vulkan codebases and this is how everyone seems to do it, they just let it crash. 

In `Ash` instead of letting it crash, it will just fail at loading time. You will get an `Result` that contains all function pointers that failed to load. Doing it that way seems much cleaner but it comes with another problem and that is extensibility.

Currently Vulkan is at version `1.0.38`, and Vulkan essentially uses [Semver](http://semver.org/). This means that Vulkan will most likely introduce new functions in a minor version update. `Ash` uses a load everything or fail approach and this means that I have to solve that particular problem.

I have a few options, I could implement `Ash` as a generator, that would output a Vulkan wrapper for a specific version.
```
./generateAsh 1.7
```
A few OpenGL loader do this, but the problem is how do you handle older versions? You could generate a few versions like OpenGL3, OpenGL4 and use them to implement your renderer, or you could just use the highest version and if you need to go lower only initialize function pointers for the lower version.

I decided to try a trait based approach.
```Rust
pub trait InstanceMajor1Minor0 {
    fn handle(&self) -> vk::Instance;
    fn fp_major1_minor0(&self) -> &vk::InstanceFn;
    unsafe fn create_device(&self,
                            physical_device: vk::PhysicalDevice,
                            create_info: &vk::DeviceCreateInfo,
                            allocation_callbacks: Option<&vk::AllocationCallbacks>)
                            -> Result<Device, DeviceError> {
        let mut device: vk::Device = mem::uninitialized();
        let err_code = self.fp_major1_minor0()
            .create_device(physical_device,
                           create_info,
                           allocation_callbacks.as_raw_ptr(),
                           &mut device);
        if err_code != vk::Result::Success {
            return Err(DeviceError::VkError(err_code));
        }
        let device_fn = vk::DeviceFn::load(|name| {
                mem::transmute(self.fp_major1_minor0().get_device_proc_addr(device, name.as_ptr()))
            }).map_err(|err| DeviceError::LoadError(err))?;
        Ok(Device::from_raw(device, device_fn))
    }
    // rest
pub trait InstanceMajor1Minor1: InstanceMajor1Minor0 {}
pub trait InstanceMajor1Minor2: InstanceMajor1Minor1 {}
//...
}
```
Instead of implementing all the functions directly on `Instance`, I would implement them directly in the trait. Every function now lives in the correct version and it is trivial to add functions for a newer spec. You can not accidentally call a function that hasn't been loaded as everything is checked at compile time.

In your renderer you could now easily create a fallback to a different version.

```Rust
// pseudo code
fn fall_back_create_instance() -> Result<impl InstanceMajor1Minor7, impl InstanceMajor1Minor0>;
```

You can still pass newer API's to an older one, for example in `struct Renderer<Instance: InstanceMajor1Minor7>{...}` you can still pass in an instance of version 1.8. This is because the Vulkan spec doesn't remove older functions. For example 1.3 contains 1.2 contains 1.1 contains 1.0 which maps nicely to trait inheritance.
This seems like it would be the best approach, but I still don't like the names of the traits.

# When will it be stable?

I don't expect many big changes anymore that would effect stability. The last big change will be to make `Ash` extensible, after that `Ash` will still remain <1.0 as I still want to gather more usage experience. The breakage will most likely be minimal.
