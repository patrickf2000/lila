
#OUTPUT
#S1: Hi!
#S2: How are you?
#S3: Hi!
#Len1: 3
#Len2: 12
#Len3: 3
#END

#RET 0

extern func printf(s:str, ...)

func strlen(s:str) -> int
    len : int = 0;
    c : char = 1;
begin
    
    while c != 0
        len = len + 1;
        c = s[len];
    end
    
    return len;
end

func main -> int
    s1 : str = "Hi!";
    s2 : str = "How are you?";
    s3 : str = s1;
    len1, len2, len3 : int = 0;
begin
    
    printf("S1: %s\n", s1);
    printf("S2: %s\n", s2);
    printf("S3: %s\n", s3);
    
    len1 = strlen(s1);
    len2 = strlen(s2);
    len3 = strlen(s3);
    
    printf("Len1: %d\n", len1);
    printf("Len2: %d\n", len2);
    printf("Len3: %d\n", len3);
    
    return 0;
end

