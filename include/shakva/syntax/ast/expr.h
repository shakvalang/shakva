#pragma once

#include "shakva/source/pos.h"
#include "shakva/syntax/token.h"

typedef enum {
    BINARY_EXPR,
    PAREN_EXPR,
    LIT_EXPR,
} expr_kind_t;

struct expr_t;

typedef struct {
    struct expr_t *lhs;
    tok_t op;
    struct expr_t *rhs;
} binary_expr_t;

typedef struct {
    tok_t lparen;
    struct expr_t *inner;
    tok_t rparen;
} paren_expr_t;

typedef struct {
    expr_kind_t kind;
    bpos_t bpos;
    union {
        binary_expr_t binary;
        paren_expr_t paren;
    };
} expr_t;
