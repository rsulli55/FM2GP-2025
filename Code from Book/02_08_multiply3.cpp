
auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

int mult_acc4(int r, int n, int a) {
    while (true) {
        if (odd(n)) {
            r = r + a;
            if (n == 1) return r;
        }
        n = half(n);
        a = a + a;
    }
}

// Tis is pretty good, except when n is a power of 2. e first thing we do is
// subtract 1, which means that mult_acc4 will be called with a number whose bi-
// nary representation is all 1s, the worst case for our algorithm. So we’ll
// avoid this by doing some of the work in advance when n is even, halving it
// (and doubling a) until n becomes odd:
int multiply3(int n, int a) {
    while (!odd(n)) {
        a = a + a;
        n = half(n);
    }
    if (n == 1) return a;
    return mult_acc4(a, n - 1, a);
}