auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

// getting back tail recursion
int mult_acc3(int r, int n, int a) {
    if (odd(n)) {
        r = r + a;
        if (n == 1) return r;
    }
    n = half(n);
    a = a + a;
    return mult_acc3(r, n, a);
}