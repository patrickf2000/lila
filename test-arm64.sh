#!/bin/bash

test_count=0

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ida`
        
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
            elif [[ $2 == "clib" ]] ; then
                cargo run $entry --no-corelib $3 -o $name
                mv /tmp/$name.asm /tmp/$name.s
                gcc /tmp/$name.s -o $name
            fi
        
    	    ./test.py $entry ./$name ""
    	    
    	    if [[ $? != 0 ]] ; then
        		exit 1
        	fi
        	
        	rm ./$name
        	rm /tmp/$name.o
        	rm /tmp/$name.asm
        	rm /tmp/$name.s
    	fi
    	
    	test_count=$((test_count+1))
    done
}

flags=""

echo "Running all tests..."
echo ""

run_test 'test/basic/*.ida' 'clib' $flags

#run_test 'test/int/*.ida' 'clib' $flags
#run_test 'test/int64/*.ida' 'clib' $flags
#run_test 'test/byte/*.ida' 'clib' $flags
#run_test 'test/short/*.ida' 'clib' $flags
#run_test 'test/float/*.ida' 'clib' $flags
#run_test 'test/char/*.ida' 'clib' $flags
#run_test 'test/string/*.ida' 'clib' $flags

#run_test 'test/ooop/*.ida' 'clib' $flags
#run_test 'test/loop/*.ida' 'clib' $flags
#run_test 'test/ldarg/*.ida' 'clib' $flags
#run_test 'test/const/*.ida' 'clib' $flags
#run_test 'test/func/*.ida' 'clib' $flags

#run_test 'test/errors/*.ida' 'clib' "error"
#run_test 'test/errors/ltac/*.ida' "clib" "error"

#run_test 'test/vector/*.ida' 'clib'
#run_test 'test/syscall/x86-64/*.ida' 'sys'

echo ""
echo "$test_count tests passed successfully."
echo "Done"

