
#OUTPUT
#X1: 22
#X2: 132
#END

#RET 0

use std.text_io;

func main -> int
    x : int = 22;
    y : int = 3;
begin
    
    printLnStrInt("X1: ", x);

    x = 44 * y;
    
    printLnStrInt("X2: ", x);
    
    return 0;
end
