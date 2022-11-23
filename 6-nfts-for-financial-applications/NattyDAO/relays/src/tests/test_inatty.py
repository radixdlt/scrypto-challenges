# Write some tests for the src/inatty.py module

import unittest
from relays.src.inatty import iNatty

class TestInatty(unittest.TestCase):
    
    def test_get_observations(self):
        self.assertEqual(iNatty.get_observations(), 200)

if __name__ == '__main__':
    unittest.main()