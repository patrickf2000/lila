
#OUTPUT
#Correct!
#END

#RET 0

func _start
    numbers : int[10];
    x : int = 0;
begin
    numbers[3] = 22;
    
    x = numbers[3];
    
    if x == 22
        syscall(1, 1, "Correct!\n", 9);
    end
    
    exit;
end


