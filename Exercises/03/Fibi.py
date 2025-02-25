

import numpy as np

def fib(n):
    I = np.array([[1, 1], 
              [1, 0]])
    res =  np.linalg.matrix_power(I,n-1)  *np.array([1, 0])
    return res[0,0]

# power function from the book 

def power(a, n , Op):
    while not odd( n ) :
        a = Op(a,a)
        n = half(n)
    if n ==1 : return a 
    return power_acc(a,Op(a,a),half(n-1),Op)


def power_acc(r,a,n,Op):
    if n == 0 : return r
    while True : 
        if odd(n):
            r = Op(r,a)
            if n ==1 : return r
        n = half(n)
        a = Op(a,a)


# We do power so out helper functions will look like this 

def half(a):
    return a //2

def odd(n):
    return not (n % 2 ==0 )

def Op(x,y):
    return x@y


# The final function
def fib_UsingPower(n):
    I = np.array([[1, 1], 
              [1, 0]])    
    res = power(I,n-1,Op)  *np.array([1, 0])
    return res[0,0]

# Some testibng 
n = 2
while( n< 17): 
    print(str('Fibi : ')+str(n)  + "  " + str( fib_UsingPower(n) )  )
    n=n+1
