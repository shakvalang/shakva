#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

//
// typedef enum { BINDING, ASSIGN, EXPR, FOR, WHILE, DO_WHILE } stmt_kind_t;
//
// typedef struct {
//     stmt_kind_t kind;
//     union {
//         struct {
//             tok_t mod;
//             tok_t lhs;
//             struct expr_t *rhs;
//         } binding;
//         struct {
//             struct expr_t *lhs;
//             tok_t op;
//             struct expr_t *rhs;
//         } assign;
//         struct expr_t *expr;
//     };
// } stmt_t;
//
// typedef struct {
//     int stmt_cnt;
//     stmt_t *stmts;
// } shk_line_t;

int main(int argc, char **argv) { printf("Hello, world!\n"); }
