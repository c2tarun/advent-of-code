"""Advent of Code Day 5 - Doesn't He Have Intern-Elves For This?"""

import re

with open('day5input.txt') as f:
    strings = f.readlines()

# Answer One Regexes
naughty_regex = re.compile(r'ab|cd|pq|xy')
vowel_regex = re.compile(r'([aeiou].*){3,}')
double_regex = re.compile(r'(.)\1')
# Answer Two Regexes
repeated_regex = re.compile(r'(..).*\1')
gapped_regex = re.compile(r'(.).\1')

nice_2 = 0
for string in strings:
    if repeated_regex.search(string) and gapped_regex.search(string):
        nice_2 += 1
        print(string)

print("Answer Two =", nice_2)
