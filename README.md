# Nandu

Nandu is a CLI tool which takes a boolean expression
as input and returns an equivalent boolean expression
which is only made up of `Nand` function.

Example:

```shell
$ nandu "And(a, b)"
Nand(Nand(a, b), Nand(a, b))
```

```shell
$ nandu "Xor(a, b)"
Nand(Nand(Nand(Nand(a, Nand(b, b)), Nand(a, Nand(b, b))), Nand(Nand(a, Nand(b, b)), Nand(a, Nand(b, b)))), Nand(Nand(Nand(Nand(a, a), b), Nand(Nand(a, a), b)), Nand(Nand(Nand(a, a), b), Nand(Nand(a, a), b))))
```

Why you may ask?! For no reason, it's just fun and I hoped to learn something while building this.

Nandu knowns a few basic functions which it convers by default. These include:

  - And

  - Or

  - Not

  - Xor

Others might follow.

Also, it might be possible at some point to add your own translations for custom functions.

Obviously the implementations of the different gates from Nands are not the only
possible options. It will be possible to overwrite them with custom implementations
once it is possible to add any custom function at all.

## Grammar

The input is parsed into an AST using recursive decent.
The following grammar defines valid inputs, which the
parser should accept. Note that at least one argument is
required in each function.

```ebnf
<F>       ::= FuncIdent LParen <ArgList> RParen
<ArgList> ::= <Arg> (Delim <Arg>)*
<Arg>     ::= VarIdent | <F>
```

The following substitution steps show that this grammar
does not feature left recursion (I think).

```ebnf
/* Substituting <ArgList> in <F> */
<F>       ::= FuncIdent LParen <Arg> (Delim <Arg>)* RParen
<Arg>     ::= VarIdent | <F>
```

```ebnf
/* Substituting <Arg> in <F> */
<F> ::= FuncIdent LParen VarIdent (Delim VarIdent)* RParen
      | FuncIdent LParen VarIdent (Delim <F>)*      RParen
      | FuncIdent LParen <F>      (Delim VarIdent)* RParen
      | FuncIDent LParen <F>      (Delim <F>)*      RParen
```

None of the above alternative production rules for `<F>`
contains direct left recursion. Hence, the grammar is 
not left recursive.

# To Do

## Functions with multiple outputs (e.g. DMux)

Their return values could be turned into multiple different
arguments for the outer function.

Example:

```plain
And(Dmux(a, b)) <- And expects TWO arguents. Since DMux has two outputs,
                   they are used as both arguments to the And function.

Or4Way(a, DMux(x, y), b)
```

## Functions without arguments ⇒ constants

Since any function must have at least one argument,
constants could provide values which do not require
any more arguments.