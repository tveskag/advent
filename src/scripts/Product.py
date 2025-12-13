"""
Author: Emil Henningsen
Advent of Code 2025, day 2.
"""

class Product:

    _invalid = False
    _number = 0
    _len = 0
    _leftPart = ""
    _rightPart = ""
    
    partTwo = True

    def __init__(self, number : str):
        length = len(number)
        self._len = length
        #no need to check uneven lengths, as they are always valid product numbers
        if length % 2 == 0 and not self.partTwo:
            half = int(length / 2)
            self._leftPart = number[:half]
            self._rightPart = number[half:]
            self.checkProductNumber()
        
        if self.partTwo:
            self.checkProductNumberPartTwo()

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

    def checkProductNumberPartTwo(self):
        #dividers to check (i.e. is number length 6, then dividers are 1, 2 and 3)
        dividers = [divider for divider in range(1, self._len) if self._len % divider == 0]
        print(dividers)
        #check if number is invalid with any of the found dividers
        numberString = str(self._number)
        print(numberString)
        for divider in dividers:
            parts = []
            for i in range(divider):
                part = numberString[i*divider:(i+1)*divider]
                print(part)
                parts.append(part)
            #check if all parts are the same
            self.setInvalid(all(part == parts[0] for part in parts))