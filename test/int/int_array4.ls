
#OUTPUT
#X1: 5
#X2: 1
#X3: 6
#X4: 1
#X5: 1
#END

#RET 0

use std.text_io;

func main -> int
    numbers : int[10] = array;
    base : int = 2;
    x1 : int = 0;
    x2 : int = 0;
    x3 : int = 0;
    x4 : int = 0;
    x5 : int = 0;
begin  
    numbers[1] = 3 + base;
    numbers[2] = 3 - base;
    numbers[3] = 3 * base;
    numbers[4] = 3 / base;
    numbers[5] = 3 % base;
    
    x1 = numbers[1];
    x2 = numbers[2];
    x3 = numbers[3];
    x4 = numbers[4];
    x5 = numbers[5];
    
    printLnStrInt("X1: ", x1);
    printLnStrInt("X2: ", x2);
    printLnStrInt("X3: ", x3);
    printLnStrInt("X4: ", x4);
    printLnStrInt("X5: ", x5);
    
    return 0;
end
