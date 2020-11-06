

func strlen(s:str) -> int
    int len = 0
    int i = 0
    char c = 1
    
    while c != 0
        len = len + 1
        c = s[i]
        
        if c == '\'
            i = i + 1
        end
        i = i + 1
    end
    
    return len
end

func println(s:str)
    int len = strlen(s)
    syscall(1, 1, s, len)
end

func _start
    str s1 = "Hi!\n"
    str s2 = "How are you?\n"
    str s3 = s1
    
    println(s1)
    println(s2)
    println(s3)
    
    syscall(60, 0)
end

