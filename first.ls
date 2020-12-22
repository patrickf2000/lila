
extern func printf(s:str, ...)

const int linux_read = 0;
const int linux_write = 1;
const int linux_open = 2;
const int linux_close = 3;
const int linux_lseek = 8;
const int linux_exit = 60;

const int STDOUT = 1;
const int STDIN = 1;

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

# Join to strings
func strcat(s1:str, s2:str) -> str
    len1 : int = strlen(s1);
    len2 : int = strlen(s2);
    length : int = len1 + len2 + 1;
    index, i2 : int = 0;
    
    b1 : byte = 0;
    new_str : byte[length] = array;
begin
    while index < len1
        b1 = len1[index];
        new_str[index] = b1;
        index = index + 1;
    end
    
    while i2 < len2
        b1 = len2[i2];
        new_str[index] = b1;
        index = index + 1;
        i2 = i2 + 1;
    end
    
    return new_str;
end

func main -> int
    len : int = 0;
    str1 : str = "Hello!";
    str2 : str = " How are you?";
    str3 : str = strcat(str1, str2);
begin
    len = strlen(str1);
    printf("Length1: %d\n", len);
    
    len = strlen(str2);
    printf("Length2: %d\n", len);
    
    printf("Concat: %s\n", str3);
    
    return 0;
end

