Ints  ← ⊜⋕⊸≠ @     # split by spaces and parse the split'd ints
Parse ← ⊜Ints⊸≠@\n # split by newlines

Split ← ≡⊃(⊡0)(⊡1) # pick first and second columns

Part₁ ← /+⌵- ∩⍆ Split

Max    ← ⊡0⊏⊸⍖
Counts ← ∧(insert : +1 ⬚0◡get):[]

Part₂ ← (
  Split
  ⊙Counts   # get counts of second column
  ⊙¤        # ??????????????
  ⟜(≡⬚0get) # lookup counts[first column], keeping first column
  /+×
)

&fras"test.txt"
⊃Part₂Part₁ Parse
