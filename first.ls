
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
    
    length = length - 1;
    return length;
end

# Add character to string
func append(s1:str, c:char) -> str
    len1 : int = strlen(s1);
    length : int = len1 + 2;
    index : int = 0;
    new_str : byte[length] = array;
begin
    while index < len1
        new_str[index] = s1[index];
        index = index + 1;
    end
    
    new_str[index] = c;
    new_str[index+1] = 0;
    
    return new_str;
end

func main -> int
    len : int = 0;
    str1 : str = "Hello";
    str2 : str = " How are you?";
    str3 : str = append(str1, '!');
begin
    len = strlen(str1);
    printf("Length1: %d\n", len);
    
    len = strlen(str2);
    printf("Length2: %d\n", len);
    
    printf("Concat: %s\n", str3);
    
    return 0;
end

