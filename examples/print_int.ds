
extern func printf(s:str, ...)

func print_int(num:int)
    int len = 0
    int i = num
    while i != 0
        i = i / 10
        len = len + 1
    end
    
    byte[len] number = array
    i = num
    int x = len - 1
    while i != 0
        int i2 = i % 10
        i = i / 10
        byte c = i2 + '0'
        number[x] = c
        x = x - 1
    end
    
    printf("Number: %s\n", number)
end

func main -> int
    print_int(20)
    print_int(3)
    print_int(12345)
    return 0
end

