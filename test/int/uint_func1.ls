
#OUTPUT
#X: -1531590953
#Y: 5614267
#END

#RET 0

use std.text_io;

func print_two(x:uint, y:uint)
begin
    printLnStrInt("X: ", x);
    printLnStrInt("Y: ", y);
end

func main -> int
    x : uint = 0xA4B5C6D7;
begin
    print_two(x, 0x55AABB);
    
    return 0;
end

