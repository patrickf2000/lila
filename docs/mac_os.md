## Dash on Mac OS

Dash should work on Mac OS. However, I don't have access to Mac OS and my VM's don't work anymore so I can't do it myself, but getting it to work should be trivial.

The assembly generated is only compatible with the GNU assembler; however, if you have gcc installed, you should have no issues (to check, just type "gcc" or "as" in the terminal. If you don't, you will get a popup asking if you want to install the Xcode development tools). Once you have "as" installed, it should assemble without issues.

To link, you need to check the paths in the internal link command. Mac OS currently still uses x86-64, so you will need to use this compiler layer. Open "compiler/x86/src/lib.rs", and change the paths as needed. Basically, you need these files: "crti.o", "crtn.o", and "crt1.o". Mac OS is architecturally similar to Linux so they should be somewhere under "/usr/lib". It could be under "/usr/local/lib" or "/usr/lib/gcc" or something like that.

The "/lib64/ld-linux-x86-64.so.2" is the dynamic linker. I have no idea where this will be on Mac; to find out, build a hello world C program and run "ldd" on it. It should start with "ld-...".

