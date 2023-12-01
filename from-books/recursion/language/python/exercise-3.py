# Write a function that accepts an array of strings and returns the total number of characters across all the strings.
# For example, if the input array is ["ab", "c", "def", "ghij"], the output should be 10 since there are 10 characters
# in total.
def numberOfChars(arr):
    if len(arr) == 1:
        return len(arr[0])

    sum = len(arr[0])

    return sum + numberOfChars(arr[1:])


def main():
    numberOfChars(["ab", "c", "def", "ghij"])


if __name__ == "__main__":
    main()
