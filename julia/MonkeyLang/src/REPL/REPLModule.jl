module REPLModule

using ..LexerModule
using ..TokenModule

export start_repl

const MONKEY_VERSION = v"0.1"

const MONKEY_AUTHOR = "Ifan Johnston"

const REPL_PRELUDE = """
Welcome to MonkeyLang $MONKEY_VERSION by $MONKEY_AUTHOR
"""

const PROMPT = "ml> "

const REPL_FAREWELL = "Goodbye 🙉"

isEOF(t::Token) = t.Type == EOF

function start_repl(; input::IO=stdin, output::IO=stdout)
    Base.exit_on_sigint(false)

    ## Environment stuff

    println(output, REPL_PRELUDE)

    while true
        print(output, PROMPT)

        try
            line = readline(input; keep=true)
            ## Note that Ctrl-D sends an EOF
            if isempty(line)
                println(output, REPL_FAREWELL)
                break
            end
            l = Lexer(string(strip(line, '\n')))
            while !isnothing(read_char(l))
                println(output, next_token!(l))
            end
        catch e
            if !isa(e, InterruptException)
                rethrow(e)
            end
            println(output, REPL_FAREWELL)
            break
        end
    end
end

end
