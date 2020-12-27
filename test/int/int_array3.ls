
#OUTPUT
#Numbers1:
#[0 2 4 6 8 10 12 14 ]
#Numbers2:
#[0 31 62 93 124 155 186 217 ]
#END

#RET 0

use std.text_io;

func print_numbers(list:int[], length:int)
    i : int = 0;
    x : int = 0;
begin
    print("[");

    while i < length
        x = list[i];
        
        printInt(x);
        print(" ");
        
        i = i + 1;
    end
    
    printLn("]");
end

func init_numbers(list:int[], length:int, seed:int)
    i : int = 0;
begin

    while i < length
        list[i] = i * seed;
        i = i + 1;
    end
end

func main -> int
    numbers1 : int[8] = array;
    numbers2 : int[8] = array;
begin
    
    init_numbers(numbers1, 8, 2);
    init_numbers(numbers2, 8, 31);
    
    printLn("Numbers1:");
    print_numbers(numbers1, 8);
    
    printLn("Numbers2:");
    print_numbers(numbers2, 8);
    
    return 0;
end

