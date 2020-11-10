
#OUTPUT
#Hello!
#Your number: 20, 30, 40, 50
#END

#RET 0

use std.io
use std.arch.x86_64

func _start
    str msg = "Hello!"
    println(msg)
    
    println("Your number: %i, %i, %i, %i", 20, 30, 40, 50)
    
    syscall(linux_exit, 0)
end

