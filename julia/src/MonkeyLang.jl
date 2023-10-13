module MonkeyLang

SOURCES = [
  "Token/TokenModule.jl",
  "Object/ObjectModule.jl",
  "Lexer/LexerModule.jl",
  "REPL/REPLModule.jl",
  "AST/ASTModule.jl",
  "Parser/ParserModule.jl"
]

for source in SOURCES
  try
    include(source)
  catch
    # Throw an explicit error mentioning the file, otherwise we don't know which file failed.
    @error "An error occurred while including source file $source"
    rethrow()
  end
end

using Reexport

@reexport using .LexerModule, .ObjectModule, .TokenModule, .REPLModule, .ParserModule

end # module MonkeyLang
