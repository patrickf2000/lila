#!/bin/bash

echo "Building standard library..."
echo ""

cargo build --release

cwd=`pwd`
export dashc="$cwd/target/release/dashc"

cd target

if [[ -d ./std ]] ; then
    rm -r ./std
fi

if [[ -f libdash.so ]] ; then
    rm libdash.so
fi

# Order matters
cargo run --release ../stdlib/x86_64.ds -o x86_64.o --no-link --pic
cargo run --release ../stdlib/math.ds -o math.o --no-link --pic
cargo run --release ../stdlib/string.ds -o string.o --no-link --pic
cargo run --release ../stdlib/io.ds -o io.o --no-link --pic
cargo run --release ../stdlib/unix.ds -o unix.o --no-link --pic

$dashc -o libdash.so --lib \
    x86_64.o \
    io.o \
    math.o \
    string.o \
    unix.o
    
rm *.o

cd ..

echo "Done"

