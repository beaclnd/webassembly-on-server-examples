#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define Export __attribute__((visibility("default")))

// For a bug of wasmer: https://github.com/wasmerio/wasmer/discussions/2618 .
// And llvm doesn't support both import and export memory.
// A work around: set a exported global variable as place holder, then modify it to export the memory in the wat format.
// Utilize the transformation folw: c -> wasm -> wat -> modified wat -> wasm
Export 
int placeHolderForMemoryExport;

void printThis(char *message, int length);

#define MAX_STRING_LENGTH_ACCEPTED 1000
void getString(void *param_ptr, int param_length, void *result_ptr, int *result_length);

double getPi();

double getR();


Export
void doSomethings(int param1, char* param2) {
    double pi = getPi();
    double r = getR();
    double area = pi * pow(r, 2);
    char *str = malloc(MAX_STRING_LENGTH_ACCEPTED);
    int *str_length = malloc(sizeof(int));
    if (area < 10.5) {
        char *param_ptr = "string1";
        getString(param_ptr, strlen(param_ptr), str, str_length);
    }
    else if (area >= 10.5 && area < 20.5 ) {
        char *param_ptr = "string2";
        getString(param_ptr, strlen(param_ptr), str, str_length);
    } else {
        getString(NULL, 0, NULL, NULL);
    } 
    char buffer[500];
    sprintf(
        buffer, 
        "Hi! I am a function from wasm module, \n"
        "my caller give me param1(integer: %d) and param2(string: %s)"
        "through the outside environment, \n"
        "I have got the pi(%f) by getPie function, \n"
        "the r(%f) by the getR function, \n"
        "then calculated the area which is %f \n"
        "I also got the string(%s)(whose length is %d) by the getString function.\n"
        "And this message is printed by calling the outside environment's printThis function.", 
        param1,
        param2,
        pi,
        r,
        area,
        str,
        *str_length
    );
    printThis(buffer, strlen(buffer));
    free(str);
    free(str_length);
}

Export
void *mallocInModule(int size) {
    return malloc(size);
}

Export
void freeInModule(void *offset) {
    free(offset);
}