+++
date        = "2016-12-30"
title       = "Constrain API versions statically with traits"
tags        = [ "Rust", "Vulkan", "Ash"]
+++

As some of you know I am currently writing a Vulkan wrapper called [Ash](https://github.com/MaikKlein/ash). Ash dynamically links Vulkan, which means that it has to load the functions pointers on its own, and it currently works for Vulkan 1.0.x. Vulkan also follows [semver](http://semver.org/) which means that in the future new functionality will be added. The easiest way to implement extensibility is to load everything and don't resolve function pointers that failed to load (runtime error). In Ash I resolved
this problem statically with traits.


The idea is pretty simple but I haven't seen something similar before and I thought it could be worth sharing.

```Rust
pub trait VkVersion {
    type InstanceFp;
    type DeviceFp;
}

#[warn(non_camel_case_types)]
pub struct InstanceFpV1_0 {
    pub instance_fn: vk::InstanceFnV1_0,
}

#[derive(Clone)]
pub struct Instance<V: VkVersion> {
    handle: vk::Instance,
    instance_fp: V::InstanceFp,
}
```

We have a VkVersion that just exposes associated types which are the functions pointers. In `vk::InstanceFnV1_0` live all the raw Vulkan function pointer for the version v1.0.x.

To add a new spec with the new function pointer, we could simply create a new struct that contains all the function pointer that we need.

```Rust
#[warn(non_camel_case_types)]
pub struct InstanceFpV1_1 {
    pub instance_fn_v1_0: vk::InstanceFnV1_0,
    pub instance_fn_v1_1: vk::InstanceFnV1_1,
}
```

Now we simply just need to implement the trait `VkVersion`
```Rust
#[warn(non_camel_case_types)]
pub struct V1_0;
impl VkVersion for V1_0 {
    type InstanceFp = InstanceFpV1_0;
    type DeviceFp = DeviceFpV1_0;
}
```
You could now use it like this
```Rust
struct Context{
    instance: vk::Instance<V1_0>,
    device: vk::Device<V1_0>,
}
// or
struct Context<V: VkVersion>{
    instance: vk::Instance<V>,
    device: vk::Device<V>,
}
let context: Context<V1_0> = ...;
```
One thing is still missing, we need to implement the correct functions for the correct version. This maps perfectly to a trait.
```Rust
#[warn(non_camel_case_types)]
pub trait InstanceV1_0 {
    fn handle(&self) -> vk::Instance;
    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0;
    fn get_device_proc_addr(&self,
                            device: vk::Device,
                            p_name: *const vk::c_char)
                            -> vk::PFN_vkVoidFunction {
        unsafe { self.fp_v1_0().get_device_proc_addr(device, p_name) }
    }

    unsafe fn destroy_instance(&self, allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0().destroy_instance(self.handle(), allocation_callbacks.as_raw_ptr());
    }
    // ... Rest
}
```
Instead of implementing the functions directly on `Instance<V>` or `Device<V>`, we implement them in the trait.
```Rust
impl InstanceV1_0 for Instance<V1_0> {
    fn handle(&self) -> vk::Instance {
        self.handle
    }

    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0 {
        &self.instance_fp.instance_fn
    }
}
```
This exposes all the functions of `InstanceV1_0` to `Instance<V1_0>`, which is just what we wanted. Future functions can be added like this:
```Rust
pub trait InstanceV1_1: InstanceV1_0 {
    fn fp_v1_1(&self) -> &vk::InstanceFnV1_1;
    // new functions
}
pub trait InstanceV1_2: InstanceV1_1 {
    fn fp_v1_2(&self) -> &vk::InstanceFnV1_2;
    // new functions
}
```

This works because Vulkan 1.2 contains all the functions of 1.1 which contains all the functions of 1.0 and so on. The only thing that can not be in the trait are the constructors. In Ash, Entry creates and Instance, Instance creates a Device. This means for newer versions we have to change the create functions to also load the new functions.

Luckily this also maps very nicely to Rust. We simply implement the `create_xxx` functions for a specific version.
```Rust
impl Instance<V1_0> {
    pub unsafe fn create_device(&self,
                                physical_device: vk::PhysicalDevice,
                                create_info: &vk::DeviceCreateInfo,
                                allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                -> Result<Device<V1_0>, DeviceError> {
        // stuff
    }
}
```

Loading everything is as simple as.
```Rust
let entry = Entry::<V1_0>::load_vulkan().unwrap();
let instance = entry.create_instance(...).unwrap();
let device = entry.create_device(...).unwrap();
```

What are the implications?
If you decide to hard code the versions in your library like this
```Rust
struct Context{
    instance: vk::Instance<V1_0>,
    device: vk::Device<V1_0>,
}
```
you can still upgrade painlessly (with out any breakage) to a newer version.

```Rust
struct Context{
    instance: vk::Instance<V1_1>,
    device: vk::Device<V1_1>,
}
```

I will also implement `From` so that you can still use newer versions
```Rust
struct Context{
    instance: vk::Instance<V1_1>,
}
Context{
    instance: newer_version_1_5.clone().into()
}
```
This will only work for higher versions to lower versions.

You could also use the traits directly.

```Rust
struct Context<I: InstanceV1_4>{
    instance: I
}
```

The only thing that I still don't fully like is that I had to break the naming scheme. Types now include snake_case and CamelCase `InstanceFpV1_1`. The alternative could be `InstanceFpMajor1Minor1` which is pretty long.

To recap:

- Constrains API versions at compile time
- Turns runtime errors into compile time errors
- Easy to extend functionality without breakage
- Load only what you need

Interestingly this also reduced the build time of `Ash` from 14 seconds to 5 seconds, and I am not sure why.
