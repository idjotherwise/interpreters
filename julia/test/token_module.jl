using Test

using MonkeyLang.TokenModule

@testset "Making a Token" begin
  t = Token(INT, "1")
  @test t.type == INT
  @test t.literal == "1"
end
