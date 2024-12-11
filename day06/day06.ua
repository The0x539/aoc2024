FindCaret ← ⊂⊃(⊗1∈:@^|/↧≡⊗@^)
Parse ← (
  ⊜∘⊸≠@\n
  ⊃(FindCaret|=@#)
)

PickDir ← ⊡:[¯1_0 0_1 1_0 0_¯1]

InBounds ← /↧⊂⊃(≥0_0|<:⊙△)⋅⊙∘

Move ← |3.3 (
  ⍜⊙⋅∘+⊸PickDir # try going forwards

  ◡(⬚0⊡ ⋅⊙∘) # check whether we hit a wall

  # if we hit a wall, step back, turn, and move again
  ⨬∘(
    ⍜⊙⋅∘-⊸PickDir # go backwards
    ◿4+1          # rotate clockwise
    Move          # try again
  )
)

Iterate₁ ← (
  Move    # take a step
  ⍜⋅⊙⋅∘⟜⊂ # add new position to list
)

TracePath ← (
  ⊙::⊸¤               # list of positions
  0                   # starting direction
  ⍢Iterate₁ ◡InBounds # step until out-of-bounds
  ↘1⍥₃◌               # omit final (OoB) position
)

Part₁ ← ⧻◴TracePath

Iterate₂ ← (
  ⟜Move          # take a step, then push a copy of the previous direction
  ⊸≠             # has the direction changed?
  ⨬∘(⍜⊙⋅⋅⋅∘⊂ ◡⊂) # if so, it's a collision - add (pos, dir) to the list
)

DejaVu ← /↥≠⊃(⧻|⧻◴) ⍥₃◌ # check if the collisions list is non-unique

NotDone ← ↧⊃(InBounds|¬DejaVu)

HasLoop ← (
  ⊙⊙(°△0_3)          # list of collisions
  0                  # starting direction
  ⍢Iterate₂ ◡NotDone # step until out-of-bounds or in a loop
  DejaVu
)

AddObstacle ← ⍜⊡⋅1

Part₂ ← (
  ◴◡TracePath
  ⍜⊙⋅∘(≡AddObstacle⊙¤) # try an obstacle at each position along the original path
  /+≡HasLoop¤          # takes 2-3 minutes :(
)

&fras"input.txt"
Parse
⊃(Part₂|Part₁)
