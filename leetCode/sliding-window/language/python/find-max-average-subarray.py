def findMaxAverage(nums, k):
    max_sum = sum(nums[:k])
    cur_sum = max_sum
    for i in range(k, len(nums)):
        cur_sum += nums[i] - nums[i - k]
        max_sum = max(max_sum, cur_sum)
    return max_sum / k


if __name__ == "__main__":
    print(findMaxAverage([1, 12, -5, -6, 50, 3], 4))
