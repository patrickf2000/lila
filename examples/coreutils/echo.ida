
# Example of the echo program

use std.text_io;

func help
begin
    println("echo v0.1");
    println("");
    println("-h, --help -> Display this message and exit.");
    println("-v, --version -> Display program information and exit.");
    println("");
    
    exit;
end

func version
begin
    println("echo v0.1");
    println("");
    println("This is a simple version of the UNIX echo program written in Lila");
    println("Lila and this program were written by Patrick Flynn");
    
    exit;
end

# The main function
func main(args:str[]) -> int
    size : int = sizeof(args);
    i, z : int = 1;
    current : str = "";
    c : char = 0;
begin
    # Check to make sure we have arguments
    if size == 1
        println("");
        return 0;
    end
    
    # Next, check for any command line arguments
    # For right now, we only have "-h, --help" and "-v, --version"
    while i < size
        current = args[i];
        i++;
        
        if current == "-h"
            help();
        elif current == "--help"
            help();
        elif current == "-v"
            version();
        elif current == "--version"
            version();
        end
    end
    
    # If we make it to this point, echo everything out
    i = 1;
    
    while i < size
        current = args[i];
        i++;
        
        print(current);
        z = i + 1;
        if z <= size
            print(" ");
        end
    end
    
    println("");
    
    return 0;
end

