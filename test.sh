#!/bin/bash

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .qk`
    	
    	if [[ $2 == "sys" ]] ; then
            cargo run $entry
        elif [[ $2 == "clib" ]] ; then
            cargo run $entry --use-c
        fi
        
    	./test.py $entry ./$name
    	
    	if [[ $? != 0 ]] ; then
    		exit 1
    	fi
    	
    	rm ./$name
    done
}

if [[ $1 != "x86-64" && $1 != "aarch64" ]] ; then
    echo "Invalid architecture: $1"
    echo "Please choose either \"x86-64\" or \"aarch64\""
    exit 1
fi

echo "Running all tests..."
echo ""

run_test 'test/math/*.qk' 'clib'

if [[ $1 == "x86-64" ]] ; then
    run_test 'test/cond/*.qk' 'sys'
    run_test 'test/syscall/*.qk' 'sys'
elif [[ $1 == "aarch64" ]] ; then
    run_test 'test/syscall/aarch64/*.qk' 'sys'
fi

echo "Done"
