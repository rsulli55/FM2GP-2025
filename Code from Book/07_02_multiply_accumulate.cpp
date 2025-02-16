// Here we’re using NonCommutativeAdditiveSemigroup as a C++ concept, a set of
// requirements on types that we’ll discuss in Chapter 10. Instead of saying
// type- name, we name the concept we wish to use. Since concepts are not yet
// supported in the language as of this writing, we’re doing a bit of
// preprocessor slight-of- hand:

#define NonCommutativeAdditiveSemigroup typename
#define Integer typename

template <Integer N>
bool odd(N n) {
    return bool(n & 0x1);
}
template <Integer N>
N half(N n) {
    return n >> 1;
}

template <NonCommutativeAdditiveSemigroup A, Integer N>
A multiply_accumulate_semigroup(A r, N n, A a) {
    // precondition(n >= 0);
    if (n == 0) return r;
    while (true) {
        if (odd(n)) {
            r = r + a;
            if (n == 1) return r;
        }
        n = half(n);
        a = a + a;
    }
}