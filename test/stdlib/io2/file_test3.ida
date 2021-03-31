
#OUTPUT
#Hello
#END

use std.io;

func write_out(fd:int)
begin
    write(fd, "Hello", 5);
end

func main -> int
    fd : int = 0;
    buf : byte[5];
begin
    fd = create("./first.txt");
    if fd < 0
        return 1;
    end
    
    write_out(fd);
    close(fd);
    
    fd = open("./first.txt");
    
    read(fd, buf, 5);
    
    println(buf);
    
    close(fd);
    
    return 0;
end

