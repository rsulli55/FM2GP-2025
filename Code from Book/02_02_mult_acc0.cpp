auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

// multiply-accumulate function:
int mult_acc0(int r, int n, int a) {
    if (n == 1) return r + a;
    if (odd(n)) {
        return mult_acc0(r + a, half(n), a + a);
    } else {
        return mult_acc0(r, half(n), a + a);
    }
}