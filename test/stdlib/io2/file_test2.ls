
#OUTPUT
#Invalid
#END

#RET 1

use std.io;

func main -> int
    fd : int = 0;
begin
    fd = open("./first.txt");
    if fd < 0
        println("Invalid");
        return 1;
    end
    
    println("FAIL!!!!!");
    
    close(fd);
    return 0;
end
