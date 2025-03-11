import functools
from typing import Iterable

def horner(coeffs: Iterable[float], xis: float) -> float:
    return functools.reduce((lambda acc, upd: acc * xis + upd), coeffs)

if __name__ == "__main__":
    # 4x**7 -3x**6 + 2x**5 -5x**4 +7x**3 -7x**2 +1x**1 -8 = 166671953143
    assert horner((4, -3, 2, -5, 7, -7, 1, -8), 33) == 166671953143
    print("✅")
