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

echo "Running all tests..."
echo ""

run_test 'test/cond/*.qk' 'sys'
run_test 'test/math/*.qk' 'clib'
run_test 'test/syscall/*.qk' 'sys'

echo "Done"
