
#OUTPUT
#Str: Hi 1
#Str: Hi 2
#Str: Hi 3
#END

#RET 0

use std.text_io;

func main(argc:int, argv:str[]) -> int
    line : str = "";
    index : int = 0;
    x : int = -92;
begin
    printLnInt(x);
    print("Argc: ");
    printLnInt(argc);

    while index < argc
        line = argv[index];
        printLn2("Str: ", line);
        
        index = index + 1;
    end
    
    return 0;
end

