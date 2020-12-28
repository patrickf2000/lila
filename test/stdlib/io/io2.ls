
#OUTPUT
#123
#20
#45
#END

#RET 0

use std.text_io;
use std.io;

func main
    num : int = 3;
begin
    printInt(1);
    printInt(2);
    printInt(num);
    printLn("");
    
    num = 20;
    printLnInt(num);
    printLnInt(45);
    
    sys_exit(0);
end
