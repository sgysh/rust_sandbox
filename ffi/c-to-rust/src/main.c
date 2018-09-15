#include <stdint.h>
#include <stdio.h>

int32_t add_one(int32_t input);

int main() {
    int input = 2;
    int output = add_one(input);
    printf("%d + 1 = %d\n", input, output);

    return 0;
}
