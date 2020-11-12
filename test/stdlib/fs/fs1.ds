
#OUTPUT
#Line:Hello, how are you?
#Line:I am good.
#Line:Excellent.
#Line:
#END

#RET 0

use std.io
use std.string
use std.unix
use std.fs

func _start
    int file = open("./file.txt")
    
    if file < 0
        println("Error: Unable to open file.")
        sys_exit(1)
    end
    
    str line = ""
    int length = 1
    
    while length > 0
        line = get_line(file)
        print("Line:")
        println(line)
        
        length = strlen(line)
    end

    close(file)
    sys_exit(0)
end

