#!/bin/bash

test_count=0

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ls`
        
        if [[ $3 == "error" ]] ; then
            if [ -f ./ERROR_TEST.sh ] ; then
                rm ERROR_TEST.sh
            fi
            
            echo "#!/bin/bash" >> ERROR_TEST.sh
            echo "cargo run $entry --use-c" >> ERROR_TEST.sh
            chmod 777 ERROR_TEST.sh
            ./test.py $entry  ./ERROR_TEST.sh "error"
            
            if [[ $? != 0 ]] ; then
                rm ERROR_TEST.sh
                exit 1
            fi
            
            rm ERROR_TEST.sh
        else
            if [[ $2 == "sys" ]] ; then
                cargo run $entry $3 -o $name
            elif [[ $2 == "sys2" ]] ; then
                cargo run $entry $3 -o $name --no-start
            elif [[ $2 == "clib" ]] ; then
                cargo run $entry --use-c $3 -o $name
            fi
        
    	    ./test.py $entry ./$name ""
    	    
    	    if [[ $? != 0 ]] ; then
        		exit 1
        	fi
        	
        	rm ./$name
        	rm /tmp/$name.o
        	rm /tmp/$name.asm
    	fi
    	
    	test_count=$((test_count+1))
    done
}

flags=""

if [[ $1 == "--risc" ]] ; then
    flags=" --risc "
fi

echo "Running all tests..."
echo ""

run_test 'test/basic/*.ls' 'sys' $flags
run_test 'test/int/*.ls' 'clib' $flags
run_test 'test/int64/*.ls' 'clib' $flags
run_test 'test/byte/*.ls' 'clib' $flags
run_test 'test/short/*.ls' 'clib' $flags
run_test 'test/float/*.ls' 'clib' $flags
run_test 'test/char/*.ls' 'clib' $flags
run_test 'test/string/*.ls' 'clib' $flags

run_test 'test/assign/*.ls' 'sys' $flags
run_test 'test/ooop/*.ls' 'clib' $flags
run_test 'test/loop/*.ls' 'clib' $flags
run_test 'test/mem/*.ls' 'sys' $flags
run_test 'test/const/*.ls' 'clib' $flags
run_test 'test/func/*.ls' 'clib' $flags
run_test 'test/enum/*.ls' 'clib' $flags

run_test 'test/errors/*.ls' 'clib' "error"
run_test 'test/errors/ltac/*.ls' "clib" "error"

run_test 'test/vector/*.ls' 'clib'
run_test 'test/syscall/x86-64/*.ls' 'sys2'

echo ""
echo "$test_count tests passed successfully."
echo "Done"

