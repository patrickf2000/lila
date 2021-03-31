
#OUTPUT
#T: 5
#T: 105
#END

#RET 0

extern func printf(s:str, ...)

func get_num -> int
    x : int = 20;
    y : int = 30;
    answer : int = x + y;
begin
    return answer;
end

# Output should be 5, then 105
func main -> int
    total : int = 0;
begin
    total = total + 5;
    printf("T: %d\n", total);
    
    total = total + get_num();
    total = total + get_num();
    
    printf("T: %d\n", total);
    
    return 0;
end

