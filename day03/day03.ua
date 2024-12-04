Part₁ ← (
  regex "mul\\((\\d+),(\\d+)\\)"
  ≡(⋕↘1)
  /+≡/×
)

Part₂ ← (
  regex "(^|do\\(\\))((.|\n)*?)($|don't\\(\\))"
  /◇⊂≡⊡2
  Part₁
)

&fras"input.txt"
⊃Part₂Part₁
