import re, pyperclip

text = pyperclip.paste()

# TODO: Make a regex that cleans the dates and formats them into single format
dates = text.split('\n')

for i in range(len(dates)):
    

pyperclip.copy(text)
