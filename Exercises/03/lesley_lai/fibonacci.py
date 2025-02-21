import numpy as np

# Expect `a` and `op` to form a semigroup
def power_semigroup(a, n, op):
    def power_accumulate_semigroup(r, a, n, op):
        if n == 0:
            return r
        while True:
            if n % 2 != 0:
                r = op(r, a);
                if n == 1:
                    return r
            n //= 2
            a = op(a, a);

    while n % 2 == 0:
        a = op(a, a)
        n //= 2
    if n == 1:
        return a
    else:
        return power_accumulate_semigroup(a, op(a, a), (n - 1) // 2, op)

# Expect `a` and `op` to form a monoid with identity as `identity`
def power_monoid(a, n, op, identity):
    if n == 0:
        return identity
    else:
        return power_semigroup(a, n, op)

def fib(n):
    A = [[1, 1], [1, 0]]
    A = np.array(A, dtype=np.int64)
    I = np.array([[1, 0], [0, 1]], dtype=np.int64)
    result = power_monoid(A, n, np.dot, I)
    return result[1, 0]

for i in range(50):
    print(f"fib({i}) = {fib(i)}")
