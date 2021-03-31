#!/bin/bash

echo "Building standard library..."
echo ""

cargo build --release

cwd=`pwd`
export idac="$cwd/target/release/idac"

cd target

if [[ -d ./std ]] ; then
    rm -r ./std
fi

if [[ -f libida.so ]] ; then
    rm libida.so
fi

# Order matters
# Build the core library
$idac ../corelib/x86_64.ida -o x86_64.o --no-link --pic --no-corelib
$idac ../corelib/mem.ida -o mem.o --no-link --pic --no-corelib
$idac ../corelib/string.ida -o string.o --no-link --pic --no-corelib
$idac ../corelib/io.ida -o io.o --no-link --pic --no-corelib

ar -rc libida_core.a \
    x86_64.o \
    mem.o \
    string.o \
    io.o
    
rm *.o

# Build the standard library
$idac ../stdlib/string.ida -o string.o --no-link --pic
$idac ../stdlib/io.ida -o io.o --no-link --pic
$idac ../stdlib/os.ida -o os.o --no-link --pic
$idac ../stdlib/file_io.ida -o file_io.o --no-link --pic
$idac ../stdlib/text_utils.ida -o text_utils.o --no-link --pic
$idac ../stdlib/text_io.ida -o text_io.o --no-link --pic

$idac -o libida.so --lib \
    string.o \
    io.o \
    os.o \
    file_io.o \
    text_utils.o \
    text_io.o
    
rm *.o

as ../stdlib/x64_start.asm -o irt.o

cd ..

echo "Done"

