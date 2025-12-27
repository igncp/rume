#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

#include "rume_api.h"

int main() {
    char* log_dir = getenv("RUME_LOG_DIR");
    struct RumeNewConfigC config = {
        .app_name = "test_rume",
        .log_dir = log_dir,
        .stdout_log = true
    };
    RumeC* rume_instance = rume_new(&config);

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

    RumeSessionIdC session_id = rume_create_session(rume_instance);

    rume_process_key(rume_instance, session_id, 11, 1<<3);
    rume_process_key(rume_instance, session_id, 12, 1<<3);

    rume_delete_session(rume_instance, session_id);
    rume_free(rume_instance);

    return rv;
}