
#OUTPUT
#Hello!
#Yo!
#END

#RET 0

use std.text_io;
use std.arch.x86_64;

func main
    msg : str = "Hello!";
begin
    printLn(msg);
    printLn("Yo!");
    
    syscall(linux_exit, 0);
end

