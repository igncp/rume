#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum RumeKeyEventResultC {
    RumeKERHandled,
    RumeKEREnabled,
    RumeKERDisabled,
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

typedef uint32_t RumeSessionIdC;

typedef struct RumeMenuC {
    uint32_t num_candidates;
} RumeMenuC;

typedef struct RumeContextC {
    struct RumeMenuC menu;
    const char *preedit_text;
    const char *committed_text;
} RumeContextC;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct RumeC *rume_new(const struct RumeNewConfigC *config);

void rume_free(struct RumeC *instance);

int32_t rume_init(struct RumeC *instance);

RumeSessionIdC rume_create_session(struct RumeC *instance);

void rume_delete_session(struct RumeC *instance, RumeSessionIdC session_id);

enum RumeKeyEventResultC rume_process_key(struct RumeC *instance,
        RumeSessionIdC session_id,
        uint16_t key_code,
        uint32_t modifier_flag);

const struct RumeContextC *rume_get_context(struct RumeC *instance, RumeSessionIdC session_id);

void rume_free_context(const struct RumeContextC *context);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
