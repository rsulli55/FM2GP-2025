
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

// But now we notice that we’re making mult_acc4 do one unnecessary test for
// odd(n), because we’re calling it with an even number. So we’ll do one halving
// and doubling on the arguments before we call it, giving us our final version:
int multiply4(int n, int a) {
    while (!odd(n)) {
        a = a + a;
        n = half(n);
    }
    if (n == 1) return a;
    // even(n − 1) =⇒ n − 1 ̸= 1
    return mult_acc4(a, half(n - 1), a + a);
}