"""
Author: Emil Henningsen
Advent of Code 2025, day 1.
"""

import numpy as np
import CircleIterator
import DialInstruction

instructions = np.genfromtxt("C:\\Users\\emilh\\Desktop\\code\\advent\\data\\emil\\day1.txt", dtype=np.str_)

dial = CircleIterator.CircleIterator(startIndex = 50, lower = 0, upper = 99)

moves = [DialInstruction.DialInstruction(instruction) for instruction in instructions]

for move in moves:
    print("Direction: " + str(move.forward) + " steps: " + str(move.steps))
    print("Dial start: ", dial.getIndex())
    if move.forward:
        dial.iterateForwards(move.steps)
    else:
        dial.iterateBackwards(move.steps)

    print("Dial finish: ", dial.getIndex(), " and the counter: ", dial.getCounter())

print(dial.getCounter())