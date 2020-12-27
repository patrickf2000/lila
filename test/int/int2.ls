
#OUTPUT
#287454020
#286361736
#END

#RET 0

extern func puts(s:str)

func main -> int
    x : int = 0x11223344;
begin
    printLnInt(287454020);
    
    x = 0x11118888;
    printLnInt(286361736);
    
    return 0;
end
