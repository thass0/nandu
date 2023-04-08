# Nandu

Nandu is a CLI tool which takes a boolean expression
as input and returns an equivalent boolean expression
which is only made up of `Nand` function.

Example:

```shell
$ nandu "and(a, b)"
nand(nand(a, b), nand(a, b))
```

```shell
$ nandu "xor(a, b)"
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