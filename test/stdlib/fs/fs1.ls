
#OUTPUT
#Hello, how are you?
#I am good.
#Excellent.
#
#END

#RET 0

use std.string;
use std.io;
use std.fs;
use std.text_io;

func main
    file : int = 0;
    c : char = 1;
    buf : byte[1] = array;
    
    len, num : int = 1;
    line : str = "";
begin
    file = open("./file.txt");
    if file < 0
        println("Unable to open file.");
        exit;
    end
    
    while len > 0
        line = getLine(file);
        println(line);
        
        len = strlen(line);
    end
    
    close(file);
    exit;
end
