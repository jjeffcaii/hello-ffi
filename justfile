default:
        @cargo build
        @gcc main.c -L ./target/debug/ -lhello_ffi -o main
        @./main

