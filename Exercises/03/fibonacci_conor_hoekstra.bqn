# Fibonacci using Matrix Multiplication - O(n)
_Ip_ ← { 𝔽˝∘𝔾⎉1‿∞ } # inner product
Mm   ← +_Ip_×       # matmul
m    ← 2‿2⥊0∾˜3⥊1
FibN ← { ¯1⊑m⊸Mm⍟𝕩1‿0 }

•Show FibN ¨↕11 # ⟨ 0 1 1 2 3 5 8 13 21 34 55 ⟩

# Fibonacci using Matrix Exponentiation - O(logn)
FibHelper ← {
    𝕩≤1 ? m ;
    2|𝕩 ? m Mm (𝕊 𝕩-1) ;
    Mm˜ 𝕊 𝕩÷2
}

FibLogN ← 1⊸⌊×0‿1⊑FibHelper

•Show FibLogN ¨↕11 # ⟨ 0 1 1 2 3 5 8 13 21 34 55 ⟩
