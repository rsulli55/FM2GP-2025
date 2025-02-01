auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

int mult_acc1(int r, int n, int a) {
    if (n == 1) return r + a;
    if (odd(n)) r = r + a;
    return mult_acc1(r, half(n), a + a);
}