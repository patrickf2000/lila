
#OUTPUT
#Hello!
#How are you?
#END

#RET 0

use std.text_io;

func main -> int
    msg1 : str = "Hello!\n";
    msg2 : str = "How are ";
begin
    printf(msg1);
    printf(msg2);
    printf("you?\n");
    
    return 0;
end

