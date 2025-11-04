import random

adjective = input('> Input an adjective: ')
noun_first = input('> Input the first noun: ')
verb = input('> Input a verb: ')
noun_second = input('> Input the second noun: ')

lines = open(f'mad_libs_lines.txt', encoding='utf-8').read().strip().split('\n')

line = random.choice(lines)

line = line.replace('ADJECTIVE', adjective)
line = line.replace('NOUN', noun_first, 1)
line = line.replace('VERB', verb)
line = line.replace('NOUN', noun_second)

print(line)
