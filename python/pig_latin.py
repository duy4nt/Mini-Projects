print('Enter the english language to translate into pig latin')
text = input('>')

VOWELS =('a', 'e', 'i', 'o', 'u')

pig_latin = []

for word in text.split():
    prefix_non_letters = ''
    
    while len(word) > 0 and not word[0].isalpha():
        prefix_non_letter += word[0]
        word = word[1:]

    if len(word) == 0:
        pig_latin.append(prefix_non_letters)
        continue
