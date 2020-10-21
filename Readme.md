## Dash

Welcome to Dash! Dash is a high-level imperative systems programming language inspired in design and purpose by C and syntactically by BASIC (and a few other languages). This repository contains the compiler for Dash, as well as the examples and test system.

### Why Dash?

I started Dash as a descendant to my Quik compilers. The original Quik was when I was learning compiler development, so they weren't designed with any big purpose. I started Dash because I liked the Quik language I came up with, and because I needed a simple compiler so I could experiment with different architectures, learn a little more about compiler transformations, and so forth. I also started the project in hopes I can use this for some benchmarking work I do in one of my jobs.

However, Dash is not solely a personal thing. The first goal with Dash is simplicity- in the language itself and in the implementation. Dash is meant to be like C in that its basically portable assembly. The language should be super easy to port to any platform, whether a CPU or VM architecture (source interpretation should be easy as well). However, I do not wish to re-invent the wheel, so part of Dash is full interpolation with C and the system libraries.

They key difference of Dash from C is the language design and the abstracting of certain concepts. For example, safety is a big goal. There are no pointers and references in C. Any programming construct requiring pointers and references is abstracted away. Dash also aims to provide better support for things like strings and data structures, two things which I think are very lacking in C. On the hardware level, I also have a goal of providing constructs to make newer hardware features easier to use, including threading and SIMD. And finally, since I like living close the hardware and the operating system, I have constructs to make it easy to interface with the underlying OS.

### Features

All the stuff here is either completely implemented or in-process.

* Complete compatibility with C objects and libraries
* Super simple to parse and understand
* No ambiguous grammar
* Easy to translate to underlying architectures
* Default x86-64 and AArch64 support
* Support for all numerical types
* Automated test system
* Syntax checking and reporting
* Experimental SIMD support- no intrinsics!

### Why Rust for the compiler?

When I started learning about compilers, I used C++. And that was natural choice; C++ lends itself well to compiler development (think LLVM...), and for me personally, I had been using C++ almost daily for six years, so I wanted to focus on the concept and not worry about learning the language.

However, I feel that C++ presents issues. It has pointers and references, which is a useful feature in the right situations. And I do not think compiler development is always the right situation. I was getting tired of the segmentation faults and random crashes which can be really hard to debug. And while C++'s OOP is really good for the trees needed in compiler development, the language lacks (in my opinion) features that are good for compiler development, pattern matching being one. 

I experimented with a few other languages, and eventually settled on Rust. I think Rust is extremely well suited for compiler development. Its memory model is extremely safe- I've never had a crash. Its enforced error handling is amazing. In fact, Rust is so safe I don't think I've ever had a significant crash with it- I don't think I've really ever had one at all. Usually if there's a compiler issue, it shows up in the assembly stage. Rust has fabulous pattern matching, I love how you can attach values to enums, and even though it doesn't have OOP in the sense of C++/Java/etc, I actually like the structure I came up with better. Finally, Rust runs on pretty much any platform and architecture.


