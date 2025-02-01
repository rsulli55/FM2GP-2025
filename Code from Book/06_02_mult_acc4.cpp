auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

// back to iterative
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