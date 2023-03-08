# Grammar documentation for `church-lamb`

This document explains the grammar of the variant of Church encoding used in
this program, `church-lamb`. Traditionally, Church encoding uses mathematical
symbols that are not very convenient to type in regular computer keyboards.
Thus, this program makes some adjustments to the syntax to make the language
easier for users to type and for computers to parse.

## Basic building blocks

These are the basic building blocks of the Church encoding. As such, they
represent either fundamental concepts of Lambda Calculus or grammatical
utility classes that make up these concepts.

### Expressions

> **Syntax** \
> `<expression> ::= <identifier> | <function definition> | '(' <function application> ')'`

An _expression_ is the smallest standalone "thing" that has a value. For more
information, see the corresponding sections below.

### Identifier

An _identifier_ is a string of alphanumeric (including underbar) characters. It
may serve the role of an _atom_ or a _variable_. If the identifier is _bound_;
i.e. 


### Function definitions

> **Syntax** \
> `<function definition> ::= '\' <identifier> '.' <expression>`

A _function definition_ create a new function. While traditionally a function is
typeset with a Greek letter lambda(`λ`), here it is replaced with a backslash
character(`\`) as it is much more convenient to type on regular keyboard while
having similar appearance.

Note that this expression is right-associative. Before pairing with the argument
identifier, the body of the function will be


#### Example

```
# Identitiy function
\x.x

# Self-application function
\s.(s s)

# Function application function
\f.\x.(f x)
```

### Function applications

> **Syntax** \
> `<function application> ::= <expression> <expression>`

A _function application_ expression takes the first expression as the function,
and the second expression as the argument. And the function expression is then
evaluated with the argument expression.

### Comments

> **Syntax** \
> `<comment> ::= '#' <any string until a newline>`

Comments are ignored by the evaluator, as they are not part of the expression
and exist as helpful notes or reminder for the programmer. This syntax allows
Church encoding be evaluated as a command-line program with the use of
_shebang_ line in POSIX operating systems.

#### Example

```
# This is an example of church-lamb comment. These lines will be ignored when
# evaluating the expression.
```
