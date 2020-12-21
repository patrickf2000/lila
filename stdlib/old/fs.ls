
# Provides common file operations

module std

# Reads a file line by line
func get_line(file:int) -> str
    byte[1] buf = array
    int64 length = 0
    char c = 1
    
    while c != 0xA
        read(file, buf, 1)
        c = buf[0]
        
        if c == 0
            break
        end
        
        length = length + 1
    end
    
    if length == 1
        buf[0] = 0x0
        return buf
    end
    
    int64 pos = -length
    lseek(file, pos, 1)
    
    int len = length - 1       # Downcast to 32-bit int
    byte[len] buf2 = array
    read(file, buf2, len)
    
    # Consume the new line
    read(file, buf, 1)
    
    return buf2
end

