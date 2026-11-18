#pragma once

#include "shakva/ir.h"

typedef struct {
    shk_file_t file;
    char **errors;
} shk_parse_file_res_t;

shk_parse_file_res_t shk_parse_file(const char *input);
