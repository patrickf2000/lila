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
                cargo run $entry $3 -o $name --llvm
            elif [[ $2 == "sys2" ]] ; then
                cargo run $entry $3 -o $name --no-start --llvm
            elif [[ $2 == "clib" ]] ; then
                cargo run $entry --use-c $3 -o $name --llvm
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

echo "Running all tests..."
echo ""

run_test 'test/basic/*.ls' 'sys' $flags


echo ""
echo "$test_count tests passed successfully."
echo "Done"

