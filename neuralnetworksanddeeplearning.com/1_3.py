import math


def sigmoid(inputs, weights, bias):
    output = bias
    for input, weight in zip(inputs, weights):
        output += input * weight

    try:
        output = 1 / (1 + math.exp(-1 * output))
    except OverflowError:  # happens when denominator is too large
        output = 0
    return output


# generate all the assumed outputs from the old output layer
outputs = []
for i in range(10):
    outputs.append([0.001] * i + [0.99] + [0.001] * (9 - i))

for i in range(10):
    p = 10000
    w1 = [0] * 8 + [p] * 2
    w2 = [0] * 4 + [p] * 4 + [0] * 2
    w3 = [0, 0, p, p, 0, 0, p, p, 0, 0]
    w4 = [0, p, 0, p, 0, p, 0, p, 0, p]

    b = -1000

    o1 = sigmoid(outputs[i], w1, b)
    o2 = sigmoid(outputs[i], w2, b)
    o3 = sigmoid(outputs[i], w3, b)
    o4 = sigmoid(outputs[i], w4, b)

    print(o1, o2, o3, o4)
