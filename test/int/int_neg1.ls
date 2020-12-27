
#OUTPUT
#X: -15
#X: -9
#Y: 20
#X: 9
#Y: -9
#X: 10
#Y: 10
#END

#RET 0

use std.text_io;

func test1
    x : int = -6 + -9;
begin
    printLnStrInt("X: ", x);
end

func test2
    x : int = -9;
    y : int = 20 + x - -9;
begin
    printLnStrInt("X: ", x);
    printLnStrInt("Y: ", y);
end

func test3
    x : int = 9;
    y : int = -x;
begin  
    printLnStrInt("X: ", x);
    printLnStrInt("Y: ", y);
end

func test4
    x : int = 10;
    y : int = -x + 30 + -x;
begin
    printLnStrInt("X: ", x);
    printLnStrInt("Y: ", y);
end

func main -> int
begin
    test1();
    test2();
    test3();
    test4();

    return 0;
end
