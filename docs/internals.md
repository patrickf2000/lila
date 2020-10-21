## Dash Internals

This is meant to give a quick overview of how Dash works internally. Part of the purpose of this project is to have a simple compiler and make it easy to port and hack on. Even if you have a minimal compiler background, it should be easy to understand.

### The Parser

The parser is contained in one library and contains two intermediate representations (IR). The first IR is the abstract syntax tree, which is built from the source, and the LTAC, which is built from the AST and translated to assembly.

The AST is the internal representation of the source program. The tree never goes very deep, and is almost exactly the same as the source file. The purpose of this layer is to serve as a starting point and catch as many syntax errors as possible.

The LTAC layer is basically portable assembly (it stands for Low-level Three-Address Code). The LTAC tree is built directly from the AST. LTAC is generally designed to map directly to any architecture, but there are a few instructions that may not completely map (by design). LTAC is meant for the final code generation and for any optimizations (the AST is also suited for optimizations, but optimizing is not a major goal of this project).

### The Transform Layer

By default, LTAC generates the portable assembly modeled after CISC architectures. There may be some instructions that don't natively exist on CPUs, such as malloc and free. This layer takes care of that. This is also the layer where any optimizations would take place.

By default, there are two operations in the transform layer. The first translates non-native instructions into native instructions. Currently, this is only the malloc, free, and exit instructions. The method of transformation depends on whether or not the user wishes to use the C library. If the C library is used, this is nothing more than function calls to malloc, free, and exit respectively. If the C library is disabled, this will be translated into Linux system calls.

The second transform is the RISC optimizer. This transforms the LTAC code into RISC-style code (pretty much this means moving all memory references to separate load/store instructions).

### The Compiler

The compiler layer translates LTAC into assembly. Currently, I have complete support for x86-64 and almost complete support for AArch64 (Arm 64-bit).

