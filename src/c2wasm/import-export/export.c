#define EXPORT(name) __attribute__( ( export_name(name) ) )

EXPORT("add_2_numbers") int add_2_numbers(int a, int b) {
    return a + b;
}

EXPORT("get_reverse_bool") _Bool get_reverse_bool(_Bool param) {
    return !param;
}

// EXPORT("get_string") char* get_string(char* str) {
// }