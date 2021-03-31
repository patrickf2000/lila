
#OUTPUT
#Numbers1:
#[0 2 4 6 8 10 12 14 ]
#Numbers2:
#[0 31 62 93 124 155 186 217 ]
#Result:
#[0 33 66 99 132 165 198 231 ]
#
#END

#RET 0

extern func puts(s:str)
extern func printf(s:str, ...)

func print_numbers(list:int[], length:int)
    i, x : int = 0;
begin
    printf("[");

    while i < length
        x = list[i];
        printf("%d ", x);
        
        i = i + 1;
    end
    
    printf("]\n");
end

func init_numbers(list:int[], length:int, seed:int)
    i : int = 0;
begin
    while i < length
        list[i] = i * seed;
        i = i + 1;
    end
end

func main -> int
    numbers1 : int[8];
    numbers2 : int[8];
    numbers_sum : int[8];
begin
    init_numbers(numbers1, 8, 2);
    init_numbers(numbers2, 8, 31);
    
    puts("Numbers1:");
    print_numbers(numbers1, 8);
    
    puts("Numbers2:");
    print_numbers(numbers2, 8);
    
    numbers_sum = numbers1 + numbers2;
    
    puts("");
    puts("Result:");
    print_numbers(numbers_sum, 8);
    
    return 0;
end

