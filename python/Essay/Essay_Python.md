Python is High Level Language. It has become one of the most popular language because of its simple syntax and huge library. 
Python is an Interpreted language. Interpreted language usually emans that the code is not directly compiled to native machine code ahead to time for direct execution by the hardware. 
Python uses 2 stage process that involves compilation to bytecode, which is then executed by the Python virtual machine. 

```
print("hello, world")
```
Comments are ignored by Python during execution; single line comment start with `#`, while multi line comment start with `"` and must end with the same. 
Other programming languages use curly braces to group code, but Python uses indentation. 

Python has dynamic typing which allows variables to be associated with data type dynamically during the runtime. 
A variable is the name that refers to a value stored in the computer's memory. It is used to store information. We dont need to specify the type of the variable. The values of the variable can change during the execution of the program. 
A valid variable name must start with a letter or an underscore. It variable name can contains letters, digit and underscores. It is case sensitive. Dont use Python keywords and any special symbols.  
Primary data types in Python are:
1. Numeric types
2. Text types
3. Sequence types
4. Mapping types
5. Set types
6. Boolean types
7. Binary types
8. None types


```
name = "duyant"
age = 20

print("Name: ", name)
print("Age: ", age)
```

A string can be defined with single as well as double quotes. 
The `input("")` function takes the input from the user. It also takes an input, which will be printed on the screen before the user's input. This function returns a string value which is inputed by the user. 
Tuple unpacking helps us to relate variables according to their position. 

Local variables are defined inside a function and are only accessible within the function. They are destroyed and their memory is freed after the function has ended. 
While Global variables are defined outside all the functions and can be accessed by all the functions. They are destroyed and memory is freed when the program has ended. 

Python Operations:
1. Arithmatic Operation: It includes addition, subtraction, multiplication, division, floor division, modulus, and exponentiation.
2. Assignment Operation: It is used to assign values to variables and modify them. It included `=`, `+=`, `*=`, `/=`, `//=`, `-=`, `%=`, and `**=`.
3. Comparison Operation: They compare 2 values and return a Boolean. It included `==`, `!=`, `>`, `<`, `<=`, and `>=`.
4. Logical Operation: It is used to combine conditional statements. It included `and`, `or`, and `not`.
5. Identity Operator: It check if 2 variables point to same memory location(object). It included `is` and `is not`. Note that Python internally caches small integers(typically from -5 to 256) as well as small strings.
6. Membership Operation: It tests whether a sequence contains a value. It includes `in` and `not in`. 
7. Bitwise Operation: It operates on binary bits of numbers and perform operations bit by bit at a time. It includes `&`, `|`, `^`, `~`, `<<` and `>>`.

The operation precedence is similar to other languages. Most operators in Python follow the Left-to-Right Associativity except exponent, assignment and not operator, which follow Right-to-Left Associativity. 
Short Circuit evaluation means that Python stops evaluating the logical expression as soon as the result is known. It applies to logical operator `or` and `and`.

A block is a group of code that is executed together. The block is defined by the level of indentation.
A function is a set of statements that can be reused whenever the function is called. A function may or may not return a value. 
```
def func_name():
    # The contents go here
``` 
A loop is a set of statements which are executed a specific number of times.It can be of 2 types i.e. for-loop and while-loop. Loops can be nested; that means a loop can contain one or more loops in its body.

