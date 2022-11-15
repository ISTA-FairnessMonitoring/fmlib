import math
import numpy as np
from matplotlib import pyplot as plt

def solve_linear_sum(n, b, i=0):
    if i == n:
        yield []
    else:
        for x in range(b + 1):
            for y in solve_linear_sum(n, b - x, i + 1):
                if x + sum(y) == b:
                    yield [x] + y

N, GAMMA = 3, 700
# U = [np.random.randint(low=0, high=7, size=7)]
U = [5, 15, 25]


def main():
    sls = solve_linear_sum(n=N, b=GAMMA)
    A = np.array([_ for _ in sls]) / GAMMA
    assert len(A) == math.comb(GAMMA+N-1, N-1)
    B = np.prod(A ** U, axis=1)
    print(np.unique(B, return_counts=True))
    
    print(1 - np.count_nonzero(B) / len(B))
    
    x, y = np.unique(B, return_counts=True)
    assert list(x) == sorted(x)
    plt.plot(x, np.cumsum(y) / len(B))
    plt.show()
    
if __name__ == '__main__':
    main()
