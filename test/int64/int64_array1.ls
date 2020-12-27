
#OUTPUT
#X: 22
#X: 25
#END

#RET 0

use std.text_io;

func test1
    numbers : int64[10] = array;
    x : int64 = 0;
begin
    
    numbers[3] = 22;
    
    x = numbers[3];
    
    printLnStrInt("X: ", x);
end

func test2
    numbers : int64[10] = array;
    i : int = 5;
    x : int64 = 0;
begin

    numbers[i] = 25;
    
    x = numbers[i];
    
    printLnStrInt("X: ", x);
end

func main -> int
begin
    test1();
    test2();
    
    return 0;
end
