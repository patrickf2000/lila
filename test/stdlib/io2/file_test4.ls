
#OUTPUT
#Hello
#Hi
#oHi
#END

use std.io;

func write_out(fd:int)
begin
    write(fd, "Hello", 5);
    write(fd, "Hi", 2);
end

func clear_array(buf:byte[])
    i : int = 0;
begin
    while i < 5
        buf[i] = 0x0;
        i = i + 1;
    end
end

func main -> int
    fd : int = 0;
    buf : byte[5] = array;
    pos : int64 = -3;
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
    
    clear_array(buf);
    
    read(fd, buf, 2);
    println(buf);
    
    lseek(fd, pos, SEEK_CUR);
    
    clear_array(buf);
    read(fd, buf, 3);
    
    println(buf);
    
    close(fd);
    
    return 0;
end

