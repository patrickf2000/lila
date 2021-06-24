## Ida

Welcome to Ida! Ida is my high-level imperative systems programming language. Although its meant to be suitable for low-level work, its primarily meant for userspace applications. Ida is inspired by the Wirth-family of programming languages, specifically Ada and Pascal.

This contains the parser, compiler, libraries, and tests for the language.

Here's a small example:

```
use std.text_io;

func main(args:str[]) -> int
    length : int = sizeof(args);
begin
    println("Hello world!");
    
    printf("Number of args: %d\n", length);
    
    for arg in args
        println(arg);
    end

    return 0;
end
```

Compile with `idac first.ida -o first`

### Status

This has been my hobby/learning project of the past three months now. I don't think the syntax will change much anymore, but its still under a lot of development. Its to the point where it can do some non-trivial tasks, but I still have a ways to go.

### Features

All the stuff here is implemented in some way.

* Entire compiler is written in Rust
* Easy to parse and understand
* Support for all numerical types (signed and unsigned are separate types)
* Built-in string type and operations
* Unique approach to arrays
* Automatic memory allocation and deallocation
* Module system instead of headers
* Standard and core libraries written in Ida
* Experimental SIMD support- no intrinsics! (not fully implemented however...)
* Optimization layer for RISC architectures
* Automated test system
* Compatible with C objects and libraries

### Architecture Support

Currently, only x86-64 is fully supported; other architectures are in progress. The LTAC IR (which translates to the final assembly) resembles CISC assembly, which makes it super easy to generate code for x86 (and besides, my computers all run it...). There is an optimization layer that converts the IR to RISC-like assembly which makes it much easier to port to other architectures. See the "docs" folder for creating a new backend (Note: I will probably be reworking the main LTAC generation layer to eliminate the need for this).

Current state:   

* x86-64: Fully supported, all tests pass *
* RISC-V (64-bit): About 50-60% supported; all integer, byte, short, loop, and several other tests pass
* Arm64: Hello world and a few basic integer tests pass

Note for x86-64: I recently rewrote the entire x86-64 code generator to cleanup and hopefully make it a little easier to expand and optimize later on. I'm also working on phasing out the C library. As a result, there is very little code generation for floating point and none for vector instructions. The standard library will still build and all the example programs will pass. I do plan on getting this re-implemented at a future point.

### System Requirements

Currently, Ida only compiles programs for Linux. I started development on Linux Mint, but I have since been using either Fedora or Manjaro depending on the day, so you can safely assume it works on any version of Linux. However, it only works on Linux at the moment because the core and standard libraries use Linux system calls.

I am very seriously thinking about doing a Windows port, but I have to think about how to do the libraries. This will probably still be a ways off, but it is in the pipeline.

### The Standard Library

Ida has two libraries: the core library, and the standard library.

The core library is statically linked to whatever your programs and libraries. This contains a few core functions such as malloc, free, println, strlen, and others. These are small, self-contained functions that have no dependencies; they make the appropriate system calls. There is an undocumented compiler flag to disable this, but certain language features will fail if you do so.

The standard library has all the nice functions, such as printf, file reading, strings, and so forth. This is dynamically linked.

### Licensing

Ida is licensed under the BSD-3 license. See the copying file for more information.

Depending on how long you've followed the project, you may noticed this is the 3rd rename of this codebase. The rename is partially due to personal reasons, but I also decided I would prefer to have this under the BSD-3 license. Going forth, all development to this project will be under that license. If you want the older, GPL-2 licensed version (which was named "Lila"), you may email me and ask. However, any subsequent changes to that codebase ARE NOT GPL. They are BSD. So if you create a fork and release it under the GPL license, you can use my BSD modifications, but they will be BSD-licensed. If the file is any different from the GPL, it is now BSD.
