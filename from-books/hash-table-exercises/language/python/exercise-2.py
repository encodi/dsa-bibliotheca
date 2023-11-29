# Write a funciton that accepts an array of strings and returns the first duplicate value it finds.
# For example, if the array is ["a", "b", "c", "d", "c", "e", "f"], the function should return "c",
# since it's duplicated within the array. (You can assume that there's one pair of duplicates within the array.).
# Make sure the function has an efficiency of O(n).
def firstDuplicate(arr: list[str]) -> str:
    hashMap: dict[str, int] = {}

    for letter in arr:
        hashMap[letter] = hashMap.get(letter, 0) + 1

    for index, value in hashMap.items():
        if value > 1:
            return index

    return ""
