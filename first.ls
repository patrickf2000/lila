
#OUTPUT
#Str: Hi 1
#Str: Hi 2
#Str: Hi 3
#END

#RET 0

extern func printf(s:str, ...)

func main(argc:int, argv:str[]) -> int
    line : str = "";
    index : int = 0;
begin    
    while index < argc
        line = argv[index];
        printf("Str: %s\n", line);
        
        index = index + 1;
    end
    
    return 0;
end

