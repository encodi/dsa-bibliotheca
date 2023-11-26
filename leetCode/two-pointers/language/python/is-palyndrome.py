def isPalyndrome(word):
    left = 0
    right = len(word) - 1
    while left < right:
        if word[left] != word[right]:
            return False
        left += 1
        right -= 1

    return True


if __name__ == "__main__":
    print(isPalyndrome("abba"))
    print(isPalyndrome("abbaa"))
