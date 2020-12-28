
#OUTPUT
#Also correct!
#Idk!!
#Hello!
#END

#RET 0

extern func puts(s:str)

func main() -> int
    x : int = 3;
begin
    
    if x == 5
        puts("Correct");
    elif x == 3
        puts("Also correct!");
        if x == 3
            puts("Idk!!");
        end
    else
        puts("Idk");
    end

    puts("Hello!");
    return 0;
end

