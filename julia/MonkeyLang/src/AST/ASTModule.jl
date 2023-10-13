module ASTModule

using ..TokenModule

export Node, Program, Statement, Expression, Identifier, LetStatement, token_literal

abstract type Node end
nodetype(::Type{Node}) = Expression()
token_literal(n::T) where {T} = token_literal(nodetype(T), n)

Base.string(node::Node) = token_literal(node)
Base.show(io::IO, node::Node) = print(io, string(node))

# maybe statement and expression could be traits instead?
struct Statement <: Node end
nodetype(::Type{Statement}) = Statement()
token_literal(::Statement, node::Node) = node.token.literal

struct Expression <: Node end
nodetype(::Type{Expression}) = Expression()
token_literal(::Expression, node::Node) = node.token.literal

struct Program <: Node
    statements::Vector{Statement}
end

function token_literal(::Statement, p::Program)
    if isempty(p.statements)
        ""
    else
        token_literal(p.statements[1])
    end
end

nodetype(::Type{Program}) = Statement()

struct Identifier <: Node
    token::Token
    value::String
end
nodetype(::Type{Identifier}) = Expression()

struct LetStatement <: Node
    token::Token # the LET token
    name::Identifier
    value::Expression
end
nodetype(::LetStatement) = Statement()

end