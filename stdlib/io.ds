# Common IO functions

module std

use std.arch.x86_64
use std.string

# Exit
func sys_exit(code:int)
    syscall(linux_exit, code)
end

# Print integers
func print_int(num:int)
    int len = 0
    int i = num
    while i != 0
        i = i / 10
        len = len + 1
    end
    
    byte[len] number = array
    i = num
    int x = len - 1
    while i != 0
        int i2 = i % 10
        i = i / 10
        byte c = i2 + '0'
        number[x] = c
        x = x - 1
    end
    
    syscall(linux_write, STDOUT, number, len)
end

func clear_array(data:byte[], len:int)
    int i = 0
    char null = 0
    while i < len
        data[i] = null
        i = i + 1
    end
end

func print(msg:str, ...)
    uint64 arg1 = ldarg(2, uint64)
    uint64 arg2 = ldarg(3, uint64)
    uint64 arg3 = ldarg(4, uint64)
    uint64 arg4 = ldarg(5, uint64)
    int arg_index = 0

    int len = strlen(msg)
    char[len] msg2 = array
    
    int i = 0
    int pos = 0
    while i < len
        char c = msg[i]
        
        # Check to see if we have a format specifier
        if c == '%'
            i = i + 1
            c = msg[i]
            
            # Print integers
            if c == 'i'
               syscall(linux_write, STDOUT, msg2, pos)
               pos = 0
               clear_array(msg2, len)
               
               if arg_index == 0
                   int num = arg1
                   print_int(num)
               elif arg_index == 1
                   int num = arg2
                   print_int(num)
               elif arg_index == 2
                   int num = arg3
                   print_int(num)
               elif arg_index == 3
                   int num = arg4
                   print_int(num)
               end
               
               arg_index = arg_index + 1
            end
            
        # Otherwise, just add the character to be printed
        else
            msg2[pos] = c
            pos = pos + 1
        end
        
        i = i + 1
    end
    
    pos = pos - 1
    syscall(linux_write, STDOUT, msg2, pos)
end

# This works because of the argument registers; they shouldn't be modified
func println(msg:str, ...)
    print(msg)
    print("\n")
end

