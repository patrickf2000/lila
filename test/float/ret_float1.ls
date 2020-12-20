
#OUTPUT
#F: 5.443000
#D: 2.000010
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func get_float -> float
begin
    return 5.443
end

func get_double -> double
begin
    return 2.00001
end

func main -> int
    f : float = 0.0
    d : double = 0.0
begin
    f = get_float()
    printf("F: %f\n", f)
    
    d = get_double()
    printf("D: %f\n", d)
    
    return 0
end

