#!/bin/bash

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .qk`
    	
        cargo run $entry
        
    	./test.py $entry ./$name
    	
    	if [[ $? != 0 ]] ; then
    		exit 1
    	fi
    	
    	rm ./$name
    done
}

echo "Running all tests..."
echo ""

run_test 'test/cond/*.qk'
run_test 'test/math/*.qk'
run_test 'test/syscall/*.qk'

echo "Done"
