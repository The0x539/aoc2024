Pad ← ⬚@.⊂:@.

S ← Pad↘1
N ← ⍜⇌S
E ← ⍜⍉S
W ← ⍜(⇌⍉)S

Move ← ⊟₈⊃(⍥N|⍥E|⍥S|⍥W|⍥(N W)|⍥(N E)|⍥(S W)|⍥(S E))

Part₁ ← (
  ⊃(=@X Move0|=@M Move1|=@A Move2|=@S Move3) # Find the characters, traveling in each direction
  ⍥₃↧                                        # Find places where the characters line up
  ⍥₃/+                                       # Count the hits
)

EcksMas ← (
  ⊃(=@A|=@M N W|=@M S W|=@S N E|=@S S E)
  ⍥₄↧
)

Rotate ← ≡⇌⍉

Part₂ ← (
  ⊃(∘|Rotate|⍥₂Rotate|⍥₃Rotate)
  ∩∩EcksMas
  ⍥₂/+⍥₃+
)

&fras"input.txt"
⊜∘⊸≠@\n
⊃Part₂Part₁
