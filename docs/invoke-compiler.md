## Invoking the Compiler

The compiler has multiple options to help you build the type of binary you need. You can pass multiple inputs, which can be a ".ida" source file or an object file. Here are all the current compiler options:

* --ast/--ltac: See above
* --use-c: Link to C start-up files and the C standard library.
* --lib: Generate a dynamic library
* --pic: Generate position independent code (x86 only- you need this if you are building a library)
* --no-link: Only generate an object file
* -l<lib>: Link to a certain library
* -o <name>: Specify the output name
* --risc: Run the RISC optimizer regardless of platform (the x86 code generator can convert RISC instructions)
