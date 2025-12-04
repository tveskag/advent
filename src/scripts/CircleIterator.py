"""
Author: Emil Henningsen
Advent of Code 2025, day 1.
"""
class CircleIterator:
    #properties
    _index = 0
    _counter = 0
    _dist = 0
    lower = 0
    upper = 0

    #Constructor
    def __init__(self, startIndex = 0, lower = 0, upper = 99):
        self._index = startIndex
        if lower > upper:
            raise Exception("Lower bound on Circular Iterator cannot be greater than Upper!")
        self.lower = lower
        self.upper = upper
        self._dist = upper - lower

    #access
    def getIndex(self):
        return self._index

    def getCounter(self):
        return self._counter

    #methods

    def finalizeSteps(self):
        #Implement checks after an iteration
        if self._index == 0:
            self._counter += 1

    def iterateForwards(self, numberOfSteps):
        for step in range(numberOfSteps):
            if self._index == 99:
                self._index = 0
                #For day 1 part 2, we also count each time it passes 0:
                if step != (numberOfSteps - 1):
                    self._counter += 1
            else:
                self._index += 1
        self.finalizeSteps()

    def iterateBackwards(self, numberOfSteps):
        for step in range(numberOfSteps):
            if self._index == 0:
                #For day 1 part 2, we also count each time it passes 0:
                if step != 0:
                    self._counter += 1
                self._index = 99
            else:
                self._index -= 1
        self.finalizeSteps()