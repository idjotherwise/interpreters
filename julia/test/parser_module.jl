using Test

using MonkeyLang.LexerModule
using MonkeyLang.TokenModule
using MonkeyLang.ParserModule

@testset "Parsing a Program" begin
    input = """
    let x = 5;
    let y = 10;
 let foobar = 838383;
    """
    l = Lexer(input)
    p = Parser(l)

    program = parse_program(p)

    @test !isnothing(program)
    @test length(program.statements) == 3
    expected = [
        ("x", 5),
        ("y", 10),
        ("foobar", 838383),
    ]
    for (e, s) in zip(expected, program.statements)
        @test token_literal(s) == "let"
        @test l.identifier == e[1]
    end

end
