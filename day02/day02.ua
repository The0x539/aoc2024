Ints  ← ⊜⋕⊸≠@  # a
Parse ← ⊜(□Ints)⊸≠@\n

Differences ← -⊃(↘1)(↙-1⊸⧻)

Monotonic ← ≍1⧻◴±Differences

InRange ← /↧↧⊃(≥1)(≤3)⌵Differences

Safe  ← ↧⊃Monotonic InRange
Part₁ ← °□/+⍚Safe

Remove   ← ⊂⊃(↙)(↘+1)
Removals ← ≡Remove ⊙¤ ⇡⊸⧻

AlmostSafe ← /↥≡Safe Removals
Safe₂      ← ↥⊃Safe AlmostSafe
Part₂      ← °□/+⍚Safe₂

&fras"input.txt"
Parse
⊃Part₂Part₁
