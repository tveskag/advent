"""
Author: Emil Henningsen
Advent of Code 2025, day 1.
"""

class DialInstruction:
    forward = True
    steps = 0

    def __init__(self, instruction):
        #instruction always in format R49
        direction = instruction[0]
        self.steps = int(instruction[1:])

        if direction == "L":
            self.forward = False
        elif direction == "R":
            self.forward = True
        else:
            raise Exception("Unsupported dial direction")