
use std.text_io;

func help
begin
    printf("Help!!\n");
end

func version
begin
    printf("Version!!\n");
end

func main(argc:int, argv:str[]) -> int
    i : int = 1;
    current : str = "";
begin
    printf("Argc: %d\n\n", argc);
    
    # First, check command line arguments
    while i < argc
        current = argv[i];
        i = i + 1;
        
        if current == "--help"
            help();
            return 0;
        elif current == "--version"
            version();
            return 0;
        else
            printf("%s", current);
        end
    end
    
    printf("\n");
    
    return 0;
end
