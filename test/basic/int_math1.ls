
#OUTPUT
#X: 20
#x + 5 = 25
#x - 5 = 15
#x * 5 = 100
#x / 5 = 4
#x % 6 = 2
#X = 4
#x & 5 = 4
#x | 5 = 5
#x ^ 5 = 1
#x << 2 = 16
#x >> 2 = 1
#END

#RET 0

use std.text_io;

func test1
    x : int = 20;
    a1, a2, a3, a4, a5 : int = 0;
begin
    a1 = x + 5;
    a2 = x - 5;
    a3 = x * 5;
    a4 = x / 5;
    a5 = x % 6;

    printLnStrInt("X: ", x);
    printLnStrInt("x + 5 = ", a1);
    printLnStrInt("x - 5 = ", a2);
    printLnStrInt("x * 5 = ", a3);
    printLnStrInt("x / 5 = ", a4);
    printLnStrInt("x % 6 = ", a5);
end

func test2
    x : int = 4;
    a1, a2, a3, a4, a5 : int = 0;
begin
    a1 = x & 5;
    a2 = x | 5;
    a3 = x ^ 5;
    a4 = x << 2;
    a5 = x >> 2;

    printLnStrInt("X = ", x);
    printLnStrInt("x & 5 = ", a1);
    printLnStrInt("x | 5 = ", a2);
    printLnStrInt("x ^ 5 = ", a3);
    printLnStrInt("x << 2 = ", a4);
    printLnStrInt("x >> 2 = ", a5);
end

func main -> int
begin
    test1();
    test2();
    return 0;
end

