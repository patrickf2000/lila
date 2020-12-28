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
# Build the standard library
$lilac ../stdlib/x86_64.ls -o x86_64.o --no-link --pic
$lilac ../stdlib/riscv64.ls -o riscv64.o --no-link --pic
$lilac ../stdlib/string.ls -o string.o --no-link --pic
$lilac ../stdlib/io.ls -o io.o --no-link --pic
$lilac ../stdlib/fs.ls -o fs.o --no-link --pic
$lilac ../stdlib/text_io.ls -o text_io.o --no-link --pic

$lilac -o liblila.so --lib \
    x86_64.o \
    riscv64.o \
    string.o \
    io.o \
    fs.o \
    text_io.o
    
rm *.o

if [[ $1 == "x86_64" ]] ; then
    as ../stdlib/x64_start.asm -o lrt.o
elif [[ $1 == "riscv64" ]] ; then
    as ../stdlib/riscv64_start.asm -o lrt.o
else
    echo "Invalid architecture."
    echo "Expected x86_64 or riscv64"
    exit 1
fi

cd ..

echo "Done"

