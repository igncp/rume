#include <stdio.h>

// Include the generated header file
#include "rume_api.h"

int main() {
    Rume* rume_instance = rume_new();
    if (rume_instance == NULL) {
        fprintf(stderr, "Failed to create Rume instance\n");
        return 1;
    }
    int init_result = rume_instance->init(rume_instance);
    if (init_result != 0) {
        fprintf(stderr, "Failed to initialize Rume instance\n");
        free(rume_instance); // Free the allocated memory before exiting
        return 1;
    }
    rume_free(rume_instance);

    fprintf(stdout, "Rume instance created and initialized successfully, exiting\n");

    return 0;
}