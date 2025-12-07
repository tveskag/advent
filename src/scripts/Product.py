"""
Author: Emil Henningsen
Advent of Code 2025, day 2.
"""

class Product:

    _invalid = False
    _number = 0
    _leftPart = ""
    _rightPart = ""

    def __init__(self, number : str):
        length = len(number)
        self._leftPart = number[:length]
        self._rightPart = number[length:]
        self._number = int(number)
        self.checkProductNumber()

    def isInvalid(self) -> bool:
        return self._invalid
    
    def setInvalid(self, invalid : bool):
        self._invalid = invalid

    def getProductNumber(self) -> int:
        return self._number

    def checkProductNumber(self):
        if self._leftPart == self._rightPart:
            self.setInvalid(True)