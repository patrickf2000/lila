#!/bin/bash

cwd=`pwd`
export PATH="$cwd/target/release:$PATH"
export LD_LIBRARY_PATH="$cwd/target:$LD_LIBRARY_PATH"

which idac

test_count=0

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ida`
        
         idac $entry -o $name -lida
    
	    ../test.py $entry ./$name ""
	    
	    if [[ $? != 0 ]] ; then
    		exit 1
    	fi
    	
    	rm ./$name
    	rm /tmp/$name.o
    	rm /tmp/$name.asm
    	
    	test_count=$((test_count+1))
    done
}

echo "Running all standard library tests..."
echo ""

cd target

run_test '../test/stdlib/io/*.ida'
run_test '../test/stdlib/text_io/*.ida'

# Generate test file
if [[ -f ./file.txt ]] ; then
    rm file.txt
fi
echo "Hello, how are you?" >> file.txt
echo "I am good." >> file.txt
echo "Excellent." >> file.txt
echo "" >> file.txt

if [[ -f first.txt ]] ; then
    rm first.txt
fi

run_test '../test/stdlib/fs/*.ida'
run_test '../test/stdlib/io2/*.ida'

cd ..

echo ""
echo "$test_count tests passed successfully."
echo "Done"

