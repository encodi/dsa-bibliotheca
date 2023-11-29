# Write a function that returns the first non-duplicated character in a string.
# For example, the string "minimum" has two characters that only exist once, the "n" and "u",
# so your function should return the "n", since it occurs first. The function should have an efficiency of O(n).


def firstNonDuplicate(sentence: str) -> str:
    hashMap: dict[str, int] = {}

    for letter in sentence:
        hashMap[letter] = hashMap.get(letter, 0) + 1

    for letter in hashMap:
        if hashMap.get(letter) == 1:
            return letter

    return ""


if __name__ == "__main__":
    print(firstNonDuplicate("minimum"))
