Frequencies ← ◴▽⊸(↧⊃(≠@.)(≠@\n))/⊂
ToIndices   ← ▽:⇡⊸⧻

Locations ← (
  =
  ToIndices⊸⍜:∈1
  :⟜⊏
  ≡(□≡⊂:ToIndices)
  /◇⊂
)

Parse ← (
  ⊜∘⊸≠@\n
  ⊸Frequencies
  ⍜⊙¤⊸≡(□Locations)
  ⊙△
)

Pairs ← (
  ⊞⊟.        # cartesian product with self
  ↯ ∞_2_2    # flatten to a list of pairs of pairs
  ▽⊸≡(/↥≠°⊟) # remove pairings of a point with itself
  ◴≡⍆
  ≡°⊟
)

Antinodes₁ ← (
  ◡-
  ⊃(-|+⊙◌) # subtract from A and add to B
  ⊂
)

Antinodes₂ ← (
  ⊸-
  # hypothetically, the rough upper bound could be determined based on the stride and/or position
  # but for official inputs, it's performant enough to just overshoot and filter later
  -50⇡101
  # come up with 100 possible offsets
  ⊞×
  # add them all to the base node
  +⊙¤
  /⊂
)

Finalize ← (
  /◇⊂
  ▽⊸(≡/↧<)¤:
  ▽⊸(≡/↧≥)¤0_0
  ⧻◴
)

Part₁ ← Finalize ⍚(Antinodes₁ Pairs)
Part₂ ← Finalize ⍚(Antinodes₂ Pairs)

&fras"input.txt"
Parse
⊃Part₂Part₁
