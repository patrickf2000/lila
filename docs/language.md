## Dash Language

This is a very high-level overview of the Dash language. For more practical examples, see the "examples" and "test" directory. Unless otherwise noted, everything here is implemented.

### Types

Dash has the following types:

* byte
* short
* int
* float
* double
* string (str in the code)

We still need these types:

* char
* bool
* ubyte
* ushort
* uint
* uint64/int64

Byte/ubyte are 8-bit (1 byte) types. Short/ushort are 16-bit (2 byte) types. Char will only be for characters, but they will have the same size as byte/ubyte. A bool will represent either true/false (1/0), but will be represented internally as integers.

Note that variables must have an initial value when you create them.

### Arrays

Arrays have this syntax: ```int[10] numbers = array```

In this example, we create an integer array of 10 elements. The array means it is dynamically created in the heap.

Note that Dash will automatically insert free statements for each array created.

Currently only byte/int arrays are created.

### Operators

For all numerical types, we have the following math operators: +, -, *, and /.

For the byte/short/int types, we also have % (modulo), &, |, ^,  <<, >> (and, or, xor, left-shift, right-shift respectively).

For comparisons, we have the usual ==, !=, >, <, >=, <=. 

Order of operations are not supported, but planned.

### Functions

Functions generally have this syntax:

```
func main(arg1:int, arg2:byte) -> int
 .....
 return 0
end
```

The arrow with the int designates the return type. Note that the return type and arguments are not strictly required. If no return type is specified, it returns void. So a function like this works too:

```
func main
....
end
```

Functions are called like in any other language:

```
myFunc(arg1, arg2)
```

### Conditionals

Conditionals have the syntax: ```(value) (comparison operator) (value)```. Single value comparisons are planned but not supported.

Conditionals use if/elif/else. A conditional block looks like this:

```
if x > 5  
.....
elif x < 5
.....
else
....
end
```

### Loops

Currently, only while loops are supported. They have the syntax like the conditional statements. A while loop may look like this:

```
while i < 10
    ....
end
```

The "break" and "continue" keywords are implemented and work in loops.

### Other Features

If you wish to make a system call, you can use the "syscall" construct. It works exactly like a function call. Note that only integer and string parameters are accepted. The system call for exit on x86-64 Linux is:

```
func main
    # The return code
    int rc = 5
    
    syscall(60, rc)
end
```

This is highly experimental and currently only works on x86-64. You can do math on integer arrays, which uses SIMD (vectorization). There is an example in test/vector. Full implementation of this feature is planned.

