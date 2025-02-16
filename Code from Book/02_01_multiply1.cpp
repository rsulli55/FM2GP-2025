auto odd(int n) -> bool { return n & 0x1; }
auto half(int n) -> int { return n >> 1; }

// the Egyptian multiplication algorithm in C++:
int multiply1(int n, int a) {
    if (n == 1) return a;
    int result = multiply1(half(n), a + a);
    if (odd(n)) result = result + a;
    return result;
}