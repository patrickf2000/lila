
#OUTPUT
#Numbers:
#[0 2 4 6 8 10 12 14 16 18 20 22 24 26 28 30 ]
#Result:
#[16 20 24 28 32 36 40 44 ]
#
#END

#RET 0

extern func puts(s:str)
extern func printf(s:str, ...)

func print_numbers(list:int[], length:int)
    i, x : int = 0
begin
    printf("[")

    while i < length
        x = list[i]
        printf("%d ", x)
        
        i = i + 1
    end
    
    printf("]\n")
end

func init_numbers(list:int[], len:int, seed:int)
    i : int = 0
begin
    while i < len
        list[i] = i * seed
        i = i + 1
    end
end

func main -> int
    numbers : int[16] = array
    numbers_sum : int[8] = array
    x : int = 8
begin
    init_numbers(numbers, 16, 2)
    
    puts("Numbers:")
    print_numbers(numbers, 16)
    
    numbers_sum = numbers + numbers[x]
    
    puts("")
    puts("Result:")
    print_numbers(numbers_sum, 8)
    
    return 0
end

