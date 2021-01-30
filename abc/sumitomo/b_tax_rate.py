import math
import unittest
from typing import Union


def solution(input: str) -> Union[str, int]:
    x = int(input)
    excl = math.ceil(x/1.08)
    if x == math.floor(excl*1.08):
      return excl
    else:
      return ':('


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [{'input': 432,
                            'expect': 400
                            },
                            {'input': 1079,
                             'expect': ':('
                             },
                             {'input': 1001,
                              'expect': 927
                             }]
    
    def test_solution(self):
        for test_case in self.test_cases:
            res = solution(test_case['input'])
            self.assertEqual(res, test_case['expect'])


if __name__ == '__main__':
    unittest.main()
