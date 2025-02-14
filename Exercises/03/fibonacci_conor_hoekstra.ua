# Experimental!
Math ~ "git: github.com/Omnikar/uiua-math"

Mm   ← Math~Mmp
m    ← ↯2_2<3⇡4
FibN ← ⊡¯2♭⍥(Mm m):1_0
∵FibN⇡10 # [1 1 2 3 5 8 13 21 34 55]

Idx     ← +⤙×◿2:>1.
FibH    ← |1 ⨬(m|Mm . FibH÷2|Mm m FibH-1)Idx. # FibHelper
FibLogN ← ⊢⊢FibH
∵FibLogN⇡10 # [1 1 2 3 5 8 13 21 34 55]
