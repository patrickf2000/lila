## Dash

Welcome to Dash! Dash is a high-level imperative systems programming language inspired in design and purpose by C and syntactically by BASIC (and a few other languages). This repository contains the compiler for Dash, the standard library, the test system, and a few examples.

### Why Dash?

I started Dash as a descendant to my Quik compilers. The original Quik was when I was learning compiler development, so they weren't designed with any big purpose. I started Dash because I liked the Quik language I came up with, and because I needed a simple compiler so I could experiment with different architectures, learn a little more about compiler transformations, and so forth. I also started the project in hopes I can use this for some benchmarking work I do in one of my jobs.

However, Dash is not solely a personal thing. The first goal with Dash is simplicity- in the language itself and in the implementation. Dash is meant to be like C in that its basically portable assembly. The language should be super easy to port to any platform, whether a CPU or VM architecture (source interpretation should be easy as well). However, I do not wish to re-invent the wheel, so part of Dash is full interpolation with C and the system libraries.

They key difference of Dash from C is the language design and the abstracting of certain concepts. For example, safety is a big goal. There are no pointers and references in C. Any programming construct requiring pointers and references is abstracted away. Dash also aims to provide better support for things like strings and data structures, two things which I think are very lacking in C. On the hardware level, I also have a goal of providing constructs to make newer hardware features easier to use, including threading and SIMD. And finally, since I like living close the hardware and the operating system, I have constructs to make it easy to interface with the underlying OS.

See the "docs" folder for why I chose Rust.

### Features

All the stuff here is either completely implemented

* Complete compatibility with C objects and libraries
* Super simple to parse and understand
* No ambiguous grammar
* Easy to translate to underlying architectures
* Default x86-64 (others planned)
* Optimization layer for RISC architectures
* Support for all numerical types (signed and unsigned are separate types)
* Automated test system
* Syntax checking and reporting
* Experimental SIMD support- no intrinsics! (not fully implemented however...)
* Automatic memory allocation and deallocation
* Module system instead of headers

### Architecture Support

Currently, I only support x86-64. The internal representations generate an CISC-like assembly, which lends itself well to x86. I just finished an optimization layer that converts the IR to a RISC-like assembly, which will allow for very easy porting to architectures such as Arm, MIPS, and PowerPC. See the "docs" folder for creating a new backend.

For x86, I currently use the GNU Assembler; however, I plan to drop that and use my own "as" assembler project in the very near future for final assembly (that will probably be my next step).

### System Requirements

All development was done on Linux Mint. Any version of Linux with a fairly recent version of Rust will work. Linux Mint/Ubuntu/Debian/other derivatives should work right out of the box. For other Linux distributions, you may need to adjust the paths for the linking step, which are location in "compiler/x86/src/lib.rs", under the "link" function. I will eventually address this shortcoming.

If you use only the C library, Dash may work on other Unix-like platforms. However, the standard library and non-C versions use Linux system calls for x86-64. Eventually, this will be expanded as I move to other platforms, but for the present, be warned. Windows is not supported at all.

### Compiler Internals

The of the goals of this project is for the compiler to be easy to understand and hack on. All the parsing code is located under "parser"; the transform code, which optimizes for RISC if needed and converts pseudo-instructions such as malloc and free are located under "transform". The code generators are located under "compiler".

The compiler uses two intermediate representations (IR): the abstract syntax tree (AST) and the the pseudo-assembly layer (LTAC). The AST is created from the source file; you can see what the IR looks like from the "parser/src/ast.rs" file. The LTAC IR is where most of the work happens. LTAC is very strongly typed, so there are a lot of instructions. This is also partly because we need to be able to easily port it to different architectures. You can understand the IR by looking at the "parser/src/ltac.rs" file.

To understand how sources are represented during compilation, you can use the "--ast" and "--ltac" compiler flags. The "--ast" flag will load the source into an AST and print it to the console. The "--ltac" flag will output the LTAC code to a file named after your source. The transform layer is still run when you use the "--ltac" flag. If you wish to see the equivalent RISC code regardless of your platform, use the "--risc" flag.

### The Standard Library

There's currently a small standard library with a few commonly used procedures and functions. This will be expanded in time. In practice, developing this has shown the shortcomings of the language, so I've had to take breaks to work on compiler improvements... Oh well. Its what makes it fun.

### Invoking the Compiler

Unfortunately, I don't have a "--help" yet... I'm sorry... However, it works similar to most other compilers. You can pass multiple inputs, which can be a ".ds" source file or an object file. Here are all the current compiler options:

* --ast/--ltac: See above
* --use-c: Link to C start-up files and the C standard library.
* --lib: Generate a dynamic library
* --pic: Generate position independent code (x86 only- you need this if you are building a library)
* --no-link: Only generate an object file
* -l<lib>: Link to a certain library
* -o <name>: Specify the output name
* --risc: Run the RISC optimizer regardless of platform (the x86 code generator can convert RISC instructions)

### Modules

Rather than headers, Dash uses a simple module system. To get an understanding of how it works, take a look at the stdlib folder for how the modules are generated, and the "test/stdlib" folder for how to use them.

I'll provide more documentation later as it gets more solidified.

### Testing

There are a lot of tests (at the time of writing, I think over 180). In order to make sure I don't break things, I use a unit-test approach, which basically is a bunch of very small programs that test a certain construct. The tests are divided among the different data types and features. To run, simply run the "./test.sh" script. If you need to test the RISC layer, run it as "./test.sh --risc".

All tests use the C library for things like output. NEVER use the standard library for testing; there's a separate script to test that. The point of the unit tests is to test only one thing, and to eliminate the potential surface area of other bugs.


