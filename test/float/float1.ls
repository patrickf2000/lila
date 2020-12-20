
#OUTPUT
#Your float : 4.232000
#END

#RET 0

extern func printf(s:str, ...)

func print_float(f:float)
begin
    printf("Your float : %f\n", f)
end

func main -> int
    x : float = 4.232
begin
    print_float(x)
    return 0
end

