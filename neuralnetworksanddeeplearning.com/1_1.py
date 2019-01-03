def perceptron(inputs, weights, bias, c=1):

    output = bias * c
    for input, weight in zip(inputs, weights):
        output += input * weight * c
    if output > 0:
        return 1
    return 0


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


test_all_combos(perceptron)
test_all_combos(perceptron, 2)
test_all_combos(perceptron, 2000)
