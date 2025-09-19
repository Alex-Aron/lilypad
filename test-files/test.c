#include <stdint.h>
#include <stdio.h>

typedef struct {
    uint8_t id;
    uint8_t* data;
    char valid;
} struct_example;

int main(){
    printf("Where is waldo");
    struct_example instance;
    return 0;
}