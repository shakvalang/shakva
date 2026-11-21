#pragma once

typedef struct {
    char *start, end;
} shk_pos_t;

typedef enum {
    /* trivia */

    // A sequence of whitespace characters excluding CR (`\r`) and LF (`\n`).
    WS,
    // A comment starting with `#`, excluding the terminating line ending.
    COMMENT,

    /* punct */

    // `,`.
    COMMA,
    // `:`.
    COLON,
    // `.`.
    DOT,
    // `;`.
    SEMI,
    // A single line ending: LF (`\n`) or CRLF (`\r\n`).
    NL,
    // `{`.
    LBRACE,
    // `}`.
    RBRACE,
    // `[`.
    LBRACKET,
    // `]`.
    RBRACKET,
    // `(`
    LPAREN,
    // `)`
    RPAREN,
    // `?`.
    QUEST,
    // `_`.
    WILDCARD,
    // `->`.
    ARROW,
    // `=>`.
    FARROW,

    /* operators */

    // `!`.
    EXCL,
    // `+`.
    PLUS,
    // `-`.
    MINUS,
    // `*`.
    STAR,
    // `/`.
    SLASH,
    // `%`.
    PERCENT,
    // `**`.
    DSTAR,
    // `//`.
    DSLASH,
    // `>`.
    GT,
    // `<`.
    LT,
    // `==`.
    EQ,
    // `>=`.
    GE,
    // `<=`.
    LE,

    /* binding operators */

    // `:=`.
    BIND,

    /* assignment operators */

    // `=`.
    REBIND,
    // `+=`.
    PLUS_EQ,
    // `-=`.
    MINUS_EQ,
    // `*=`.
    STAR_EQ,
    // `/=`.
    SLASH_EQ,
    // `%=`.
    PERCENT_EQ,
    // `**=`.
    DSTAR_EQ,
    // `//=`.
    DSLASH_EQ,

    LIT,
    IDENT,

    // `match`.
    MATCH_KW,
    // `if`.
    IF_KW,
    // `elif`.
    ELIF_KW,
    // `else`.
    ELSE_KW,
    // `mut`
    MUT_KW,
    // `and`.
    AND_KW,
    // `or`.
    OR_KW,
    // `for`.
    FOR_KW,
    // `in`.
    IN_KW,
    // `while`.
    WHILE_KW,
    // `do`.
    DO_KW,
    // `not`.
    NOT_KW,

    // A character the lexer does not know.
    UNKNOWN,
    // End of input.
    EOI,
} shk_tok_kind_t;

typedef enum {
    INT,
    FLOAT,
} shk_lit_kind_t;

typedef enum {
    BIN = 2,
    OCT = 8,
    DEC = 10,
    HEX = 16,
} shk_num_sys_t;

typedef struct {
    shk_lit_kind_t kind;
    shk_num_sys_t num_sys;
} shk_lit_data_t;

typedef struct {
    shk_tok_kind_t kind;
    shk_lit_data_t *lit_data;
    char *v;
} shk_tok_t;

typedef enum {
    BINARY_EXPR,
    PAREN_EXPR,
    LIT_EXPR,
} shk_expr_kind_t;

struct shk_expr_t;

typedef struct {
    struct shk_expr_t *lhs;
    shk_tok_t op;
    struct shk_expr_t *rhs;
} shk_bin_expr_t;

typedef struct {
    shk_tok_t lparen;
    struct shk_expr_t *inner;
    shk_tok_t rparen;
} shk_paren_expr_t;

typedef struct {
    shk_expr_kind_t kind;
    union {
        shk_bin_expr_t *binary;
        shk_paren_expr_t *paren;
    };
} shk_expr_t;

typedef enum { EXPR } shk_stmt_kind_t;

typedef struct {
    shk_stmt_kind_t kind;
    shk_expr_t *expr;
} shk_stmt_t;

typedef struct {
    shk_stmt_t *stmts;
} shk_file_t;
