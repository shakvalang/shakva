#pragma once

#include "shakva/source/pos.h"

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
} tok_kind_t;

typedef enum {
    INT,
    FLOAT,
} lit_kind_t;

typedef struct {
    lit_kind_t kind;
} lit_data_t;

typedef struct {
    tok_kind_t kind;
    bpos_t bpos;
    union {
        char *value;
        lit_data_t lit_data;
    };
} tok_t;
