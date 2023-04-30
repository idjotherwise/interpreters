# Parsing expressions
When parsing an expression, one important thing to consider is operator precedence. E.g, for an expression like
```go
5 + 5 * 2
```
you want the multiplication to be "deeper" in the AST and evaluated before the addition, like so:
```go
(5 + (5 * 2))
```
So it might look something like:
```go
- PLUS
	- 5
	- MULTIPLY
		- 5
		- 2
```

A relevant consequence of this is that we also need to consider when tokens mean something different depending on their context. E.g,
```go
5 * ( add(2, 5) + 3)
```
where the outer set of parentheses create a new scope, while the inner parentheses are a function call. So it might look something like
```go
- MULTIPLY
	- 5
	- PARENS
		- ADD
		- 3
		- FUNC
			- add
			- 2
			- 5
```


Examples of expressions:
- Prefix operators
	- `-5`
	- `!true`
	- `!false`
- Infix operators ("binary operators")
	- `5 + 5`
	- `5 - 5`
	- `5 / 5`
	- `5 * 5`
- Besides the basic arithmetic operations, there are also
	- `foo == bar`
	- `foo!=bar`
	- `foo < bar`
	- `foo > bar`
- We can use parentheses to group expressions and influence the order of evaluation:
	- `5 * (5 + 5)`
	- `((5 + 5) * 5) * 5`
- And there are `call` expressions:
	- `add(5, 5)`
	- `add(add(5, 5), add(5, 5))`
	- `max(5, add(2,4))`
- Identifiers are also expressions:
	- `foo * bar / foobar`
	- `add(foo, bar)`

## Top Down Opeator Precendence - Pratt Parsing
We wll be implementing a Pratt parser, which is an alternative to BNF/EBNF or CFG (context-free grammar). Some resoures:
- [Top Down Oerator Precedence](http://crockford.com/javascript/tdop/tdop.html)
- [Pratt Parsers: Expression Parsing Made Easy](http://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/) - which goes through implementing a Pratt parser in Java
The main difference between Pratt parsing vs BNF or CFG is that instead of associating parsing functions (think the `parseLetStatement` method) with grammar rules (as in BNF or EBNF), Pratt associiates these functions (which he calls "semantic code") with single token types. The crucial part of this idea is that each token type can have two parsing functions associated with it: depending on the token's position: infix or prefix.

## Terminology
A __prefix operator__ is an operator "in front of" its operand. Example:
```go
--5
```
Here the operator is `--` (derement), the operand is the integer literal 5 and the operator is in the prefix position.
A __postfix operator__ is an operator "after" its operand. Example:
```go
foobar++
```
(note that Monkey will not have postix operators).

An __infix operator__ sits between its operands:
```go
5 * 8
```
The `*`  operator sit in the infix operator between the two integer literals 5 and 8. Infix operators appear in __binary expressions__ - where the operator has two operands.

The last term is __operator precedence__ - which is just an alternaiveterm or __order of operations__.