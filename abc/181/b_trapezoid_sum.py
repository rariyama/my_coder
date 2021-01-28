import unittest
from typing import List


'''
等差数列の和を求める。
a = (s+l)*len/2
lenはlとsの差で求める。
'''

class Solution():
    def solution(self, n: int, data: str):
        ans = 0
        for i in range(n):
          inputs = list(map(int, data[i].split()))
          ans += int((inputs[0]+inputs[1])*(inputs[1]-inputs[0]+1)/2)
        return ans


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test = {'data_len': 3, 'data': ['11 13', '17 47', '359 44683']}
        self.collect = 998244353

    def test_solution(self):
        res = Solution().solution(n=self.test['data_len'], data=self.test['data'])
        self.assertEqual(res, self.collect)


if __name__ == '__main__':
    unittest.main()
