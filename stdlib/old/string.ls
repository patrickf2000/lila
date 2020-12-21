
module std

func strlen(s:str) -> int
    int len = 0
    int i = 0
    char c = s[i]
    
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

