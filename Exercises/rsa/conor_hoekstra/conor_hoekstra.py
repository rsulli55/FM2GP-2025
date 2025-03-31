import random
import math
from typing import Tuple, Optional


# Extended GCD implementation
def extended_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Returns (gcd, x, y) such that ax + by = gcd(a, b)"""
    if a == 0:
        return b, 0, 1
    gcd, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return gcd, x, y


# Multiplicative inverse
def multiplicative_inverse(a: int, n: int) -> Optional[int]:
    """Returns multiplicative inverse of a modulo n, or None if it doesn't exist"""
    gcd, x, y = extended_gcd(a, n)
    if gcd != 1:
        return None
    return x % n


# Generate a random prime number within a range
def generate_prime(min_val: int, max_val: int) -> int:
    """Generate a random prime number between min_val and max_val"""
    while True:
        num = random.randint(min_val, max_val)
        if is_prime(num):
            return num


def is_prime(n: int) -> bool:
    """Check if a number is prime using primality test"""
    if n <= 1:
        return False
    if n <= 3:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False

    # Check using 6k+-1 optimization
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True


# RSA Key generation
def generate_keys(bit_length: int = 1024) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    """
    Generate RSA public and private keys
    Returns ((n, e), (n, d)) where (n, e) is public key and (n, d) is private key
    """
    # Generate two random prime numbers
    p = generate_prime(2 ** (bit_length // 2 - 1), 2 ** (bit_length // 2))
    q = generate_prime(2 ** (bit_length // 2 - 1), 2 ** (bit_length // 2))
    while p == q:
        q = generate_prime(2 ** (bit_length // 2 - 1), 2 ** (bit_length // 2))

    # Calculate n and phi(n)
    n = p * q
    phi_n = (p - 1) * (q - 1)

    # Choose public exponent e
    e = 65537  # Common choice, a prime Fermat number
    while math.gcd(e, phi_n) != 1:
        e = random.randrange(3, phi_n, 2)

    # Calculate private exponent d
    d = multiplicative_inverse(e, phi_n)

    # Return public and private key pairs
    return ((n, e), (n, d))


# RSA Encryption/Decryption
def encrypt(message: str, public_key: Tuple[int, int]) -> list:
    """Encrypt a message using RSA public key"""
    n, e = public_key
    # Convert message to integers and encrypt
    cipher = [pow(ord(char), e, n) for char in message]
    return cipher


def decrypt(cipher: list, private_key: Tuple[int, int]) -> str:
    """Decrypt a message using RSA private key"""
    n, d = private_key
    # Decrypt and convert back to characters
    message = "".join([chr(pow(char, d, n)) for char in cipher])
    return message


# Example usage
if __name__ == "__main__":
    # Generate keys (using smaller bit_length for demonstration)
    public_key, private_key = generate_keys(bit_length=64)

    # Encrypt and decrypt a message
    message = "Hello, RSA!"
    encrypted = encrypt(message, public_key)
    decrypted = decrypt(encrypted, private_key)

    print(f"Original: {message}")
    print(f"Encrypted: {encrypted}")
    print(f"Decrypted: {decrypted}")
