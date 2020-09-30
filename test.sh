#!/bin/bash

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .qk`
    	arch="--amd64"
    	
    	if [[ $3 == "aarch64" ]] ; then
    	    arch="--arm64"
    	fi
    	
    	if [[ $2 == "sys" ]] ; then
            cargo run $entry $arch
        elif [[ $2 == "clib" ]] ; then
            cargo run $entry --use-c $arch
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

run_test 'test/math/*.qk' 'clib' $1
run_test 'test/cond/*.qk' 'clib' $1
run_test 'test/func/*.qk' 'clib' $1

if [[ $1 == "x86-64" ]] ; then
    run_test 'test/syscall/*.qk' 'sys' $1
elif [[ $1 == "aarch64" ]] ; then
    run_test 'test/syscall/aarch64/*.qk' 'sys' $1
fi

echo "Done"
