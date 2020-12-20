
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
    x : int = 20
    y : int = 30
    answer : int = 0
begin
    answer = x + y
    return answer
end

func get_numu32 -> uint
    x : uint = 20
    y : uint = 30
    answer : uint = 0
begin
    answer = x + y
    return answer
end

func get_num64 -> int64
    x : int64 = 20
    y : int64 = 30
    answer : int64 = 0
begin
    answer = x + y
    return answer
end

func get_numu64 -> uint64
    x : uint64 = 20
    y : uint64 = 30
    answer : uint64 = 0
begin
    answer = x + y
    return answer
end

# Output should be 5, then 105
func test1
    total : int = 0
begin
    total = total + 5
    printf("T32: %d\n", total)
    
    total = total + get_num32()
    total = total + get_num32()
    
    printf("T32: %d\n", total)
end

func test2
    total : uint = 0
begin
    total = total + 5
    printf("Tu32: %d\n", total)
    
    total = total + get_numu32()
    total = total + get_numu32()
    
    printf("Tu32: %d\n", total)
end

func test3
    total : int64 = 0
begin
    total = total + 5
    printf("T64: %d\n", total)
    
    total = total + get_num64()
    total = total + get_num64()
    
    printf("T64: %d\n", total)
end

func test4
    total : uint64 = 0
begin
    total = total + 5
    printf("Tu64: %d\n", total)
    
    total = total + get_numu64()
    total = total + get_numu64()
    
    printf("Tu64: %d\n", total)
end

func main -> int
begin
    test1()
    test2()
    test3()
    test4()
    
    return 0
end

