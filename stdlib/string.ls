
module std;

use core.string;

# Convert string to int
# TODO: This extra num variable makes it worse, I think something
# about memory alignment. We need to figure this out
#
func str2int(s:str) -> int
    result, length, i, num : int = 0;
    b : byte = 0x0;
begin
    length = strlen(s);
    
    if length == 1
        b = s[0];
        result = b - 48;
        return result;
    end
    
    while i < length
        b = s[i];
        i++;
       
        b -= 48;
        result *= 10;
        result += b;
    end
    
    return result;
end

# Join to strings
func strcat(s1:str, s2:str) -> str
    len1 : int = strlen(s1);
    len2 : int = strlen(s2);
    length : int = len1 + len2 + 1;
    index, i2 : int = 0;
    
    new_str : byte[length];
begin
    while index < len1
        new_str[index] = s1[index];
        index++;
    end
    
    while i2 < len2
        new_str[index] = s2[i2];
        index++;
        i2++;
    end
    
    return new_str;
end

# Add character to string
func str_append(s1:str, c:char) -> str
    len1 : int = strlen(s1);
    length : int = len1 + 2;
    index : int = 0;
    new_str : byte[length];
begin
    while index < len1
        new_str[index] = s1[index];
        index++;
    end
    
    new_str[index] = c;
    new_str[index+1] = 0;
    
    return new_str;
end

