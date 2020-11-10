
#OUTPUT
#T32: 5
#T32: 105
#Tu32: 5
#Tu32: 105
#T64: 5
#T64: 105
#Tu64: 5
#Tu64: 105
#END

#RET 0

extern func printf(s:str, ...)

# All the get number functions
func get_num32 -> int
    int x = 20
    int y = 30
    int answer = x + y
    return answer
end

func get_numu32 -> uint
    uint x = 20
    uint y = 30
    uint answer = x + y
    return answer
end

func get_num64 -> int64
    int64 x = 20
    int64 y = 30
    int64 answer = x + y
    return answer
end

func get_numu64 -> uint64
    uint64 x = 20
    uint64 y = 30
    uint64 answer = x + y
    return answer
end

# Output should be 5, then 105
func test1
    int total = 0
    
    total = total + 5
    printf("T32: %d\n", total)
    
    total = total + get_num32()
    total = total + get_num32()
    
    printf("T32: %d\n", total)
end

func test2
    uint total = 0
    
    total = total + 5
    printf("Tu32: %d\n", total)
    
    total = total + get_numu32()
    total = total + get_numu32()
    
    printf("Tu32: %d\n", total)
end

func test3
    int64 total = 0
    
    total = total + 5
    printf("T64: %d\n", total)
    
    total = total + get_num64()
    total = total + get_num64()
    
    printf("T64: %d\n", total)
end

func test4
    uint64 total = 0
    
    total = total + 5
    printf("Tu64: %d\n", total)
    
    total = total + get_numu64()
    total = total + get_numu64()
    
    printf("Tu64: %d\n", total)
end

func main -> int
    test1()
    test2()
    test3()
    test4()
    
    return 0
end

