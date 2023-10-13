module ParserModule

using Expronicon: @match

using ..ASTModule, ..LexerModule, ..TokenModule

import ..LexerModule: next_token!

export Parser, parse_program

mutable struct Parser
    lexer::Lexer
    cur_token::Token
    peek_token::Token
end

function Parser(lexer::Lexer)::Parser
    cur_token = next_token!(lexer)
    peek_token = next_token!(lexer)
    Parser(lexer, cur_token, peek_token)
end

function next_token!(p::Parser)::Token
    p.cur_token = p.peek_token
    p.peek_token = next_token!(p.lexer)
    p.cur_token
end

function expect_peek(p::Parser, expected::TokenType)::Bool
    if p.peek_token.type == expected
        true
    else
        false
    end
end

function parse_program(p::Parser)
    @debug "Parsing program"
    stmts = Statement[]
    while p.cur_token.type != EOF
        stmt = parse_statement(p)
        if !isnothing(stmt)
            push!(stmts, stmt)
        end
        next_token!(p)
    end
    Program(stmts)
end

function parse_statement(p::Parser)::Union{Statement,Nothing}
    @debug "Parsing statement"
    @match p.cur_token.type begin
        LET => parse_let_statement(p)
        _ => nothing
    end
end

function parse_let_statement(p::Parser)::Union{LetStatement,Nothing}
    @debug "Parsing let statement, $p"
    stmt = LetStatement(p.cur_token, Identifier(p.cur_token, p.cur_token.literal), Expression())
    if !expect_peek(p, IDENT)
        return nothing
    end
    if !expect_peek(p, ASSIGN)
        return nothing
    end
    # skip until semicolon
    while p.cur_token != SEMICOLON
        next_token!(p)
    end
    stmt
end

end # end ParserModule