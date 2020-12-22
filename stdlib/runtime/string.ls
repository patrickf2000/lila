
module std;

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
    
    return length;
end
