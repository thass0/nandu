# Nandu

Nandu is a CLI tool which takes a boolean expression
as input and returns an equivalent boolean expression
which is only made up of `Nand` function.

For example:

```shell
$ nandu "And(a, b)"
Nand(Nand(a, b), Nand(a, b))
```

or even more fun:

```shell
$ nandu "Xor(a, b)"
Nand(Nand(Nand(Nand(a, Nand(b, b)), Nand(a, Nand(b, b))), Nand(Nand(a, Nand(b, b)), Nand(a, Nand(b, b)))), Nand(Nand(Nand(Nand(a, a), b), Nand(Nand(a, a), b)), Nand(Nand(Nand(a, a), b), Nand(Nand(a, a), b))))
```

If this seems kind of point less to you, that's because it its. It's somewhat educational at best and for-fun xat worst.

Nandu knowns a few basic functions which it is able to convert by default:

  - And
  - Or
  - Not
  - Xor

I might add some more in the future.

Also, it might be possible at some point to add your own translations for custom functions.

Obviously, the implementations of the different gates from Nands are not the only
possible options. It must be possible to overwrite them with custom implementations
once, it is possible to add any custom function at all.

## Grammar

The input is parsed into an AST using recursive descent.
The following grammar defines valid inputs, which the
parser should accept. Note that at least one paramter is
required in each function.

```ebnf
<F>       ::= FuncIdent LParen <ParamList> RParen
<ParamList> ::= <Param> (Delim <Param>)*
<Param>     ::= VarIdent | <F>
```

# To do

## User-definied functions

See above for the general idea. One of the main problems is the question of
where to store the user-defined functions. The obvious approach would be to
put them into source files that are read (just like an interpreter). However,
I'd like for this tool to go more into a CLI direction than an interpreter.
Ideas are welcome. On the other hand, there is no need to re-invent the wheel
I guess.

## Functions with multiple outputs (e.g. DMux)

Their return values could be turned into multiple different
parameter for the outer function.

Example:

```plain
And(Dmux(a, b)) <- And expects TWO arguents. Since DMux has two outputs,
                   they are used as both arguments to the And function.

Or4Way(a, DMux(x, y), b)
```

## Functions with 0 parameter ⇒ constants

Since any function must have at least one parameter,
constants could provide values which do not require
any extra parameters.
