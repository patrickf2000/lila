#!/bin/bash

cwd=`pwd`
export PATH="$cwd/target/release/dashc:$PATH"
export LD_LIBRARY_PATH="$cwd/target:$LD_LIBRARY_PATH"

test_count=0

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ds`
        
        dashc $entry -o $name -ldash
    
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

run_test '../test/stdlib/io/*.ds'

# Generate test file
if [[ -f ./file.txt ]] ; then
    rm file.txt
fi
echo "Hello, how are you?" >> file.txt
echo "I am good." >> file.txt
echo "Excellent." >> file.txt
echo "" >> file.txt

run_test '../test/stdlib/fs/*.ds'

cd ..

echo ""
echo "$test_count tests passed successfully."
echo "Done"

