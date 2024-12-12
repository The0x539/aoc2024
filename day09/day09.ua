Parse ← (
  ▽⊸∈"0123456789"
  ≡⋕
  ⬚0↯ ∞_2
)

Expand₁ ← (
  °⊟₃
  ⊙∩¤
  ↯:
  ⊙(↯:¯∞)
  ⊂
)

Compact₁ ← (
  ⟜⊣
  ↙-1⊸⧻
  ⊸⊗¯∞
  ⍜⊡◌
)

Enumerate ← ≡⊂⇡⊸⧻

Checksum₁ ← /+≡/×Enumerate

Append₁ ← (
  °⊂            # pop front
  ⍢(⍜⇌°⊂◌|⊸=¯∞) # if blank, pop from back until we get a non-blank
  ⍜⊙⋅∘(⊂:)      # push to accumulator
)

Part₁ ← (
  Enumerate
  /◇⊂≡(□Expand₁)
  ⊙[]
  ◌⍢(Append₁|>0⊸⧻) # append to accumulator until source is empty
  Checksum₁
)

Process₂ ← (
  °⊟₃
  ⍜(⋅⊙⋅⋅⋅∘)(⊙+⇌◡⊂) # Build file map entry and add length to cursor
  ⍜(⊙⊙⋅⋅∘)insert   # Insert file map entry
  ⍜(⊙⋅⋅∘)(⊃⊂+)     # Create gap list entry
  ⊂:⇌              # Append gap list entry
)

Relocate₂ ← (
  ⊡₀◡⊡
  ⊙⊙⊙(⊡₁◡get)
  ⍜⊙⋅⋅∘⊸⊂           # Construct updated file map entry
  ⍜(⊙⋅⋅⋅∘)(insert:) # Insert updated entry into map

  ⊙⊙(×1_¯1) # Position of gap increases, while length decreases
  ⍜⊡+
)

FindGap ← (
  ≥∩≡⊡₁¤
  ⬚¯1⊗1
)

Compact₂ ← (
  ⍜(⊙⋅∘)◡get

  ⊸FindGap
  ⊸≥0 # does a suitable gap exist?
  ⨬0(
    ⊙⊙(⊡₀◡get) # Get the file's current position
    ⊡₀◡⊡       # Get the candidate new position
    ⊙:⊙⊙:      # Rearrange stack
    >          # Check if new position is further left
  )
  ⨬(⊙◌◌)(Relocate₂)
)

Checksum₂ ← /+×+⇡:°⊟:⟜⊡

Part₂ ← (
  Enumerate
  ⊙0         # current position
  ⊙(map[][]) # file location map
  ⊙(↯0_2[])  # list of gaps

  ∧Process₂
  ⊙⊙◌

  ⇌⇡⊸⧻
  ∧Compact₂
  ◌

  ⇡⊸⧻
  /+≡Checksum₂⊙¤
)

&fras "input.txt"
Parse
⊃Part₂Part₁
