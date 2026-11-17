#include "shakva/driver.h"
#include "sds.h"
#include "stb_ds.h"
#include <stddef.h>
#include <stdio.h>
#include <string.h>

typedef enum { SHORT, LONG, VALUE } argv_t;

int shk_driver_parse_args(shk_driver_parsed_args_t *res, int argc, char **argv) {
    int idx = 1;
    char **opts = NULL;
    char **positionals = NULL;
    while (idx < argc) {
        char *cur = argv[idx];
        argv_t cur_type = VALUE;

        if (strncmp(cur, "--", 2) == 0) {
            cur += 2;
            cur_type = LONG;
        } else if (cur[0] == '-') {
            ++cur;
            cur_type = SHORT;
        }

        switch (cur_type) {
        case SHORT:
            if (strlen(cur) == 0) {
                fprintf(stderr, "error: stdin input placehoder `%s` is unsupported\n", cur);
                return 1;
            }
            if (strlen(cur) > 1) {
                fprintf(stderr, "error: glued option flags `%s` are unsupported\n", cur - 1);
                return 1;
            }
            fprintf(stderr, "error: unknown option `%s`\n", cur - 1);
            return 1;
        case LONG:
            if (strlen(cur) == 0) {
                fprintf(stderr, "error: escape option `%s` is unsupported\n", cur);
                return 1;
            }
            fprintf(stderr, "error: unknown option `%s`\n", cur - 2);
            return 1;
        case VALUE:
            arrpush(positionals, cur);
            break;
        }

        ++idx;
    }

    while (idx < argc) {
        arrpush(positionals, argv[idx++]);
    }

    if (arrlen(positionals) == 0) {
        res->cmd = REPL;
        return 0;
    }

    if (arrlen(positionals) == 1) {
        char *cmd = positionals[0];
        if (strcmp(cmd, "repl") == 0) {
            res->cmd = REPL;
        } else {
            fprintf(stderr, "error: unknown command `%s`\n", cmd);
            return 1;
        }
    }

    return 0;
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

int shk_driver_run(const shk_driver_parsed_args_t *args) {
    switch (args->cmd) {
    case REPL:
        return shk_driver_repl();
    }
}
