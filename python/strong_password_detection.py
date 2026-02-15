import re

password_re = re.compile(r'[a-zA-Z]([a-z]*[A-Z]*[0-9]+[_.@$]+){8, }')

passord = input("Enter the password: ")
print(passord)

if len(password_re.findall(passord)):
    print("sounds good")
else:
    print("lets try that once more")
