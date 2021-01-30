import unittest


def solution(input: str):
    arr = list(map(int, input.split()))
    a = arr[0]
    b = arr[1]
    ans = 0
    plug=1
    while plug<b:
      plug+=(a-1)
      ans+=1
    return ans    


class Solution(unittest.TestCase):
    def setUp(self):
        self.test = '4 10'
        self.expect = 3

    def test_solution(self):
        res = solution(self.test)
        self.assertEqual(res, self.expect)


if __name__ == '__main__':
    unittest.main()
