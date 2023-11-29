# Write a function that returns the intersection of two arrays. The intersection is a third array
# that contains all values contained within the first two arrays. For example, the intersection
# of [1, 2, 3] and [2, 3, 4] is [2, 3]. Your function should have a complexity of O(n). Do not use
# any built-in functions in the language used.


def intersection(arr1: list[int], arr2: list[int]) -> list[int]:
    hashMap: dict[int, int] = {}

    for element in arr1:
        hashMap[element] = hashMap.get(element, 0) + 1

    ans: list[int] = []
    for element in arr2:
        if hashMap.get(element, 0) > 0:
            ans.append(element)

    return ans


if __name__ == "__main__":
    print(intersection([1, 2, 3, 4], [3, 4, 5, 6]))
