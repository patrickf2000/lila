
#OUTPUT
#X: 4.232000
#Your double : 4.232000
#END

#RET 0

extern func printf(s:str, ...)

func print_double(d:double)
begin
    printf("Your double : %f\n", d)
end

func main -> int
    x : double = 4.232
begin
    printf("X: %f\n", x)
    
    print_double(x)
    return 0
end

