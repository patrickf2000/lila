
#OUTPUT
#Str: Hi 1
#Str: Hi 2
#Str: Hi 3
#END

#RET 0

module default;

use std.arch.x86_64 if "x86_64";
#use std.arch.x86_64 if "riscv64";
#use std.arch.x86_64;

func main(argc:int, argv:str[]) -> int
    line : str = "";
    index : int = 0;
begin
    printf("Hello! % %d, %x, %c, %s\n", 6, 10, 'Z', "Yo!");
    printf("5 % 6 = %d\n", 67);
    printLn("");
    
    print("Argc: ");
    printLnInt(argc);

    while index < argc
        line = argv[index];
        printLn2("Str: ", line);
        
        index = index + 1;
    end
    
    return 0;
end

