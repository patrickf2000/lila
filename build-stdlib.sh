#!/bin/bash

echo "Building standard library..."
echo ""

cargo build --release

cwd=`pwd`
export PATH="$cwd/target/release/dashc:$PATH"

cd target

if [[ -d ./std ]] ; then
    rm -r ./std
fi

if [[ -f libdash.so ]] ; then
    rm libdash.so
fi

# Order matters
dashc ../stdlib/x86_64.ds -o x86_64.o --no-link --pic
dashc ../stdlib/math.ds -o math.o --no-link --pic
dashc ../stdlib/string.ds -o string.o --no-link --pic
dashc ../stdlib/io.ds -o io.o --no-link --pic

dashc -o libdash.so --lib \
    x86_64.o \
    io.o \
    math.o \
    string.o
    
rm *.o

cd ..

echo "Done"

