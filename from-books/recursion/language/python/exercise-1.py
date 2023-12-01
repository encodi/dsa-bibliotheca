# Write a function that recieves two  numbers called low and high, the function returns the sum of all the numbers
# from low to high.


def sum(low, high):
    if low > high:
        return 0
    return high + sum(low, high - 1)


if __name__ == "__main__":
    print(sum(1, 10))
