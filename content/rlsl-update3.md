+++
title = "The future of RLSL"
date  = 2020-10-22
+++

# History

3 years ago I started a project called "Rust Like Shading Language" to bring Rust to the GPU. I was very unhappy with the current state of affairs of glsl and hlsl, and I imagined what shader programming would be like if we had proper tooling like formatting, compiler errors, proper type system, package manager, auto completion etc. After Vulkan introduced SPIR-V, I was anticipating the appearance of many new shading languages, but nothing really did and I decided to take the matters in my own hand.

Knowing nothing about compilers, I wrote a simple parser and type checker for a very simple language, and I could generate a triangle ![triangle](https://camo.githubusercontent.com/fcf8368d94842046bcb1856ecd69386109ce672f/687474703a2f2f692e696d6775722e636f6d2f50515a634c36772e6a7067) But I realized that writing your own compiler is quite the undertaking and I was thinking about using rustc instead.

After a few days of head scratching I could render a triangle again going from `Rust -> HIR (High level IR) -> SPIR-V`. While translating HIR to SPIR-V was simple, it was also time consuming. I decided to try out MIR (Mid level IR). After a few weeks of head scratching I finally could render the triangle again, and a few days later I could also generate the default shader from shadertoy.

{{ iframe(src="UnrulyNeglectedBedbug") }}

I expected to run into major blocking issues. While there were certainly some challenges, I always found a way to hack around them. And I just kept on adding features constantly:

* Support for cargo and crates.io
* Simple math library
* Closures
* Complex branches (especially with `Try/?`)
* Simple loops
* Enums (Option<T> in shader)
* Computer shaders
* Full GPU/CPU testing
* Render fragement shaders on the CPU and GPU with the same code

{{twitter(id="1071852847669612545")}}


By that time I was working on slightly more complex shaders like raymarching, and I ran into a small slump. The ecosystem was quite immature at that time and debugging incorrect codegen was time consuming. I really needed a way to automate the debugging process.

I was also a University student, and I spent way too much time working on RLSL than I should have. I had a really hard time justifying my work on RLSL compared to focusing on my studies.

A few months later, I saw a post from [Embark Studios](https://www.embark-studios.com/) about Rust. Long story short, I applied, got the Job, moved from Germany to Sweden (Stockholm). Everyone at work was really interested in bringing Rust to the GPU as well. It just wasn't the right time to work on it professionally.

I also struggled to have a good work life balance while working on RLSL and [ash](https://github.com/MaikKlein/ash) in my spare time.

# RLSL is dead, long live [rust-gpu](https://github.com/EmbarkStudios/rust-gpu)

Fast forwarding 1.5 years, we now have [rust-gpu](https://github.com/EmbarkStudios/rust-gpu). See the [announcement](https://github.com/EmbarkStudios/rust-gpu/releases/tag/v0.1) for more details. `rust-gpu` will replace `RLSL`. `rust-gpu` is written by some extremely talented colleagues of mine, and I couldn't be happier with the work they have done. I am collaborating with the team although I am not actively committing code right now. I am extremely excited to be writing shaders in Rust soon and I hope you are as well.

I want to thank you all for the support you have given me over the years. I can't tell you how much all of the kind comments helped me going though some of my harder times. Thank you ❤️.
