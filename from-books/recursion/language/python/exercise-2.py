def print_numbers(data):
    # Check if data is a list (or any iterable)
    if isinstance(data, list):
        for item in data:
            print_numbers(item)
    else:
        print(data)


def main():
    mixed = [
        1,
        2,
        3,
        [4, 5, 6],
        7,
        8,
        [
            9,
            [10, 11],
        ],
    ]

    print_numbers(mixed)


if __name__ == "__main__":
    main()
