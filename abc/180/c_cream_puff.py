import unittest


class Solution():
    def solution(self, input: int):
        # you should replace from input to input() 
        # if you actually neet to submit your code.
        n = input
        ans = list()
        for i in range(1, n+1):
            if i*i > n:
                break
            if n%i==0:
                ans.append(i)
                ans.append(int(n/i))
        ans = list(set(ans))
        ans.sort()
        return ans


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.case = {'input': 6,
                'output': [1,2,3,6]}
    
    def test_solution(self):
        ans = Solution().solution(input=self.case['input'])
        self.assertEqual(ans, self.case['output'])


if __name__ == '__main__':
    unittest.main()
