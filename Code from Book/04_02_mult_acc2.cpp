auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

// So we can reduce the number of times we have to compare with 1 by a factor of
// 2, simply by checking for oddness first:
int mult_acc2(int r, int n, int a) {
    if (odd(n)) {
        r = r + a;
        if (n == 1) return r;
    }
    return mult_acc2(r, half(n), a + a);
}