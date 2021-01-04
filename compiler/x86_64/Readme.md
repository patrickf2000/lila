
## Codegen2 -> x86-64

This is a small experimental layer for a new type of code generator. Basically, the idea behind this will be to introduce a new IR on the final code generation level that matches the architecture. This IR will hopefully make certain elements of LTAC translation easier, and could eventually allow for architecture-specific optimizations

This IR will not use the C backend.
