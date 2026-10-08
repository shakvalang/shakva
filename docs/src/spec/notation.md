# Notation

The syntax is specified using a [variant](https://en.wikipedia.org/wiki/Wirth_syntax_notation) of Extended Backus-Naur Form (EBNF):

```ebnf
syntax      = { production } .
production  = ProductionName "=" [ expression ] "." .
expression  = term { "|" term } .
term        = factor { factor } .
factor      = ProductionName 
            | Token [ "…" Token ] 
            | group 
            | option 
            | repetition .

group       = "(" expression ")" .
option      = "[" expression "]" .
repetition  = "{" expression "}" .
```

Productions are expressions constructed from terms and the following operators, in increasing precedence:

```text
|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)
```

CamelCase production names are used to identify lexical (terminal) tokens. Non-terminals are in snake_case. Lexical tokens are enclosed in double quotes "" or back quotes ``.

The form `a … b` represents the set of characters from a through b as alternatives. The horizontal ellipsis `…` is also used elsewhere in the spec to informally denote various enumerations or code snippets that are not further specified. The character `…` is not a token of the Shakva language.
