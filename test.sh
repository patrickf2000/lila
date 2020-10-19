#!/bin/bash

function run_test() {
    for entry in $1
    do
    	name=`basename $entry .ds`
    	arch="--amd64"
    	
    	if [[ $3 == "aarch64" ]] ; then
    	    arch="--aarch64"
    	fi
        
        if [[ $4 == "error" ]] ; then
            if [ -f ./ERROR_TEST.sh ] ; then
                rm ERROR_TEST.sh
            fi
            
            echo "#!/bin/bash" >> ERROR_TEST.sh
            echo "cargo run $entry --use-c $arch" >> ERROR_TEST.sh
            chmod 777 ERROR_TEST.sh
            ./test.py $entry  ./ERROR_TEST.sh "error"
            
            if [[ $? != 0 ]] ; then
                rm ERROR_TEST.sh
                exit 1
            fi
            
            rm ERROR_TEST.sh
        else
            if [[ $2 == "sys" ]] ; then
                cargo run $entry $arch
            elif [[ $2 == "clib" ]] ; then
                cargo run $entry --use-c $arch
            fi
        
    	    ./test.py $entry ./$name ""
    	    
    	    if [[ $? != 0 ]] ; then
        		exit 1
        	fi
        	
        	rm ./$name
        	rm /tmp/$name.o
        	rm /tmp/$name.asm
    	fi
    done
}

if [[ $1 != "x86-64" && $1 != "aarch64" ]] ; then
    echo "Invalid architecture: $1"
    echo "Please choose either \"x86-64\" or \"aarch64\""
    exit 1
fi

echo "Running all tests..."
echo ""

run_test 'test/math/*.ds' 'clib' $1
run_test 'test/cond/*.ds' 'clib' $1
run_test 'test/func/*.ds' 'clib' $1
run_test 'test/loop/*.ds' 'clib' $1
run_test 'test/array/*.ds' 'clib' $1
run_test 'test/byte/*.ds' 'clib' $1
run_test 'test/string/*.ds' 'clib' $1
run_test 'test/errors/*.ds' 'clib' $1 "error"
run_test 'test/errors/ltac/*.ds' "clib" $1 "error"

if [[ $1 == "x86-64" ]] ; then
    run_test 'test/short/*.ds' 'clib' $1
    run_test 'test/float/*.ds' 'clib' $1
    run_test 'test/vector/*.ds' 'clib' $1
    run_test 'test/syscall/x86-64/*.ds' 'sys' $1
elif [[ $1 == "aarch64" ]] ; then
    run_test 'test/syscall/aarch64/*.ds' 'sys' $1
fi

echo "Done"

