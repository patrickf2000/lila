#!/bin/bash

export LD_LIBRARY_PATH=./

test_count=0

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ds`
    	arch="--amd64"
    	
    	if [[ $2 == "aarch64" ]] ; then
    	    arch="--aarch64"
    	fi
        
        dashc $entry -o $name -ldash
    
	    ./test.py $entry ./$name ""
	    
	    if [[ $? != 0 ]] ; then
    		exit 1
    	fi
    	
    	rm ./$name
    	rm /tmp/$name.o
    	rm /tmp/$name.asm
    	
    	test_count=$((test_count+1))
    done
}

if [[ $1 != "x86-64" && $1 != "aarch64" ]] ; then
    echo "Invalid architecture: $1"
    echo "Please choose either \"x86-64\" or \"aarch64\""
    exit 1
fi

echo "Running all tests..."
echo ""

run_test 'test/io/*.ds' $1

echo ""
echo "$test_count tests passed successfully."
echo "Done"

