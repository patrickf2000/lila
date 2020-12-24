
#OUTPUT
#Str: Hi 1
#Str: Hi 2
#Str: Hi 3
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    strlist : str[3] = array;
    line : str = "";
    index : int = 0;
begin
    strlist[0] = "Hi 1";
    strlist[1] = "Hi 2";
    strlist[2] = "Hi 3";
    
    while index < 3
        line = strlist[index];
        printf("Str: %s\n", line);
        
        index = index + 1;
    end
    
    return 0;
end

