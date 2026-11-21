#include "shakva/driver.h"
#include "sds.h"
#include "stb_ds.h"
#include <stdio.h>

typedef struct {
} shk_driver_impl_t;

shk_driver_t shk_driver_new(void) {
    shk_driver_t d = malloc(sizeof(shk_driver_impl_t));
    return d;
}

void shk_driver_free(shk_driver_t d) {
    free(d);
    d = NULL;
}

static sds readline(FILE *fp) {
    char buf[1024];
    sds line = sdsempty();
    if (!line) {
        return NULL;
    }

    while (fgets(buf, sizeof(buf), fp) != NULL) {
        size_t len = strlen(buf);
        line = sdscatlen(line, buf, len);
        if (!line) {
            return NULL;
        }

        // Stop if we reached the end of the line
        if (len > 0 && buf[len - 1] == '\n') {
            break;
        }
    }

    size_t total_len = sdslen(line);
    // Handle immediate EOF with no data read
    if (total_len == 0 && feof(fp)) {
        sdsfree(line);
        return NULL;
    }

    while (total_len > 0 && (line[total_len - 1] == '\r' || line[total_len - 1] == '\n')) {
        --total_len;
    }

    sdssetlen(line, total_len);
    // maintain null termination
    line[total_len] = '\0';

    return line;
}

static int shk_driver_repl(void) {
    while (1) {
        fputs("🌊 >>> ", stdout);
        fflush(stdout);

        sds input = readline(stdin);
        if (input == NULL) {
            break;
        }
        fprintf(stdout, "%s\n", input);
    }
    return 0;
}

int shk_driver_run(shk_driver_t d, int argc, char **argv) {
    return shk_driver_repl();
}
