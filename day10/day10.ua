Parse ← ⊜≡⋕⊸≠@\n

FindZeros ← (
  =0
  ⊸≡(∈:1)
  ▽:⇡⊸⧻
  ⊙::.
  ⊏
  ≡(□▽:⇡⊸⧻)
  ⍚≡⊂:
  /◇⊂
)

Dirs ← [0_¯1 1_0 0_1 ¯1_0]

InBounds ← (
  ▽⊸≡(/↧>0_0)
  ▽≡/↧<¤:◡(⊙△)
)

&fras"test.txt"
Parse

⊸FindZeros

Iterate ← (
  /⊂⊞+Dirs
  InBounds
  ◡⍜⊙¤≡⊡
)

F ← ▽∘=⊙Iterate

# F1+⇡9

F1
F2
F3
F4
F5
F6
F7
F8
F9

⧻
