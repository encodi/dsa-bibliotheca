# Write a function that acceps a string that contains all the letters of the alphabet
# except one and returns the missing letter. For example, the string "the quick brown box
# jumps over a lazy dog" contains all the letters of the alphabet except the letter, "f".
# The function should have a time complexity of O(n).
def generateAlphabet() -> dict[str, int]:
    alphabet: dict[str, int] = {}

    for i in range(26):
        letter: str = chr(97 + i)
        alphabet[letter] = i + 1

    return alphabet


def missingLetter(sentence: str) -> str:
    alphabetHashMap: dict[str, int] = generateAlphabet()
    sentenceMap: dict[str, int] = {}

    for letter in sentence:
        if not sentenceMap.get(letter):
            sentenceMap[letter] = 1

    missing: str = ""
    for letter in alphabetHashMap:
        if not sentenceMap.get(letter):
            missing = letter
            break

    return missing


print(missingLetter("the quick brown box jumps over a lazy dog"))
