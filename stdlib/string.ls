
module std;

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
        i = i + 1;
       
        b = b - 48;
        result = result * 10;
        result = result + b;
    end
    
    return result;
end

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

# Join to strings
func strcat(s1:str, s2:str) -> str
    len1 : int = strlen(s1);
    len2 : int = strlen(s2);
    length : int = len1 + len2 + 1;
    index, i2 : int = 0;
    
    new_str : byte[length] = array;
begin
    while index < len1
        new_str[index] = s1[index];
        index = index + 1;
    end
    
    while i2 < len2
        new_str[index] = s2[i2];
        index = index + 1;
        i2 = i2 + 1;
    end
    
    return new_str;
end

# Add character to string
func str_append(s1:str, c:char) -> str
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
