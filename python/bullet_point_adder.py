import pyperclip

text = pyperclip.paste()

lines = text.split('\n')

for i in range(len(lines)):
    lines[i] = f'* {lines[i]}'
    print(lines[i])

text = '\n'.join(lines)
pyperclip.copy(text)
