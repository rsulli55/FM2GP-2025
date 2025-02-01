auto multiply0(int n, int a) -> int {
    if (n == 1) return a;
    return multiply0(n - 1, a) + a;
}