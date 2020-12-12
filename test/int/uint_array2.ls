
#OUTPUT
#0
#2
#4
#6
#8
#10
#12
#14
#16
#18
#Done
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    uint[10] numbers = array
    
    uint i = 0
    while i < 10
        numbers[i] = i * 2
        i = i + 1
    end
    
    i = 0
    while i < 10
        int x = numbers[i]
        printf("%d\n", x)
        i = i + 1
    end
    
    printf("Done\n")
    
    return 0
end

