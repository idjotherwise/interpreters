monorepo of monkeylang implementations that I'm working on

- Go
	- [x] Interpreter
	- [] Compiler
		- [] Bytecodes
- Rust
	- [] Intepreter
		- [] Lexer
		- [] AST
		- [] Eval
- Julia
	- [] Intepreter
		- [x] Lexer
		- [x] AST
		- [x] Eval
		- [x] REPL
		- [] ???

Also contains some monkeylang code snipets, such as doing a `sum` using a `reduce` function:

```
let reduce = fn(arr, initial, f) { let iter = fn(arr, result) { if (len(arr) == 0) { result } else {iter(rest(arr), f(result, first(arr)));}};iter(arr, initial);};

let sum = fn(arr) { reduce(arr, 0, fn(initial, el) { initial + el })};
```
