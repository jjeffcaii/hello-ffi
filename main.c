#include <stdio.h>

// extern int32_t double_input(int32_t input);
// extern intptr_t add(int32_t a, int32_t b);
// extern int32_t block_get_i32(uintptr_t future);

extern uintptr_t add_future(int32_t a, int32_t b);
extern uintptr_t block_get_add(uintptr_t future);

int main(int argc, char const *argv[])
{
    uintptr_t future = add_future(90, 10);
    int32_t output = block_get_add(future);
    printf("C call Rust: %d\n", output);
    return 0;
}
