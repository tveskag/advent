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
        #no need to check uneven lengths, as they are always valid product numbers
        if length % 2 == 0:
            half = int(length / 2)
            self._leftPart = number[:half]
            self._rightPart = number[half:]
            self.checkProductNumber()
        
        self._number = int(number)
        
    def isInvalid(self) -> bool:
        return self._invalid
    
    def setInvalid(self, invalid : bool):
        self._invalid = invalid

    def getProductNumber(self) -> int:
        return self._number

    def checkProductNumber(self):
        #print("left part: " + self._leftPart + ", right part: " + self._rightPart)
        if self._leftPart == self._rightPart:
            self.setInvalid(True)