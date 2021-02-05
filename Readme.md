## Lila

Welcome to Lila! Lila is my high-level imperative systems programming language. Although its meant to be suitable for low-level work, its primarily meant for userspace applications. Lila is inspired by the Wirth-family of programming languages, specifically Ada and Pascal.

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

Compile with `lilac first.ls -o first`

### Status

This has been my hobby/learning project of the past three months now. I don't think the syntax will change much anymore, but its still under a lot of development. Its to the point where it can do some non-trivial tasks, but I still have a ways to go.

Also, note that even though Github says 20% of the code is Livescript, there's not a single line of it in here. I didn't realize that Livescript used the ".ls" extension, so that 20% is actually the Lila tests and examples. I may change the file extension later on.

Please see [the log](https://patrickflynn.co/pages/lila-log.html) for updates.

#### Update (February 2021)

I'm currently working on a new middle-end IR. The main purpose at the moment is to provide an easier transition to LLVM, but I hope to eventually replace the LTAC IR with it. As a result of this, there may be some duplicate code, and the parser will be a little messy probably for the next few months.

Please see the stable branch for the pre-LLIR build.

### Features

All the stuff here is implemented in some way.

* Entire compiler is written in Rust
* Easy to parse and understand
* Support for all numerical types (signed and unsigned are separate types)
* Built-in string type and operations
* Unique approach to arrays
* Automatic memory allocation and deallocation
* Module system instead of headers
* Standard and core libraries written in Lila
* Experimental SIMD support- no intrinsics! (not fully implemented however...)
* Optimization layer for RISC architectures
* Automated test system
* Compatible with C objects and libraries

### Architecture Support

Currently, only x86-64 is fully supported; other architectures are in progress. The LTAC IR (which translates to the final assembly) resembles CISC assembly, which makes it super easy to generate code for x86 (and besides, my computers all run it...). There is an optimization layer that converts the IR to RISC-like assembly which makes it much easier to port to other architectures. See the "docs" folder for creating a new backend.

Current state:   

* x86-64: Fully supported, all tests pass *
* RISC-V (64-bit): About 50-60% supported; all integer, byte, short, loop, and several other tests pass
* Arm64: Hello world and a few basic integer tests pass

Note for x86-64: I recently rewrote the entire x86-64 code generator to cleanup and hopefully make it a little easier to expand and optimize later on. I'm also working on phasing out the C library. As a result, there is very little code generation for floating point and none for vector instructions. The standard library will still build and all the example programs will pass. I do plan on getting this re-implemented at a future point. The original code generator is in the "codegen1" branch.

### System Requirements

All development is currently done on Linux Mint. Any version of Linux with a fairly recent version of Rust will work. Linux Mint/Ubuntu/Debian/other derivatives should work right out of the box. For other Linux distributions, you may need to adjust the paths for the linking step, which are location in "compiler/x86/src/lib.rs", under the "link" function. I will eventually address this shortcoming.

If you use only the C library, Lila may work on other Unix-like platforms. However, the standard library and non-C versions use Linux system calls for x86-64. Eventually, this will be expanded as I move to other platforms, but for the present, be warned. Windows is not supported at all.

### The Standard Library

Lila has two libraries: the core library, and the standard library.

The core library is statically linked to whatever your programs and libraries. This contains a few core functions such as malloc, free, println, strlen, and others. These are small, self-contained functions that have no dependencies; they make the appropriate system calls. There is an undocumented compiler flag to disable this, but certain language features will fail if you do so.

The standard library has all the nice functions, such as printf, file reading, strings, and so forth. This is dynamically linked.

### Licensing

Lila is licensed under the GPL v2 license (version 2, not 3. Only version 2). I feel that the GPL license best captures my goals for this project.


