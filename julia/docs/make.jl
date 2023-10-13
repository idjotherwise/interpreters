using Documenter
using MonkeyLang

makedocs(
    sitename="MonkeyLang",
    format=Documenter.HTML(),
    modules=[
        MonkeyLang,
        MonkeyLang.ObjectModule,
        MonkeyLang.TokenModule,
        MonkeyLang.LexerModule,
    ]
)

# Documenter can also automatically deploy documentation to gh-pages.
# See "Hosting Documentation" and deploydocs() in the Documenter manual
# for more information.
#=deploydocs(
    repo = "<repository url>"
)=#
