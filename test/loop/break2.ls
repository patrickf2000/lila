#OUTPUT
#*
#Num: 0
#**
#Num: 1
#***
#Num: 2
#****
#Num: 3
#*****
#Num: 4
#******
#Num: 5
#*******
#Num: 6
#********
#Num: 7
#*********
#Num: 8
#**********
#Num: 9
#**********
#Num: 10
#**********
#Num: 11
#**********
#Num: 12
#**********
#Num: 13
#**********
#Num: 14
#**********
#Num: 15
#**********
#Num: 16
#**********
#Num: 17
#**********
#Num: 18
#**********
#Num: 19
#**********
#Num: 20
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x, i : int = 0;
begin 
    while x <= 30 
        if x > 20
            break;
        end
        
        i = 0;
        while i <= x
            printf("*");
            i = i + 1;
            
            if i == 10
                break;
            end
        end
        puts("");
        
        printf("Num: %d\n", x);
        x = x + 1;
    end
    
    return 0;
end
