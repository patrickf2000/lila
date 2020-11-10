
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
    printf("[")

    int i = 0
    while i < length
        int x = list[i]
        printf("%d ", x)
        
        i = i + 1
    end
    
    printf("]\n")
end

func init_numbers(list:int[], len:int, seed:int)
    int i = 0
    while i < len
        list[i] = i * seed
        i = i + 1
    end
end

func main -> int
    int[16] numbers = array
    int[8] numbers_sum = array
    
    init_numbers(numbers, 16, 2)
    
    puts("Numbers:")
    print_numbers(numbers, 16)
    
    int x = 8
    numbers_sum = numbers + numbers[x]
    
    puts("")
    puts("Result:")
    print_numbers(numbers_sum, 8)
    
    return 0
end

