using Test
using MonkeyLang

TEST_FILES = [
  "token_module.jl",
  "lexer_module.jl",
  "object_module.jl",
  "repl_module.jl",
  "ast_module.jl",
  "parser_module.jl"
]

@testset "MonkeyLang.jl" begin
  for t in TEST_FILES
    @info "Testing $t..."
    path = joinpath(@__DIR__, t)
    @eval @time @testset $t begin
      include($path)
    end
  end
end
