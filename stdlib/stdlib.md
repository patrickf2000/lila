## Ida Standard Library

Contains the functions that should be defined in a Ida runtime system. This is also used to indicated what is currently implemented and what we still need.

The Ida standard library is not essential, but highly recommended. Please see the corresponding documentation for the core library.

### text_io

* printf -> Works similar to C printf (DONE)   
* printInt -> Output an integer (DONE)   
* printHex -> Output an integer as a hex value (DONE)   
* printFloat -> Output a floating point value   
* printDouble -> Output a double value
* readLn -> Read a string from standard input (DONE)   
* readInt -> Read an int from standard input (DONE)   

### file_io

* getByte -> Read a byte from a file (DONE)   
* getLine -> Read a line of text from a file (DONE)   
* writeByte -> Write a byte value to a file   
* writeLine -> Write a line of text to a file   

### io

Note that most of these are wrappers around Linux system calls

* open -> Open a file for reading (DONE)   
* create -> Create a new file (DONE)   
* read -> Read from a file (DONE)   
* write -> Write to a file (DONE)   
* lseek -> Move the position in a file (DONE)   
* close -> Close a file (DONE)   

### os

These are for OS and path-related functions. Again, a lot of these are wrappers around syscalls

* chdir -> Change current working directory for process (DONE)   
* fchdir -> Change current working directory for an file descriptor   

### mem

* resize -> Resize an array   

### string

* str2int -> Convert a string to an integer   
* int2str -> Convert an integer to a string   
* strcat -> Join two strings   
* str_append -> Append a character to a string   

### math

* pow -> Raise a number to a power   
