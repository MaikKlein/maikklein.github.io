+++
date        = "2016-12-26"
title       = "Update report: Ash"
tags        = [ "Rust", "Vulkan", "Ash"]
+++

[Link to Ash](https://github.com/MaikKlein/ash)

## Optional loading

I have finally implemented optional loading for Vulkan extensions. For those who don't know in Vulkan you can load extensions by requesting them when you create an Instance or Device.

```Rust
fn extension_names() -> Vec<CString> {
    vec![CString::new("VK_KHR_surface").unwrap(),
         CString::new("VK_KHR_xlib_surface").unwrap(),
         CString::new("VK_EXT_debug_report").unwrap()]
}
```

Previously I was trying to load every possible function pointer. The problem with this approach is what if some extensions are not present you can not resolve the function pointer and this begs the question how you would handle that problem. Initially I just initialized function pointers that couldn't be resolved to `null`, this meant that I also added a sanity nullptr check whenever you would try to access that pointer.

There are also a few platform specific extensions, for example `VK_KHR_xlib_surface` or `VK_KHR_win32_surface`. I refactored all extension into a seperate structure.

```Rust
use ash::extensions::Swapchain;
let swapchain_loader = Swapchain::new(&instance, &device).expect("Unable to load swapchain");
let swapchain = swapchain_loader.create_swapchain_khr(&swapchain_create_info).unwrap();
```
The loader can fail if you forget to request the extension or if the extension is not available for your platform / device. The loader is allowed to go out of scope and can be loaded multiple times. Additionally the loader also implements `Clone`, `Send` and `Sync`. The only thing that you are not allowed to do is to delete the `Instance` or `Device` and continue to use the loader. This would result in a `use after free`.

## Safety
Most functions in `Ash` are now marked as `unsafe`. Specifically every function that is marked as `extern sync` in the Vulkan spec is considered unsafe.

## Example cleanup
[Examples](https://github.com/MaikKlein/ash/tree/master/examples) now have their own "framework" in order to avoid code duplication. Additionally the examples are now crossplatform. I switched from GLFW to winit which allowed me to add windows support. I haven't had the time to actually test it on a windows machine. If you are a windows user with a Vulkan ready system I would appreciate if you could try to run the examples.

## New entry point
Previously you could specify the path to the Vulkan library manually. I decided to remove this feature temporarily because of safety concerns. Currently you load Vulkan with `Entry` which allows you to create an `Instance` which allows you to create a `Device`. If `Entry` goes out of scope it will unload the shared library, which is bad because the `Instance` and the `Device` will still need it. Currently all shared library loaders that I have looked at behave that way.

To solve that, Vulkan is now loaded globally at startup. Usually I am not the biggest fan of globals but you would probably never try to load a different Vulkan library at runtime anyways.

Now the problem is that all shared library loaders have a Drop implementation which means I can not turn them into a global on stable Rust. I also can not put it behind an `AtomicPtr` because you can only initialize globals with a constant function. I had to use `lazy_static!`, but this also meant that I could not expose the search path for the Vulkan library to the user anymore.

## Custom allocator
If you look at some function in Vulkan, you will notice that most of them require a `VkAllocationCallbacks* pAllocator`.

```C
VkResult vkCreateInstance(
    const VkInstanceCreateInfo*                 pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkInstance*                                 pInstance);
```

>Vulkan provides applications the opportunity to perform host memory allocations on behalf of the Vulkan implementation. If this feature is not used, the implementation will perform its own memory allocations. Since most memory allocations are off the critical path, this is not meant as a performance feature. Rather, this can be useful for certain embedded systems, for debugging purposes (e.g. putting a guard page after all host allocations), or for memory allocation logging.

I have two options now. I could use an `Option` to represent a `pAllocator`
```Rust
pub fn create_instance(&self,
                       create_info: &vk::InstanceCreateInfo,
                       allocator: Option<vk::AllocatorCallbacks>)
                       -> Result<Instance, InstanceError>;
```

Or I could use default types

```Rust
pub fn create_instance<A: VkAllocator = DefaultAllocator>(&self,
                                                          create_info: &vk::InstanceCreateInfo)
                                                          -> Result<Instance, InstanceError>;
```

The former would be a breaking change, which is not a problem currently as the library is still unstable. The latter could be implemented post 1.0 as it would most likely not be a breaking change. Currently it is not possible to use default types for functions in stable Rust.

I still haven't made up my mind yet, I think using an Option would be more reminiscent of Vulkan. What do you think?

## What is still missing? When will Ash become stable?
I still need to expose around 20% of the Vulkan API and I still haven't exposed all extensions yet. If I decide to use default types for the allocators, the library would be very close to be stable. Otherwise I would have to change most function signatures to include the optional allocator.
