
#OUTPUT
#Result: 42
#END

#RET 0

use std.text_io;

func add_two(x:int, y:int)
    answer : int = x + y;
begin
    printLnStrInt("Result: ", answer);
end

func main -> int
    x : int = 22;
begin
    add_two(20, x);
    return 0;
end
