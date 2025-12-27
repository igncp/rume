#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum RumeKeyEventResultC {
    RumeKERHandled,
    RumeKERNotHandled,
    RumeKERError,
} RumeKeyEventResultC;

typedef struct RumeC {
    void *inner;
} RumeC;

typedef struct RumeNewConfigC {
    const char *app_name;
    const char *log_dir;
    bool stdout_log;
} RumeNewConfigC;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct RumeC *rume_new(const struct RumeNewConfigC *config);

void rume_free(struct RumeC *instance);

int32_t rume_init(struct RumeC *instance);

enum RumeKeyEventResultC rume_process_key(struct RumeC *instance,
        uint16_t key_code,
        uint32_t modifier_flag);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
