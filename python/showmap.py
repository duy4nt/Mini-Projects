import webbrowser, sys, pyperclip
if len(sys.argv) > 1:
    address = ' '.join(sys.argv[1:])
else:
    adderss = pyperclip.paste()

webbrowser.open('https://www.openstreetmap.org/search?query=' + address)



