
#OUTPUT
#F: 132.322220
#D: 1234.567800
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func get_float -> float
    x : float = 132.32222
begin
    return x
end

func get_double -> double
    x : double = 1234.5678
begin
    return x
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

