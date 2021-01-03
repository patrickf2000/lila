
# cat
# A simple example of the UNIX cat program

use std.text_io;
use std.file_io;
use std.io;

func main(args:str[]) -> int
    length : int = sizeof(args);
    file : int = 0;
    path : str = "";
    c : char = 1;
begin
    if length == 1
        println("Error: No input specified.");
        return 1;
    end
    
    path = args[1];
    
    file = open(path);
    if file < 0
        println("Unable to open input.");
        println("Please make sure the file exists.");
        return 1;
    end
    
    while c != 0
        c = getChar(file);
        printf("%c", c);
    end
    
    close(file);
    
    return 0;
end

