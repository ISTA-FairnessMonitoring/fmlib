import math
import numpy as np
from matplotlib import cm, pyplot as plt

def solve_linear_sum(n, b, i=0):
    if i == n:
        yield []
    else:
        for x in range(b + 1):
            for y in solve_linear_sum(n, b - x, i + 1):
                if x + sum(y) == b:
                    yield [x] + y

N, GAMMA = 2, 200
# U = [np.random.randint(low=0, high=7, size=7)]
U = [6, 4]
U_P = [7, 7]

def beta(x, a, b):
    return x ** a * (1-x) ** b

def main():
    X = np.linspace(0, 1.0, GAMMA)

    Y = X
    X, Y = np.meshgrid(X, Y)
    Z =  beta(X, *U) * beta(Y, *U_P)
    
    fig = plt.figure()
    ax = plt.axes(projection='3d')
    ax.plot_surface(X, Y, Z, rstride=1, cstride=1, cmap='viridis', edgecolor='none')
    plt.show()

    
if __name__ == '__main__':
    main()
