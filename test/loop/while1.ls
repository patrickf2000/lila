#OUTPUT
#Num: 0
#Num: 1
#Num: 2
#Num: 3
#Num: 4
#Num: 5
#Num: 6
#Num: 7
#Num: 8
#Num: 9
#Num: 10
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : int = 0
begin
    while x <= 10 
        printf("Num: %d\n", x)
        x = x + 1
    end
    
    return 0
end
