
#OUTPUT
#*
#Num: 1
#**
#Num: 2
#***
#Num: 3
#****
#Num: 4
#****|*
#Num: 6
#****|**
#Num: 7
#****|***
#Num: 8
#****|***|
#Num: 9
#****|***|**
#Num: 11
#****|***|***
#Num: 12
#****|***|***|
#Num: 13
#****|***|***|*
#Num: 14
#****|***|***|**
#Num: 16
#****|***|***|**
#Num: 17
#****|***|***|**
#Num: 18
#****|***|***|**
#Num: 19
#****|***|***|**
#Num: 20
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x, i : int = 0;
begin
    while x < 20 
        x = x + 1;
        
        if x == 5
            continue;
        elif x == 10
            continue;
        elif x == 15
            continue;
        end
        
        i = 0;
        while i < x
            if i == 15
                break;
            end
            
            if i == 4
                printf("|");
            elif i == 8
                printf("|");
            elif i == 12
                printf("|");
            else
                printf("*");
            end
            
            i = i + 1;
        end
        println("");
        
        printf("Num: %d\n", x);
    end
    
    return 0;
end
