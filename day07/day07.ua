Ints  ← □⊜⋕⊸(↧⊃(≠@ |≠@:))
Parse ← ⊜Ints⊸≠@\n

Eval₁ ← (
  :°⊂ # split off the calibration result
  :°⊂ # split off the first operand
  ∧(⊂⊃+×)
  ∈
)

Finalize ← /+≡◇⊡0▽≡°□

Part₁ ← Finalize⊸⍚Eval₁

Concat ← +⊙×:⊸⍜ₙ₁₀(+1⌊)

Eval₂ ← (
  :°⊂
  :°⊂
  ∧(⊂⊂⊃(+|×|Concat))
  ∈
)

Part₂ ← Finalize⊸⍚Eval₂

&fras"test.txt"
Parse
⊃Part₂Part₁
