import math


def perceptron(inputs, weights, bias, c=1):

    output = bias * c
    for input, weight in zip(inputs, weights):
        output += input * weight * c
    if output > 0:
        return 1
    return 0


def sigmoid(inputs, weights, bias, c=1):

    output = bias * c
    for input, weight in zip(inputs, weights):
        output += input * weight * c

    output = 1 / (1 + math.exp(-1 * output))
    return output


def test_all_combos(func, c=1):
    weights = [-2, -2]

    inputs = [0, 0]
    print(func(inputs, weights, 3, c=c))

    inputs = [0, 1]
    print(func(inputs, weights, 3, c=c))

    inputs = [1, 0]
    print(func(inputs, weights, 3, c=c))

    inputs = [1, 1]
    print(func(inputs, weights, 3, c=c))
    print('\n')


test_all_combos(perceptron)

test_all_combos(sigmoid, 0.5)
test_all_combos(sigmoid, 1)
test_all_combos(sigmoid, 10)
test_all_combos(sigmoid, 100)

# fails to model a perceptron when w \cdot x + b = 0 because the activation function return 0.5
print(sigmoid([1, 1], [2, 2], -4))
