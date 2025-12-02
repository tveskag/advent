import numpy as np
import CircleIterator

instructions = np.genfromtxt("C:\\Users\\emilh\\Desktop\\code\\advent\\data\\day1.txt", dtype=np.str_)

dial = CircleIterator.CircleIterator(startIndex = 50, lower = 0, upper = 99)