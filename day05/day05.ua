Ints         ← ⊜⋕⊸≠
ParseRules   ← ⊜(Ints@|)⊸≠@\n
ParseUpdates ← ⊜(□Ints@,)⊸≠@\n

Parse ← ⊃(ParseUpdates↘+2)(ParseRules↙)⊸(⊗1⌕"\n\n")

Relevant ← ▽⊸(≡/↧∈) # Find the rules whose "before" and "after" halves both show up in the update

First ← (
  ∩◴≡⊃(⊡0)(⊡1) # get "before" and "after" halves, deduplicated
  ▽¬⊸∈:        # find the "before" value not seen in "after"
)

# takes a value and a list of rules
# removes all rules that have the value on either side
Remove ← ▽⊸(¬≡/↥∈)

Iterate ← (
  ⊸First    # determine the first unselected value and push it
  ⊙⍜⊙◌(⊂:). # add the value to the sorted update list, without popping it
  Remove    # pop the value and remove it from the rules
)

# convert a list of relevant rules to a sorted list of pages
ToSorted ← ⊂:♭ ⍢(Iterate|>2⊸⧻) ⊙[]

Solve ← ⍚(ToSorted Relevant)⊙¤

Finalize ← /◇+⍚(⊡÷2-1⊸⧻)

Part₁ ← Finalize ▽⊸= ⟜Solve
Part₂ ← Finalize ▽⊸≠ ⟜Solve

&fras"input.txt"
Parse
⊃Part₂Part₁
