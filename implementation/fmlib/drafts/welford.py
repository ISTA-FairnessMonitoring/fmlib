import unittest
import numpy as np 

class WelfordOnline:
    @staticmethod
    def update(n, mean, m2, new_x):
        new_n = n + 1
        new_mean = mean + (new_x - mean) / new_n
        new_m2 = m2 + (new_x - mean) * (new_x - new_mean)
        return (new_n, new_mean, new_m2)
    
    @staticmethod
    def finalize(n, mean, m2):
        if n < 2:
            return None
        return (mean, m2 / (n - 1))


class TestWelford(unittest.TestCase):
    N = 5000
    def test_binomial(self):
        n, mean, m2 = (0, 0, 0)
        x = np.random.binomial(n=1, p=0.5, size=self.N)
        for x_ in x:
            n, mean, m2 = WelfordOnline.update(n, mean, m2, x_)
        
        mean, var = WelfordOnline.finalize(n, mean, m2)
        
        self.assertAlmostEqual(mean, 0.5, delta=0.015)
        self.assertAlmostEqual(var, 0.25, delta=0.05)
    
    def test_uniform(self):
        n, mean, m2 = (0, 0, 0)
        x = np.random.uniform(low=0, high=1, size=self.N)
        for x_ in x:
            n, mean, m2 = WelfordOnline.update(n, mean, m2, x_)
        
        mean, var = WelfordOnline.finalize(n, mean, m2)
        
        self.assertAlmostEqual(mean, 0.5, delta=0.015)
        self.assertAlmostEqual(var, 1/12, delta=1/36)
    
    
    def test_normal(self):
        n, mean, m2 = (0, 0, 0)
        x = np.random.normal(loc=0.0, scale=0.5, size=self.N)
        for x_ in x:
            n, mean, m2 = WelfordOnline.update(n, mean, m2, x_)
        
        mean, var = WelfordOnline.finalize(n, mean, m2)
        
        self.assertAlmostEqual(mean, 0.0, delta=0.015)
        self.assertAlmostEqual(var, 0.25, delta=0.05)
    
if __name__ == '__main__':
    unittest.main()
