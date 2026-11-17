#pragma once

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

    // A character the lexer does not know.
    UNKNOWN,
    // End of input.
    EOI,
} shk_token_kind_t;

typedef enum {
    INT,
    FLOAT,
} shk_literal_kind_t;

typedef enum {
    BIN = 2,
    OCT = 8,
    DEC = 10,
    HEX = 16,
} shk_number_system_t;

typedef struct {
    shk_literal_kind_t kind;
    shk_number_system_t num_system;
} shk_literal_data_t;

typedef struct {
    shk_token_kind_t kind;
    shk_literal_data_t *lit_data;
    char *value;
} shk_token_t;

typedef enum {
    BINARY_EXPR,
    PAREN_EXPR,
    LIT_EXPR,
} expr_kind_t;

struct expr_t;

typedef struct {
    struct expr_t *lhs;
    shk_token_t op;
    struct expr_t *rhs;
} binary_expr_t;

typedef struct {
    shk_token_t lparen;
    struct expr_t *inner;
    shk_token_t rparen;
} paren_expr_t;

typedef struct {
    expr_kind_t kind;
    union {
        binary_expr_t binary;
        paren_expr_t paren;
    };
} expr_t;

typedef enum { EXPR } shk_statement_kind_t;

typedef struct {
    shk_statement_kind_t kind;
    struct expr_t *expr;
} shk_statement_t;

typedef struct {
    int stmt_cnt;
    shk_statement_t *stmts;
} shk_line_t;
