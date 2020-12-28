
#OUTPUT
#Hello!
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
begin
    printf("Hello!\n");
    
    return 0;
end

