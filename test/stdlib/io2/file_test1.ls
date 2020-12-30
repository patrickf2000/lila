
#OUTPUT
#Hello
#END

use std.io;

func main -> int
    fd : int = 0;
    buf : byte[5] = array;
begin
    fd = open("file.txt");
    
    read(fd, buf, 5);
    
    println(buf);
    
    close(fd);
    
    return 0;
end

