#include <stdio.h>
#include <stdlib.h>

// Include the generated header file
#include "rume_api.h"

int main() {
    char* log_dir = getenv("RUME_LOG_DIR");
    RumeC* rume_instance = rume_new(log_dir);

    if (rume_instance == NULL) {
        fprintf(stderr, "Failed to create Rume instance\n");
        return 1;
    }

    int init_result = rume_init(rume_instance);
    int rv = 0;

    if (init_result != 0) {
        fprintf(stderr, "Failed to initialize Rume instance\n");
        rv = 1;
    } else {
        fprintf(stdout, "Rume instance initialized successfully\n");
    }

    rume_free(rume_instance);

    return rv;
}