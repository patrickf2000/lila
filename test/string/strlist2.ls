
#OUTPUT
#Str: Hi 1
#Str: Hi 2
#Str: Hi 3
#END

#RET 0

extern func printf(s:str, ...)

func print_list(size:int, strlist:str[])
    index : int = 0;
    line : str = "";
begin
    while index < size
        line = strlist[index];
        printf("Str: %s\n", line);
        
        index = index + 1;
    end
end

func main -> int
    strlist : str[3] = array;
    line : str = "";
    index : int = 0;
begin
    strlist[0] = "Hi 1";
    strlist[1] = "Hi 2";
    strlist[2] = "Hi 3";
    
    print_list(3, strlist);
    
    return 0;
end

