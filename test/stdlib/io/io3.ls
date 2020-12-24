
#OUTPUT
#Hi, how are you?
#> $
#ls
#> ls
#END

#RET 0

use std.text_io;
use std.io;

func _start
    line : str = "> ";
    line2 : str = "ls";
begin
    print("Hi, ");
    print("how are you?");
    printLn("");
    
    print(line);
    printLn("$");
    
    printLn(line2);
    
    printLn2(line, line2);
    
    sys_exit(0);
end