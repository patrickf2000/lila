#!/bin/bash

test_count=0

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ds`
        
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
                cargo run $entry -o $name
            elif [[ $2 == "clib" ]] ; then
                cargo run $entry --use-c -o $name
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

echo "Running all tests..."
echo ""

run_test 'test/int/*.ds' 'clib'
run_test 'test/int64/*.ds' 'clib'
run_test 'test/byte/*.ds' 'clib'
run_test 'test/short/*.ds' 'clib'
run_test 'test/float/*.ds' 'clib'
run_test 'test/char/*.ds' 'clib'
run_test 'test/string/*.ds' 'clib'

run_test 'test/ooop/*.ds' 'clib'
run_test 'test/loop/*.ds' 'clib'
run_test 'test/ldarg/*.ds' 'clib'
run_test 'test/const/*.ds' 'clib'
run_test 'test/func/*.ds' 'clib'

run_test 'test/errors/*.ds' 'clib' "error"
run_test 'test/errors/ltac/*.ds' "clib" "error"

run_test 'test/vector/*.ds' 'clib'
run_test 'test/syscall/x86-64/*.ds' 'sys'

echo ""
echo "$test_count tests passed successfully."
echo "Done"

