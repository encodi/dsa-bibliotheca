def twoSum(nums, target):
    i = 0
    j = len(nums) - 1
    while i < j:
        if nums[i] + nums[j] == target:
            return [i, j]
        elif nums[i] + nums[j] < target:
            i += 1
        else:
            j -= 1
    return []


if __name__ == "__main__":
    print(twoSum([2, 7, 11, 15], 9))
    print(twoSum([2, 3, 4], 6))
