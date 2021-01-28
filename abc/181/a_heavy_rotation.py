import unittest


class Solution():
    def solution(self, input: int):
        if input%2==0:
            return 'White'
        else:
            return 'Black'


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_data = {'input': 2, 'output': 'White'}

    def test_solution(self):
        res = Solution().solution(input=self.test_data['input'])
        self.assertEqual(res, self.test_data['output'])


if __name__ == '__main__':
    unittest.main()
