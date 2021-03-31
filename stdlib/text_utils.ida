
######################################################################
# A series of utility functions for some of the print functions

module std;

# Gets the number of digits in a number
func numLength(num:int) -> int
    len : int = 0;
begin
    if num < 0
        len = 1;
    end
    
    while num != 0
        num /= 10;
        len += 1;
    end
    
    return len;
end

# Checks to see if a number is negative
func check_neg(num:int) -> int
begin
    if num < 0
        num *= -1;
    end
    
    return num;
end

# Gets the number of hex digits in a number
func getHexLength(num:int) -> int
    len : int = 0;
begin
    while num > 15
        len += 1;
        num /= 16;
    end
    
    len++;
    
    return len;
end

# Converts a digit to a hex digit
func getHexDigit(digit:int) -> byte
begin
    if digit == 1
        return '1';
    elif digit == 2
        return '2';
    elif digit == 3
        return '3';
    elif digit == 4
        return '4';
    elif digit == 5
        return '5';
    elif digit == 6
        return '6';
    elif digit == 7
        return '7';
    elif digit == 8
        return '8';
    elif digit == 9
        return '9';
    elif digit == 10
        return 'a';
    elif digit == 11
        return 'b';
    elif digit == 12
        return 'c';
    elif digit == 13
        return 'd';
    elif digit == 14
        return 'e';
    elif digit == 15
        return 'f';
    end
    
    return '0';
end

