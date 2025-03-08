
# Horner's Rule
PolyEval ← { (⊣+𝕨⊸×)´⌽0∾𝕩 } 

# Alternate
PolyEval2 ← { +´𝕩×⌽𝕨⋆↕≠𝕩 } # Explicit
PolyEval3 ← +´⊢×(⌽⋆⟜(↕≠))  # Tacit


•SHow 10 PolyEval  4‿7‿1‿7‿3
•Show 10 PolyEval2 4‿7‿1‿7‿3 
•Show 10 PolyEval3 4‿7‿1‿7‿3 
