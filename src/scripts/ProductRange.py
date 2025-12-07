"""
Author: Emil Henningsen
Advent of Code 2025, day 2.
"""

class ProductRange:
    
    _range : str
    _lowerNumber : str
    _upperNumber : str
    _numberOfProducts : int
    
    def __init__(self, input : str):
        self._range = input
        self._lowerNumber, self._upperNumber = self.splitRange(input)
        self._numberOfProducts = int(self._upperNumber) - int(self._lowerNumber)
        pass

    def splitRange(self, input : str) -> tuple[str, str]:
        parts = input.split(sep="-")
        return (parts[0], parts[1])
    
    def getLowerNumber(self) -> int:
        return int(self._lowerNumber)
    
    def getUpperNumber(self) -> int:
        return int(self._upperNumber)