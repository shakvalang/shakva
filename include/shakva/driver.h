#pragma once

#include "sds.h"
typedef enum { REPL } shk_driver_cmd_kind_t;

typedef struct {
    shk_driver_cmd_kind_t cmd;
    sds input;
    sds output;
} shk_driver_parsed_args_t;

int shk_driver_parse_args(shk_driver_parsed_args_t *, int argc, char **argv);

int shk_driver_run(const shk_driver_parsed_args_t *);
