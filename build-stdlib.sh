#!/bin/bash

echo "Building standard library..."
echo ""

cargo build --release

cwd=`pwd`
export lilac="$cwd/target/release/lilac"

cd target

if [[ -d ./std ]] ; then
    rm -r ./std
fi

if [[ -f liblila.so ]] ; then
    rm liblila.so
fi

# Order matters
$lilac ../stdlib/x86_64.ls -o x86_64.o --no-link --pic
$lilac ../stdlib/math.ls -o math.o --no-link --pic
$lilac ../stdlib/string.ls -o string.o --no-link --pic
$lilac ../stdlib/io.ls -o io.o --no-link --pic
$lilac ../stdlib/unix.ls -o unix.o --no-link --pic
$lilac ../stdlib/fs.ls -o fs.o --no-link --pic

$lilac -o liblila.so --lib \
    x86_64.o \
    io.o \
    math.o \
    string.o \
    unix.o \
    fs.o
    
rm *.o

cd ..

echo "Done"

