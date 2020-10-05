
# Creates and prints two arrays
# Note that the arrays are allocated on the heap and automatically deallocated at exit

#OUTPUT
#Numbers1:
#[0 2 4 6 8 10 12 14 ]
#Numbers2:
#[0 31 62 93 124 155 186 217 ]
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

func init_numbers(list:int[], length:int, seed:int)
    int i = 0
    while i < length
        list[i] = i * seed
        i = i + 1
    end
end

func main -> int
    int[8] numbers1 = array
    int[8] numbers2 = array
    
    init_numbers(numbers1, 8, 2)
    init_numbers(numbers2, 8, 31)
    
    puts("Numbers1:")
    print_numbers(numbers1, 8)
    
    puts("Numbers2:")
    print_numbers(numbers2, 8)
    
    return 0
end

