
#OUTPUT
#11223344
#aabbccd
#11118888
#END

#RET 0

use std.text_io;

func main -> int
    x : int = 0x11223344;
begin
    printLnHex(x);
    printLnHex(0xAABBCCD);
    
    x = 0x11118888;
    printLnHex(x);
    
    return 0;
end
