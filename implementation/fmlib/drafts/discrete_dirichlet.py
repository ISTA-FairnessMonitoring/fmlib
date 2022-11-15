import math
from matplotlib import pyplot as plt

FACT = [1 for _ in range(2000)]

for i in range(1, 240):
    FACT[i] = FACT[i-1] * i
    
comb = lambda n, r: FACT[n] // (FACT[r] * FACT[n-r])

G = 100

B = []
for n in range(2, 101):
    B.append(1 - comb(G-1, n-1) / comb(G+n-1, n-1))
    
plt.plot(range(2, 101), B)
plt.show()