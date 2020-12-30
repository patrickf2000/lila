
module core;

# String length
func strlen(s:str) -> int
    i, length : int = 0;
    c : char = 0;
begin
    c = s[i];
    
    while c != 0
        length = length + 1;
        c = s[i];
        
        if c == '\'
            i = i + 1;
        end
        i = i + 1;
    end
    
    if length > 0
        length = length - 1;
    end
    return length;
end

# Return 1 if same, 0 if not
func strcmp(s1:str, s2:str) -> int
    length, len1, len2 : int = 0;
    c1, c2 : char = 0;
begin
    len1 = strlen(s1);
    len2 = strlen(s2);
    
    if len1 != len2
        return 0;
    end
    
    len1 = 0;
    while len1 < len2
        c1 = s1[len1];
        c2 = s2[len1];
        
        if c1 != c2
            return 0;
        end
        
        len1 = len1 + 1;
    end
    
    return 1;
end

