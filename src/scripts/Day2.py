"""
Author: Emil Henningsen
Advent of Code 2025, day 2.
"""

import numpy as np
import Product, ProductRange

instructions = np.genfromtxt("C:\\Users\\emilh\\Desktop\\code\\advent\\data\\emil\\day2.txt", dtype=np.str_, delimiter=",")

sum = 0
productRanges = [ProductRange.ProductRange(instruction) for instruction in instructions]
products = []
for productRange in productRanges:
    for number in range(productRange.getLowerNumber(), productRange.getUpperNumber() + 1):
        products.append(Product.Product(str(number)))

print(products[0])

invalidProductNumbers = [product.getProductNumber() for product in products if product.isInvalid()]

print(invalidProductNumbers)