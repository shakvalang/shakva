#include "shakva/parse.h"
#include "shakva/ir.h"
#include <assert.h>
#include <stdint.h>

static shk_tok_t advance_token() {
}

typedef struct {
    uint8_t l, r;
} infix_bp_t;

typedef uint8_t prefix_bp_t;

// returns 0 for not a prefix operator token
static prefix_bp_t prefix_bp(shk_tok_kind_t op) {
    switch (op) {
    case NOT_KW:
        return 6;
    case MINUS:
        return 14;
    default:
        return 0;
    }
}

// returns l == r == 0 for not an infix operator token
static infix_bp_t infix_bp(shk_tok_kind_t op) {
    switch (op) {
    case OR_KW:
        return (infix_bp_t){1, 2};
    case AND_KW:
        return (infix_bp_t){3, 4};
    // {5, 6} is taken by `not`
    case EQ:
    case GT:
    case GE:
    case LT:
    case LE:
        return (infix_bp_t){7, 8};
    case PLUS:
    case MINUS:
        return (infix_bp_t){9, 10};
    case STAR:
    case SLASH:
    case PERCENT:
    case DSLASH:
        return (infix_bp_t){11, 12};
    // {13, 14} is taken by prefix `+` and `-`.
    case DSTAR:
        return (infix_bp_t){16, 15};
    default:
        return (infix_bp_t){0, 0};
    }
}

static shk_expr_t parse_expr_start(void) {
    shk_tok_t st = advance_token();
}

static shk_expr_t parse_expr_bp(uint8_t min_bp) {
    shk_expr_t lhs = parse_expr_start();
}

static shk_expr_t parse_expr(void) {
}
