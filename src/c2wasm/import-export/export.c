#include <string.h>
#include <stdlib.h>
#define EXPORT __attribute__((visibility("default")))
#define EXPORT_WITH_NEW_NAME(name) __attribute__((export_name(name)))

EXPORT 
int add2Numbers(int a, int b) {
    return a + b;
}

EXPORT 
_Bool getReverseBool(_Bool param) {
    return !param;
}

typedef struct {
    char *offset;
    int length;
} Simple_Str;

EXPORT_WITH_NEW_NAME("getStringWithPointerParam") 
void getString(Simple_Str *ssPtr) {
    (*ssPtr).offset = "Hello, I am from wasm module compiled from c";
    (*ssPtr).length = strlen((*ssPtr).offset);
}

EXPORT
int getSimpleStrSize () {
    return sizeof(Simple_Str);
}

EXPORT
void* mallocInModule(int size) {
    return malloc(size);
}

EXPORT
void freeInModule(void *offset) {
    free(offset);
}