Running test: tests\arrow_functions_demo.rcc
=== ARROW FUNCTIONS AS PARAMETERS ===

Original array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

--- MAP with arrow functions ---
Doubled (arrow): [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared (arrow): [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
Transformed (x*3+1): [4, 7, 10, 13, 16, 19, 22, 25, 28, 31]

--- FILTER with arrow functions ---
Even numbers: [2, 4, 6, 8, 10]
Greater than 5: [6, 7, 8, 9, 10]
Multiples of 3: [3, 6, 9]

--- REDUCE with arrow functions ---
Sum: 55
Product: 3628800

--- FIND/SOME/EVERY with arrow functions ---
First > 7: 8
Has evens? true
All positive? true

--- METHOD CHAINING with arrow functions ---
Evens * 3, > 10: [12, 18, 24, 30]

=== ASYNC FUNCTIONS ===
Future: [Future: Resolved(User_42)]
User: User_42

=== ASYNC + ARROW FUNCTIONS ===
Processed users: [Processed_1, Processed_2, Processed_3, Processed_4, Processed_5]

=== HIGHER-ORDER FUNCTIONS ===
Original: [1, 2, 3]
Doubled with func type: [2, 4, 6]
Tripled with func type: [3, 6, 9]
Doubled: [2, 4, 6]
Tripled: [3, 6, 9]

=== ALL TESTS COMPLETED ===
null
Running test: tests\async_and_arrays.rcc
=== TESTING ASYNC FUNCTIONS ===
Future: [Future: Resolved(Data fetched!)]
Result after await: Data fetched!

=== TESTING ARRAY METHODS ===
Original array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

--- map() ---
Doubled: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

--- filter() ---
Even numbers: [2, 4, 6, 8, 10]
Greater than 5: [6, 7, 8, 9, 10]

--- reduce() ---
Sum of all numbers: 55
Product of all numbers: 3628800

--- find() ---
First number > 7: 8
First number > 100: null

--- findIndex() ---
Index of 5: 4
Index of > 100: -1

--- some() ---
Has even numbers? true
Has negative numbers? false

--- every() ---
All positive? true
All even? false

=== METHOD CHAINING ===
Even numbers * 3, > 10: [12, 18, 24, 30]

--- Small array composition ---
Original: [1, 2, 3]
Doubled: [2, 4, 6]
Tripled: [3, 6, 9]

=== ALL TESTS COMPLETED ===
null
Running test: tests\comprehensive_stdlib_demo.rcc
Running test: tests\demo_colors.rcc
╔══════════════════════════════════════════════╗
║   DEMOSTRACIÓN DE COLORES EN RACCOON        ║
╔══════════════════════════════════════════════╗

🎨 TIPOS DE DATOS CON COLORES:
─────────────────────────────────────────────

📝 Strings (Verde):
  → Hola Mundo Raccoon

🔢 Números (Amarillo):
  Enteros → 42 100 -5
  Decimales → 3.14 2.71 0.5

✓ Booleanos (Amarillo):
  → true false

∅ Null (Gris):
  → null

📋 Listas (Magenta):
  → [1, 2, 3, 4, 5]
  → [a, b, c]


🎯 OPERACIONES Y RESULTADOS:
─────────────────────────────────────────────

a = 10 │ b = 20
a + b = 30
a * b = 200
a < b = true


✨ COMPARACIÓN:
─────────────────────────────────────────────

👤 Usuario: Alice
   Edad: 30 años
   Estado: Activo


╚══════════════════════════════════════════════╝
║   FIN DE LA DEMOSTRACIÓN                     ║
╚══════════════════════════════════════════════╝
null
Running test: tests\feature_array_destructuring.rcc
a = 1
b = 2
c = 3
Test 2: String Array Destructuring
hello world raccoon
Test 3: Float Array Destructuring
Pi: 3.14
E: 2.71
Sqrt2: 1.41
Test 4: Function Parameter Destructuring
Sum of first two: 30
Values: 100, 200, 300
Test 5: Block Scope Destructuring
First: 7, Second: 14
Test 6: Destructuring in Loops
[0] = 0
[1] = 10
[2] = 20
Test 7: Destructuring in Class Methods
AP-1 processing: 5, 10, 15
Result: 30
First value: 99
Test 8: Destructuring in Conditionals
Conditional values: 50, 60
Test 9: Nested Function Calls
Multiplied: 2, 4, 6
Test 10: Let vs Const Destructuring
Let: 1, 2
Const: 10, 20, 30
Test 11: For-in Loop with Arrays
Item: 100
Item: 200
Item: 300
Test 12: Range with Destructuring
Range values: 1, 2, 3
Test 13: Constructor with Destructuring
Vector created: [1, 2, 3]
Components: 3 items
=== ALL ARRAY DESTRUCTURING TESTS PASSED ===
✅ Basic array destructuring
✅ String/Float arrays
✅ Function parameters
✅ Block scope
✅ Loops
✅ Class methods
✅ Conditionals
✅ Nested calls
✅ Let vs Const
✅ For-in loops
✅ Ranges
✅ Constructors
null
Running test: tests\feature_arrow_functions.rcc
8
14
42
42
true
false
Hello, World!
12
19
true
8
15
All arrow function tests passed!
null
Running test: tests\feature_decorators.rcc
=== DECORATORS FEATURE TEST ===
Test 1: Simple Class Decorator
Value: 42
Test 2: Decorator with Single Argument
Test 3: Decorator with Multiple Arguments
Test 4: Multiple Decorators
Test 5: Decorators with String Arguments
Test 6: Decorators with Float Arguments
Test 7: Decorators with Inheritance
Test 8: Complex Decorator Logic
Size: 5
Test 9: Boolean Argument Decorators
Test 10: Decorator Ordering Test
=== ALL DECORATOR TESTS PASSED ===
✅ Simple decorators
✅ Decorators with arguments
✅ Multiple decorators
✅ String arguments
✅ Float arguments
✅ With inheritance
✅ Complex logic
✅ Boolean arguments
✅ Decorator ordering
null
Running test: tests\feature_object_destructuring.rcc
=== OBJECT DESTRUCTURING FEATURE TEST ===
Test 1: Global Variable Destructuring
x = 10
y = 20
x2 = 100
y2 = 200
Test 2: Block Scope Destructuring
Name: Alice
Age: 30
Test 3: Function Parameter Destructuring
Point(5, 15)
Hello Bob, you are 25 years old
Test 4: Destructuring in Conditionals
Resolution: 800x600
Test 5: Destructuring in While Loop
Loop 0: (0, 0)
Loop 1: (10, 20)
Test 6: Destructuring in Class Methods
Processor-1 processing: x=7, y=14
Combined result: 10
Test 7: Multiple Destructurings
User 1: user1 (user1@test.com)
User 2: user2 (user2@test.com)
Test 8: Destructuring with Function Returns
Created point: (42, 84)
Test 9: Nested Function Calls
Original: (3, 4)
Doubled sum: 14
Test 10: Destructuring in Try-Catch
Try block: Test, 99
=== ALL OBJECT DESTRUCTURING TESTS PASSED ===
✅ Global scope destructuring
✅ Block scope destructuring
✅ Function parameter destructuring
✅ Conditional destructuring
✅ Loop destructuring
✅ Class method destructuring
✅ Multiple destructurings
✅ With function returns
✅ Nested calls
✅ Try-catch blocks
null
Running test: tests\REFERENCE_COMPATIBILITY_TEST.rcc
╔════════════════════════════════════════════╗
║  RACCOON COMPATIBILITY REFERENCE TEST     ║
║  Version: 1.0.0                           ║
║  Date: October 2025                       ║
╚════════════════════════════════════════════╝
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 1: Object Destructuring
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1.1 Basic Object Destructuring
  ✓ x=10, y=20
1.2 Object Destructuring in Functions
  ✓ sum=20
1.3 Object Destructuring in Class Methods
  ✓ result=14
  ✅ Object Destructuring: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 2: Array Destructuring
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2.1 Basic Array Destructuring
  ✓ a=1, b=2, c=3
2.2 String Array Destructuring
  ✓ w1=hello, w2=world
2.3 Array Destructuring in Function Parameters
  ✓ arraySum=60
2.4 Array Destructuring in Constructors
  ✓ magnitude=7
  ✅ Array Destructuring: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 3: Decorators
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3.1 Simple Class Decorator
3.2 Decorator with Arguments
3.3 Multiple Decorators
  ✅ Decorators: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 4: Feature Integration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4.1 Decorators + Object Destructuring
  ✓ Authenticating user: admin
4.2 Decorators + Array Destructuring
  ✓ Processing: min=10, max=100, avg=55
4.3 All Features Combined
  ✓ Querying users with limit 100
  ✓ Batch inserting 10 records of size 1024
  ✓ Total bytes: 10240
  ✅ Feature Integration: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 5: Edge Cases & Stress Tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
5.1 Nested Object Access
  ✓ Location: NYC, USA
5.2 Destructuring in Loops
  ✓ Loop[0]: 0
  ✓ Loop[1]: 10
5.3 Destructuring with Ranges
  ✓ Range values: 1, 2, 3
5.4 Multiple Destructurings
  ✓ P1(1,2), P2(3,4)
5.5 Decorator with Complex Arguments
  ✓ Service call executed
  ✅ Edge Cases: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 6: Backward Compatibility
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
6.1 Regular Classes
  ✓ Old style class value: 100
6.2 Regular Functions
  ✓ Regular function result: 15
6.3 Regular Variables
  ✓ Normal var: 42, const: constant
  ✅ Backward Compatibility: PASSED
╔════════════════════════════════════════════╗
║           COMPATIBILITY REPORT            ║
╚════════════════════════════════════════════╝
✅ SECTION 1: Object Destructuring ......... PASSED
✅ SECTION 2: Array Destructuring .......... PASSED
✅ SECTION 3: Decorators ................... PASSED
✅ SECTION 4: Feature Integration .......... PASSED
✅ SECTION 5: Edge Cases ................... PASSED
✅ SECTION 6: Backward Compatibility ....... PASSED
╔════════════════════════════════════════════╗
║    ALL COMPATIBILITY TESTS PASSED ✅      ║
║                                           ║
║  Object Destructuring:    100% ✅         ║
║  Array Destructuring:     100% ✅         ║
║  Decorators:              100% ✅         ║
║  Feature Integration:     100% ✅         ║
║  Edge Cases:              100% ✅         ║
║  Backward Compatibility:  100% ✅         ║
║                                           ║
║  OVERALL STATUS:          EXCELLENT ✅    ║
╚════════════════════════════════════════════╝
null
Running test: tests\syntax_arrays.rcc
=== ARRAYS TEST ===
Test 1: Array Declaration
Int array length: 5
String array length: 3
Float array length: 3
Bool array length: 3
Test 2: Array Access
First element: 10
Second element: 20
Last element: 50
Test 3: Array Modification
Before: 1
After: 100
Test 4: Empty Array
Empty array length: 0
Test 5: Array with Variables
Array from vars: 1, 2, 3
Test 6: Nested Arrays
Matrix[0][0]: 1
Matrix[1][1]: 4
Matrix[2][0]: 5
Test 7: Array Iteration
Fruit: apple
Fruit: banana
Fruit: cherry
Test 8: Array from Range
Range 1..5 length: 5
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
Test 9: Array Push
Before push: 2
After push: 3
Last element: 3
Test 10: Array Pop
Before pop: 3
Popped value: 30
After pop: 2
Test 11: String as Character Array
String length: 5
First char: h
Last char: o
Test 12: Array Operations
Combined length: 4
Test 13: Arrays in Functions
First: 100
Last: 300
Test 14: Multi-dimensional Arrays
Grid[0][0]: 1
Grid[1][1]: 5
Grid[2][2]: 9
Test 15: Length Property
Empty: 0
After 1 push: 1
After 3 pushes: 3
=== ALL ARRAYS TESTS PASSED ===
✅ Array declaration
✅ Array access
✅ Array modification
✅ Empty arrays
✅ Arrays from variables
✅ Nested arrays
✅ Array iteration
✅ Range creation
✅ Push method
✅ Pop method
✅ String indexing
✅ Array concatenation
✅ Arrays in functions
✅ Multi-dimensional access
✅ Length property
null
Running test: tests\syntax_arrow_functions.rcc
10
30
42
42
75
true
false
Hello, TypeScript
25
11
13
true
false
8
15
15
25
49
103
35
7
12
All TypeScript-style arrow function tests passed!
null
Running test: tests\syntax_bitwise_operators.rcc
8
14
6
-6
20
5
4611686018427387902
8
512
8
14
6
20
5
8
null
Running test: tests\syntax_classes.rcc
=== CLASSES TEST ===
Test 1: Basic Class
Name: Alice
Age: 30
Test 2: Class Methods
Initial: 10
After add(5): 15
Test 3: Multiple Methods
After 2 increments: 2
After 1 decrement: 1
Test 4: String Fields
1984 by Orwell
Pages: 328
Test 5: Boolean Fields
Status: OFF
After toggle: ON
Test 6: Array Fields
Student count: 2
Test 7: Method Return Values
Area: 15
Perimeter: 16
Test 8: Inheritance
Dog name: Rex
Breed: Labrador
Sound: Woof!
Test 9: Methods with Parameters
max(10, 20): 20
min(10, 20): 10
Test 10: Multiple Instances
p1: (0, 0)
p2: (10, 20)
=== ALL CLASSES TESTS PASSED ===
✅ Basic class creation
✅ Class methods
✅ Multiple methods
✅ String fields
✅ Boolean fields
✅ Array fields
✅ Method returns
✅ Inheritance
✅ Static methods
✅ Multiple instances
null
Running test: tests\syntax_control_flow.rcc
=== CONTROL FLOW TEST ===
Test 1: If Statement
x is greater than 5
Test 2: If-Else Statement
Adult
18 or older
Test 3: If-Else Chain
Grade: B
Test 4: Nested If
Positive and greater than 10
Test 5: While Loop
Counter: 0
Counter: 1
Counter: 2
Test 6: While with Break
i: 0
i: 1
i: 2
Test 7: While with Continue
j: 1
j: 2
j: 4
j: 5
Test 8: For-In Loop with Array
Number: 10
Number: 20
Number: 30
Test 9: For-In Loop with Range
Range: 1
Range: 2
Range: 3
Range: 4
Range: 5
Test 10: Nested Loops
(0, 0)
(0, 1)
(1, 0)
(1, 1)
Test 11: Complex Conditions
a < b < c is true
At least one condition is true
a is not greater than b
Test 12: Early Return Pattern
checkValue(-5): negative
checkValue(0): zero
checkValue(10): positive
=== ALL CONTROL FLOW TESTS PASSED ===
✅ If statement
✅ If-else statement
✅ If-else chain
✅ Nested if
✅ While loop
✅ Break statement
✅ Continue statement
✅ For-in with arrays
✅ For-in with ranges
✅ Nested loops
✅ Complex conditions
✅ Early returns
null
Running test: tests\syntax_error_handling.rcc
=== ERROR HANDLING TEST ===
Test 1: Basic Try-Catch
Inside try block
Caught: Basic error
Test 2: Normal Flow
This executes normally
x = 10
Test 3: String Error
Error message: Something went wrong!
Test 4: Try-Catch in Function
Result: error: Function error
Test 5: Nested Try-Catch
Outer try
Inner try
Inner catch: Inner error
After inner try-catch
Test 6: Multiple Statements
Sum: 3
Caught: Error after calculations
Test 7: Try-Catch with Return
10 % 2 = 0
Error: Division by zero
10 % 0 = 0
Test 8: Variable Scope
Status: error
Test 9: Try-Catch in Loop
i = 0
Caught: Error at i=1
i = 2
Test 10: Conditional Throw
Valid age
Invalid: Age cannot be negative
Invalid: Age too high
Test 11: Multiple Throws
Value is valid
Validation error: Negative
Validation error: Zero
Validation error: Too large
Test 12: Error Propagation
caught: Inner function error
=== ALL ERROR HANDLING TESTS PASSED ===
✅ Basic try-catch
✅ Normal flow
✅ String errors
✅ Try-catch in functions
✅ Nested try-catch
✅ Multiple statements
✅ Try-catch with return
✅ Variable scope
✅ Try-catch in loops
✅ Conditional throws
✅ Multiple throws
✅ Error propagation
null
Running test: tests\syntax_functions.rcc
=== FUNCTIONS TEST ===
Test 1: Basic Function
Hello from function!
Test 2: Function with Parameters
5 + 3 = 8
Test 3: Multiple Parameters
Name: Alice, Age: 25, City: NYC
Test 4: Return Values
6 * 7 = 42
20 / 4 = 5
Test 5: Default Parameters
Hello, Alice!
Hello, Guest!
Test 6: Recursive Function
factorial(5) = 120
Test 7: Function Composition
quadruple(5) = 20
Test 8: String Return Functions
Full name: John Doe
Shouted: HELLO
Test 9: Boolean Return Functions
isEven(4): true
isEven(7): false
isPositive(-5): false
isPositive(10): true
Test 10: Array Parameters
Sum of array: 15
Test 11: Array Return
Range length: 5
Test 12: Nested Calls
process(5) = 8
Test 13: Float Functions
average(5.0, 10.0) = 7.5
Test 14: Void Functions
LOG: Test message
=== ALL FUNCTIONS TESTS PASSED ===
✅ Basic functions
✅ Parameters
✅ Multiple parameters
✅ Return values
✅ Default parameters
✅ Recursion
✅ Function composition
✅ String returns
✅ Boolean returns
✅ Array parameters
✅ Array returns
✅ Nested calls
✅ Float functions
✅ Void functions
null
Running test: tests\syntax_interfaces.rcc
=== INTERFACES TEST ===
Test 1: Interface Definition
Name: Alice
Test 2: Interface with Methods
Drawing circle with radius 5
Test 3: Multiple Members
Area: 20
Test 4: Multiple Interfaces
ID: 1
Product: Laptop
Test 5: Interface with Boolean
Initial: false
After toggle: true
Test 6: Multiple Methods
Counter value: 2
Test 7: Return Types
Result: 30
Test 8: Empty Interface
Marked value: 42
Test 9: String Methods
Formatted: 15/3/2024
Test 10: Implementation Chain
Bob, 35, Developer
=== ALL INTERFACES TESTS PASSED ===
✅ Basic interface
✅ Interface with methods
✅ Multiple members
✅ Multiple interfaces
✅ Boolean interfaces
✅ Multiple methods
✅ Return types
✅ Empty interfaces
✅ String methods
✅ Implementation chains
null
Running test: tests\syntax_interfaces_simple.rcc
=== INTERFACES TEST (SIMPLIFIED) ===
Test 1: Basic Interface
✓ Interface Point defined
Test 2: Multiple Interfaces
✓ Multiple interfaces defined
Test 3: Interface with Different Types
✓ Interface with multiple types defined
Test 4: Classes Following Interface Structure
Area: 50
=== ALL INTERFACE TESTS PASSED ===
✅ Basic interface definitions
✅ Multiple interfaces
✅ Interfaces with multiple types
✅ Classes following interface structure
Note: Interfaces only support properties, not methods
Note: 'implements' keyword is not currently enforced
null
Running test: tests\syntax_operators.rcc
=== OPERATORS TEST ===
Test 1: Arithmetic Operators
Addition: 13
Subtraction: 7
Multiplication: 30
Division: 3.3333333333333335
Modulo: 1
Test 2: Comparison Operators
Equal: true
Not equal: true
Less than: true
Greater than: true
Less or equal: true
Greater or equal: true
Test 3: Logical Operators
AND (true && true): true
AND (true && false): false
OR (false || true): true
OR (false || false): false
NOT (!true): false
NOT (!false): true
Test 4: Assignment Operators
Initial: 10
After +5: 15
After -3: 12
After *2: 24
After -4: 20
Test 5: Compound Assignment
num += 10: 110
num -= 5: 105
num *= 2: 210
num -= 10: 200
Test 6: Unary Operators
Negation: -42
Logical NOT: false
Test 7: String Concatenation
Concatenation: Hello World
Mixed: Raccoon v1
Test 8: Operator Precedence
2 + 3 * 4 = 14
(2 + 3) * 4 = 20
10 - 2 + 3 = 11
Test 9: Boolean Expressions
Is adult (25 >= 18): true
Passed (85 in range): true
Can edit: true
Test 10: Null Comparison
Is null: true
Is not null: true
=== ALL OPERATORS TESTS PASSED ===
✅ Arithmetic operators
✅ Comparison operators
✅ Logical operators
✅ Assignment operators
✅ Compound assignment
✅ Unary operators
✅ String concatenation
✅ Operator precedence
✅ Boolean expressions
✅ Null comparison
null
Running test: tests\syntax_primitive_types.rcc
=== PRIMITIVE TYPES TEST ===
Test 1: Integer Type
Positive: 42
Negative: -100
Zero: 0
Max: 2147483647
Min: -2147483648
Test 2: Float Type
Pi: 3.14
Negative: -2.71
Zero: 0
E: 2.71828
Test 3: String Type
String 1: Hello World
String 2: Raccoon
Empty: ''
Greeting: ¡Hola!
Test 4: Boolean Type
True: true
False: false
Constant: true
Test 5: Null Type
Null value created
Assigned value: 42
Test 6: Type Conversions
Int to Float: 10
Int to String: 10
Test 7: String Operations
Upper: HELLO
Lower: hello
Split length: 3
Test 8: Type Checking
typeof num: int
typeof msg: str
Test 9: Constants
PI: 3.14159
Version: 1
App: Raccoon
Test 10: Mixed Type Operations
Int + Float: 7.5
Number: 42
=== ALL PRIMITIVE TYPES TESTS PASSED ===
✅ Integer type
✅ Float type
✅ String type
✅ Boolean type
✅ Null type
✅ Type conversions
✅ String operations
✅ Type checking
✅ Constants
✅ Mixed operations
null
Running test: tests\syntax_scopes.rcc
=== SCOPES TEST ===
Test 1: Global Scope
Global: 100
Test 2: Block Scope
Inside block: 2
Access outer: 1
Outside block: 1
Test 3: Function Scope
Inside function: 20
Access outer: 10
Outside function: 10
Test 4: Variable Shadowing
Outer x: 1
Inner x: 2
Outer x again: 1
Test 5: Loop Scope
Loop iteration 0: 0
Loop iteration 1: 10
Loop iteration 2: 20
After loop: 5
Test 6: If Statement Scope
Inside if: 42
After if block
Test 7: Nested Blocks
Level 3: 3
Access level 2: 2
Access level 1: 1
Level 2: 2
Level 1: 1
Test 8: Reassignment
Initial: 10
After reassignment: 20
After calculation: 25
Test 9: Constants
Constant value: 100
PI: 3.14159
Test 10: For-In Scope
Number: 10
Number: 20
Number: 30
After for-in loop
Test 11: Class Field Scope
Counter: 1
Test 12: Try-Catch Scope
Try block: 100
Catch block: caught
Test 13: Multiple Variables
a=1, b=2, c=3
Test 14: Variable Lifetime
First call: 1
Second call: 1
Test 15: Scope Chain
Access all: 1, 2, 3
=== ALL SCOPES TESTS PASSED ===
✅ Global scope
✅ Block scope
✅ Function scope
✅ Variable shadowing
✅ Loop scope
✅ If statement scope
✅ Nested blocks
✅ Reassignment
✅ Constants
✅ For-in scope
✅ Class field scope
✅ Try-catch scope
✅ Multiple variables
✅ Variable lifetime
✅ Scope chain
null
Running test: tests\syntax_static_methods.rcc
=== STATIC METHODS TEST ===
Test 1: Basic Static Method
max(10, 20): 20
min(10, 20): 10
Test 2: Multiple Parameters
add(1, 2, 3): 6
multiply(2, 3, 4): 24
Test 3: String Return
HelloWorld
xxx
Test 4: Mix Static and Instance
Counter value: 1
Test 5: Factory Pattern
Origin: (0, 0)
From array: (10, 20)
=== ALL STATIC METHOD TESTS PASSED ===
✅ Basic static methods
✅ Multiple parameters
✅ String return types
✅ Mixed static and instance
✅ Factory pattern
null
Running test: tests\syntax_templates.rcc
=== TEMPLATE STRINGS TEST ===
Test 1: Basic Template
Hello, World!
Test 2: Integer Interpolation
I am 25 years old
Test 3: Float Interpolation
Price: $19.99
Test 4: Boolean Interpolation
Status: true
Test 5: Multiple Variables
Name: John Doe, Age: 30
Test 6: Expression Interpolation
5 + 3 = 8
Test 7: Templates in Functions
Hello Alice, you are 28 years old
Test 8: Object Properties
My name is Bob and I'm 35
Test 9: Array Properties
Array has 5 items
Test 10: Comparison Results
x < y is true
Test 11: Complex Expressions
Sum of 5 and 10 is 15
Test 12: Function Call Interpolation
Double of 7 is 14
Test 13: Template Concatenation
Hello World
Test 14: Empty Templates
Empty string length: 0
Test 15: Templates in Conditionals
Grade: B (85)
=== ALL TEMPLATE STRINGS TESTS PASSED ===
✅ Basic templates
✅ Integer interpolation
✅ Float interpolation
✅ Boolean interpolation
✅ Multiple variables
✅ Expressions
✅ Function templates
✅ Object properties
✅ Array properties
✅ Comparisons
✅ Complex expressions
✅ Function calls
✅ Concatenation
✅ Empty templates
✅ Conditional templates
null
Running test: tests\test_array_methods.rcc
Original array:
Doubled:
Evens:
Sum:
ForEach:
  -
  -
  -
  -
  -
First > 3:
Index of first > 3:
Has > 10:
Has < 3:
All positive:
All > 10:
All tests completed!
null
Running test: tests\test_arrow.rcc
[2, 4, 6]
null
Running test: tests\test_arrow2.rcc
[2, 4, 6]
null
Running test: tests\test_arrow_complete.rcc
=== ARROW FUNCTIONS TEST ===

1. Arrow function with map:
Doubled: [2, 4, 6, 8, 10]

2. Arrow function with filter:
Even numbers: [2, 4]

3. Arrow function with reduce:
Sum: 15

4. Arrow function with type annotations:
Squared: [1, 4, 9, 16, 25]

5. Method chaining:
Filtered > 2, then * 10: [30, 40, 50]

6. Arrow function as variable:
Tripled: [3, 6, 9, 12, 15]

=== ALL TESTS PASSED ===
null
Running test: tests\test_arrow_notype.rcc
14
null
Running test: tests\test_arrow_simple.rcc
8
null
Running test: tests\test_arrow_single.rcc
14
null
Running test: tests\test_assign_error.rcc
Running test: tests\test_builtins.rcc
Running test: tests\test_classes_advanced.rcc
🦝 === ADVANCED CLASSES TEST ===
✅ Test 1: Static Methods
  max(10, 20): 20
  min(10, 20): 10
  abs(-15): 15
  abs(15): 15
✅ Test 2: Getters and Setters
  Celsius: 25
  Fahrenheit: 77
  Kelvin: 298.15
  After setting to 68°F:
  Celsius: 20
✅ Test 3: Method Overriding
  Circle with radius 5
  Area: 78.53975
  Rectangle 4x6
  Area: 24
✅ Test 4: Encapsulation Pattern
  Initial: 0/3
  After increment: 1
  After 2 more increments: 3
  Can increment more: false
  After reset: 0
✅ Test 5: Complex Constructor
  User: john_doe
  Email: john@example.com
  Status: Active (0 logins)
  After 3 logins: Active (3 logins)
  After deactivation: Inactive
✅ Test 6: Two-Level Inheritance
  Car: Toyota (4 doors)
  Brand: Toyota
  Doors: 4
✅ Test 7: Complex State Management
  Cart empty: true
  Items in cart: 3
  Total: $12.25
✅ Test 8: Methods with Multiple Returns
  Empty string: Error: Value is empty
  'ab': Error: Value too short
  'hello': Valid
  Long string: Error: Value too long
✅ Test 9: Instance Type Checking
  Animal name: Generic
  Cat name: Whiskers
  Cat says: Meow!
✅ Test 10: Multiple Instances Stress Test
  Created 5 points
  Point 0: (0, 0)
  Point 1: (3, 4)
  Point 2: (5, 12)
🎉 === ALL ADVANCED TESTS PASSED ===
✅ Static methods
✅ Getters and setters
✅ Method overriding in inheritance
✅ Encapsulation patterns
✅ Complex constructor logic
✅ Two-level inheritance
✅ Complex state management
✅ Multiple return points
✅ Instance type checking
✅ Multiple instances stress test
🦝 Raccoon Classes: ALL ADVANCED FEATURES WORKING!
null
Running test: tests\test_classes_comprehensive.rcc
🦝 === COMPREHENSIVE CLASSES TEST ===
✅ Test 1: Basic Class with Constructor
  Name: Alice
  Age: 30
✅ Test 2: Class with Methods
  Initial value: 10
  After add(5): 15
  After multiply(2): 30
  After subtract(10): 20
  After reset: 0
✅ Test 3: Class with Different Data Types
  Laptop - $999.99 (Stock: 5)
  Total value: $4999.95
  In stock: true
  After selling 3: Laptop - $999.99 (Stock: 2)
  After selling 2 more: Laptop - $999.99 (Stock: 0)
  In stock: false
✅ Test 4: Class with Array Properties
  Classroom: Math 101
  Students: 3
  Average grade: 91.33333333333333
✅ Test 5: Class Inheritance
  Rex is 3 years old
  Rex is a Labrador
  Sound: Woof! Woof!
✅ Test 6: Multiple Instances Independence
  Counter A: 3
  Counter B: 1
✅ Test 7: Complex Methods with Logic
  Account: ACC001
  Initial balance: $1000
  After deposit $500: $1500
  After withdraw $200: $1300
✅ Test 8: Nested Method Calls
  Initial: 5
  After add(3) then multiply(2): 16
✅ Test 9: Class with String Operations
  Original: Hello Raccoon
  Uppercase: HELLO RACCOON
  Lowercase: hello raccoon
  Length: 13
  Is empty: false
✅ Test 10: Class with Object Composition
  John Doe (ID: 1001)
  John Doe (ID: 1001) - 123 Main St, Springfield 12345
🎉 === ALL TESTS PASSED ===
✅ Basic class with constructor
✅ Class with methods
✅ Different data types (str, int, float, bool)
✅ Array properties
✅ Class inheritance (extends, super)
✅ Multiple instances independence
✅ Complex methods with logic
✅ Nested method calls
✅ String operations in classes
✅ Object composition
🦝 Raccoon Classes: FULLY FUNCTIONAL!
null
Running test: tests\test_debugging_features.rcc
=== TEST DE DEBUGGING Y COLORES ===

Test 1: print() con colores para diferentes tipos
Strings en verde: Hello World
Números en amarillo: 42 3.14
Booleanos en amarillo: true false
Null en gris: null
Lista en magenta: [1, 2, 3]

Test 2: función println()
Primera línea
Segunda línea
Tercera línea

Test 3: Mezcla de tipos en una sola llamada
Usuario: Bob Edad: 25 Activo: true Score: 95.5

Test 4: Resultados de operaciones
x = 10 y = 5
x + y = 15
x * y = 50
x > y = true

Test 5: Estructuras de datos
Lista de números: [10, 20, 30, 40, 50]
Lista mixta: [1, two, 3, four]

Test 6: Funciones
Resultado: Hello, World

=== TODOS LOS TESTS COMPLETADOS ===
null
Running test: tests\test_decorators_advanced.rcc
🦝 === ADVANCED DECORATORS TEST ===
✅ Test 1: Method Decorators
  Calculator value: 15
✅ Test 2: Timing Decorator Pattern
  [TIMER END] Result: 5050
✅ Test 3: API Route Decorator Pattern
  List of users
  User created
✅ Test 4: Database Entity Decorator
  Product: Laptop - $999.99
✅ Test 5: Validation Decorator
  Age set to 25
  Name set to John
✅ Test 6: Memoization Pattern
  Result 1: 25
  Result 2: 169
✅ Test 7: Singleton Pattern
  Connected to: localhost:5432
✅ Test 8: Rate Limiting Decorator
  API response from /api/data
✅ Test 9: Transaction Decorator
  Transferred $100
  [TRANSACTION] Commit
✅ Test 10: Error Handling Decorator
  Success
✅ Test 11: AOP Pattern - Before/After
  [BEFORE] Executing...
  Processed: important data
  [AFTER] Done
✅ Test 12: Feature Flag Decorator
  Displaying new UI
✅ Test 13: Dependency Injection
  UserCtrl handling request
✅ Test 14: Event Emitter Pattern
  Created user: alice
✅ Test 15: Multi-Decorator Composition
  Executed: delete_database
🎉 === ALL ADVANCED DECORATOR TESTS PASSED ===
✅ Method decorators
✅ Timing decorator pattern
✅ API route decorator
✅ Database entity decorator
✅ Validation decorator
✅ Memoization pattern
✅ Singleton pattern
✅ Rate limiting
✅ Transaction decorator
✅ Error handling
✅ AOP pattern
✅ Feature flags
✅ Dependency injection
✅ Event emitters
✅ Multi-decorator composition
🦝 Raccoon Advanced Decorators: PRODUCTION READY!
null
Running test: tests\test_decorators_comprehensive.rcc
🦝 === COMPREHENSIVE DECORATORS TEST ===
✅ Test 1: Basic Function Decorator
  Result: Hello, Alice
✅ Test 2: Function Decorator with Arguments
  Sum: 8
✅ Test 3: Class Decorator
  User greeting: Hello, Bob
✅ Test 4: Multiple Decorators on Function
  Result: Function with multiple decorators
✅ Test 5: Decorator with Multiple Arguments
  Percentage: 75%
✅ Test 6: Class Decorator with Arguments
  Charlie is 30 years old
✅ Test 7: Performance Tracking Decorator
Running test: tests\test_error_display.rcc
Iniciando test...
Running test: tests\test_error_positions.rcc
Running test: tests\test_explicit_generics.rcc
42
100
400
25
Hello
All tests passed!
null
Running test: tests\test_func_type.rcc
=== FUNC TYPE TESTS ===

[ 1 ] Basic func type assignment
  Result: Hello, Alice

[ 2 ] func as function parameter
  square(5) = 25
  double(5) = 10

[ 3 ] Arrays of func
  Applying operations to 3:
    operations[0](3) = 9
    operations[1](3) = 6

[ 4 ] func with arrow functions
  add(10, 5) = 15
  subtract(10, 5) = 5

[ 5 ] func returning func
  addOp(4, 3) = 7
  mulOp(4, 3) = 12

[ 6 ] func with different return types
  toString(42) = 42
  toBool(5) = true
  toBool(-1) = false

[ 7 ] func with higher-order array methods
  Original: [1, 2, 3, 4, 5]
  Mapped (increment): [2, 3, 4, 5, 6]
  Filtered (isEven): [2, 4]
  Reduced (sum): 15

[ 8 ] Multiple func variables
  addFunc(10, 5) = 15
  subFunc(10, 5) = 5
  mulFunc(10, 5) = 50

[ 9 ] func with async functions
  Async result: Data_123

==================================================
✅ ALL FUNC TYPE TESTS PASSED
==================================================

The 'func' type works as a generic function type!
It can represent any function signature dynamically.
null
Running test: tests\test_generics.rcc
=== GENERICS SYSTEM TEST ===
Test 1: Generic Box<T>
Int box: 42
String box: Hello
Updated int box: 100
Test 2: Generic Pair<K, V>
Pair: age = 25
Test 3: Generic with Constraint
Container size: 3
First item: 10
Test 4: Optional<T> Type
Some has value: true
None has value: false
Some value: 42
None default: 0
Test 5: Result<T> Type
Good result success: true
Good result value: 5
Bad result success: false
Bad result error: Division by zero
Test 6: Stack<T>
Top: third
Pop: third
Pop: second
Is empty: false
=== ALL GENERIC TESTS PASSED ===
null
Running test: tests\test_generics_simple.rcc
=== GENERICS SYSTEM TEST (Simplified) ===
Test 1: Generic Box<T>
Int box: 42
String box: Hello
Updated int box: 100
Test 2: Generic Pair<K, V>
Pair: age = 25
Test 3: Generic Container<T>
Container size: 3
First item: 10
Test 4: Optional<T> Type
Some has value: true
None has value: false
Some value: 42
None default: 0
Test 5: Stack<T>
Top: third
Pop: third
Pop: second
Is empty: false
=== ALL GENERIC TESTS PASSED ===
null
Running test: tests\test_generics_vs_bitwise.rcc
42
32
10
20
All tests passed!
null
Running test: tests\test_http_client.rcc
Running test: tests\test_http_debug.rcc
Running test: tests\test_http_final.rcc
Running test: tests\test_http_minimal.rcc
Running test: tests\test_http_net.rcc
Running test: tests\test_http_simple.rcc
Testing HTTP with default import...
Response: { statusText: OK, headers: Map { cache-control: public, max-age=0, must-revalidate, content-length: 17874, last-modified: Wed, 24 Sep 2025 13:19:34 GMT, x-vercel-id: sfo1::6qcmp-1761021645159-cb922107bc39, content-disposition: inline, strict-transport-security: max-age=63072000, accept-ranges: bytes, access-control-allow-origin: *, x-vercel-cache: HIT, age: 2301670, date: Tue, 21 Oct 2025 04:40:45 GMT, etag: "a096f8b2a3cc43cd18ec5f9eb5ebd7ba", server: Vercel, content-type: text/html; charset=utf-8 }, url: https://coatl.in/, body: null, status: 200 }

Success! Default import working!
null
Running test: tests\test_import_default.rcc
Testing default import...
Http: class Http
null
Running test: tests\test_index_access_assignment.rcc
Testing index access and assignment for objects...

Test 1: Reading properties with index notation
  config["api-key"]: secret123
  config["base-url"]: https://api.example.com
  config["timeout"]: 5000
  ✓ Index access works!

Test 2: Assigning properties with index notation
  After assignment:
    config["api-key"]: new-secret-456
    config["version"]: v2.0
    config["timeout"]: 10000
  ✓ Index assignment works!

Test 3: Mixed access patterns
  user.name: Carlos
  user["user-id"]: 12345
  user.email: carlos@example.com

  After updates:
  user.name: Carlos López
  user["user-id"]: 99999
  user.email: new@example.com
  ✓ Mixed access works!

Test 4: Dynamic property access
  data[propName]: value2
  After update: updated-value2
  ✓ Dynamic property access works!

All tests passed! Index access and assignment work correctly.
null
Running test: tests\test_interface.rcc
10
null
Running test: tests\test_map_complete.rcc
=== Map<K,V> Implementation Tests ===

[ 1 ] Basic Map<str, int> operations:
  Set 3 values
  Get 'one': 1
  Get 'two': 2
  Has 'one': true
  Has 'missing': false
  Size: 3
  ✓ Test 1 passed

[ 2 ] Map delete operation:
  Initial size: 3
  Deleted 'b': true
  Size after delete: 2
  Has 'b': false
  Deleted non-existent 'xyz': false
  ✓ Test 2 passed

[ 3 ] Map clear operation:
  Size before clear: 3
  Size after clear: 0
  Has 'x': false
  ✓ Test 3 passed

[ 4 ] Map with float values:
  pi = 3.14159
  ✓ Test 4 passed

[ 5 ] Map<int, str> with int keys:
  1 -> one
  2 -> two
  Size: 3
  ✓ Test 5 passed

[ 6 ] Map overwrites existing values:
  Initial value: 100
  After overwrite: 200
  Size (should be 1): 1
  ✓ Test 6 passed

[ 7 ] Get non-existent key returns null:
  Non-existent key returns null ✓
  ✓ Test 7 passed

[ 8 ] Map toStr() method:
  Map as string: Map { a: 1, b: 2 }
  ✓ Test 8 passed

[ 9 ] Map with typed integers:
  i32 value: 12345
  u64 value: 999999
  ✓ Test 9 passed

[ 10 ] Null coalescing with Map.get():
  Existing key (with ??): 123
  Missing key (with ??): -1
  ✓ Test 10 passed

[ 11 ] Complex operation sequence:
  Added 5 elements, size: 5
  Deleted 2 elements, size: 3
  Added 1 more element, size: 4
  Has 'a': true
  Has 'b': false
  Has 'f': true
  ✓ Test 11 passed

[ 12 ] Map with float types:
  f32 size: 2
  f64 size: 2
  ✓ Test 12 passed

==================================================
✅ ALL MAP TESTS PASSED SUCCESSFULLY!
==================================================

Types tested:
  • Map<str, int>
  • Map<str, float>
  • Map<int, str>
  • Map<str, i32>
  • Map<str, u64>
  • Map<str, f32>
  • Map<str, f64>

Operations tested:
  • set(key, value)
  • get(key) -> value?
  • has(key) -> bool
  • delete(key) -> bool
  • clear()
  • size() -> int
  • toStr() -> str

🎉 Map<K,V> implementation working perfectly!
null
Running test: tests\test_map_i32.rcc
Creating Map<str, int>...
Success with int!
Creating Map<str, i32>...
Success with i32!
null
Running test: tests\test_map_minimal.rcc
null
Running test: tests\test_map_no_main.rcc
Testing Map basic operations

Map created
Value set
v1: 1
v2: 2
v3: 3
Has 'one': true
Has 'missing': false
Size: 3
Deleted 'two': true
Size after delete: 2
Size after clear: 0

✓ All Map tests passed!
null
Running test: tests\test_modules.rcc
Running test: tests\test_named_args.rcc
=== Test de argumentos nombrados ===

Llamada posicional:
Name: Alice
Age: 30
City: Madrid

Llamada con argumentos nombrados:
Name: Bob
Age: 25
City: Barcelona

Llamada mixta:
Name: Charlie
Age: 35
City: Valencia

=== Test completado ===
null
Running test: tests\test_named_args_comprehensive.rcc
=== Test Completo de Argumentos Nombrados ===

Test 1: Argumentos nombrados en orden diferente
Host: localhost Port: 8080 SSL: true
Host: example.com Port: 3000 SSL: false

Test 2: Valores por defecto
Name: Alice Age: 18 Active: true
Name: Bob Age: 25 Active: true
Name: Charlie Age: 18 Active: false
Name: Diana Age: 30 Active: false

Test 3: Mezcla de posicionales y nombrados
From: Madrid To: Barcelona Date: 2024-01-15 Class: economy
From: Madrid To: Barcelona Date: 2024-01-15 Class: business
From: Madrid To: Barcelona Date: 2024-01-15 Class: economy

Test 4: Funciones con retorno
5 + 3 = 8
5 * 3 = 15
5 * 3 = 15

Test 5: Diferentes tipos de datos
Message 1: HELLO HELLO HELLO 
Message 2: world world 

Test 6: Solo argumentos nombrados
Debug: true Verbose: false LogLevel: info
Debug: false Verbose: true LogLevel: debug

=== Todos los tests completados exitosamente ===
null
Running test: tests\test_net.rcc
Running test: tests\test_new_types.rcc
=== Tipos Enteros con Signo ===
i8:  -128
i16: -32768
i32: -2147483648
i64: -9223372036854776000

=== Tipos Enteros sin Signo ===
u8:  255
u16: 65535
u32: 4294967295
u64: 9223372036854776000

=== Tipos Punto Flotante ===
f32:     3.14159
f64:     2.718281828459045
decimal: 123.456789

=== Conversiones Automáticas ===
i8 -> i64:     -128
f32 -> f64:    3.14159
i32 -> decimal: -2147483648

=== Resultados de Funciones ===
addI32(10, 20):     30
multiplyF64(3.14, 2.0): 6.28
Decimal: 123.456789

=== Arrays con Nuevos Tipos ===
i32[]: [1, 2, 3, 4, 5]
f64[]: [1.1, 2.2, 3.3, 4.4, 5.5]
decimal[]: [10.5, 20.75, 30.25]

✅ Todos los tipos funcionan correctamente!
null
Running test: tests\test_no_conflict_generics_bitwise.rcc
16
64
8
6
40
10
All tests passed!
null
Running test: tests\test_null_safety_operators.rcc
====================================
NULL SAFETY OPERATORS TEST SUITE
====================================
✓ Test 1 passed: Null coalescing operator works
✓ Test 2 passed: Chained null coalescing works
⊘ Test 3 skipped: Inline object types not yet supported
⊘ Test 4 skipped: Inline object types not yet supported
✓ Test 5 passed: Null assertion on non-null value works
✓ Test 6 passed: Basic ternary operator works
✓ Test 7 passed: Nested ternary operators work
✓ Test 8 passed: Prefix increment works
✓ Test 9 passed: Postfix increment works
✓ Test 10 passed: Prefix decrement works
✓ Test 11 passed: Postfix decrement works
✓ Test 12 passed: Increment in for loop works
✓ Test 13 passed: Combined operators work
✓ Test 14 passed: Ternary in complex expression works
✓ Test 15 passed: Float increment/decrement works
====================================
TEST SUITE COMPLETE
====================================
null
Running test: tests\test_null_safety_simple.rcc
=== Test 1: Null Coalescing ===
default
=== Test 2: Conditional ===
adult
=== Test 3: Prefix Increment ===
6
6
=== Test 4: Postfix Increment ===
6
5
=== Test 5: Null Assertion ===
hello
null
Running test: tests\test_object_string_keys.rcc
Testing string literals as object keys...
✓ Created object with string literal keys
  Content-Type: application/json
  Authorization: Bearer token123
  X-Custom-Header: custom value

✓ Created object with mixed key types
  name: Carlos
  full-name: Carlos López
  age: 25
  user-id: 12345

✓ Created nested object with string keys
  API base URL: https://api.example.com
  API timeout: 5000
  Content-Type: application/json

All tests passed! String literals as object keys work correctly.
null
Running test: tests\test_raccoon_modules.rcc
null
Running test: tests\test_simple_array.rcc
[2, 4, 6]
Running test: tests\test_simple_modules.rcc
Test 1: print simple
Test 2: print con múltiples args 123 456
Running test: tests\test_stdlib_demo.rcc
null
Running test: tests\test_stdlib_math.rcc
Running test: tests\test_std_library.rcc
Running test: tests\test_sugar_types.rcc
=== Sugar Types Test ===

✓ int se asigna correctamente a i8, i16, i32, i64, u8, u16, u32, u64
✓ float se asigna correctamente a f32, f64, decimal
✓ Operaciones aritméticas funcionan: 1010, 8.53452
Valor i32: 42
Valor f64: 3.14159
✓ Conversión widening automática: i8 -> i32 -> i64 -> f64 = 50
✓ Arrays tipados: i8[], i32[], f64[] funcionan correctamente

✅ Todos los sugar types y conversiones funcionan perfectamente!
null
Running test: tests\test_toStr.rcc
42
Success!
null
Running test: tests\test_typescript_style_modules.rcc
╔════════════════════════════════════════════════════════╗
║   TEST: Sistema de Módulos Estilo TypeScript         ║
╚════════════════════════════════════════════════════════╝

📦 TEST 1: Named Imports
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running test: tests\test_types_edge_cases.rcc
=== EDGE CASES: Sistema de Tipos ===

[ 1 ] Valores límite de tipos enteros con signo:
  ✓ i8:  -128 a 127
  ✓ i16: -32768 a 32767
  ✓ i32: -2147483648 a 2147483647
  ✓ i64: -9223372036854776000 a 9223372036854776000

[ 2 ] Valores límite de tipos sin signo:
  ✓ u8:  0 a 255
  ✓ u16: 0 a 65535
  ✓ u32: 0 a 4294967295
  ✓ u64: 0 a 9223372036854776000

[ 3 ] Cadenas de conversión widening:
  ✓ i8 -> i16 -> i32 -> i64 -> f64 -> decimal: 42
  ✓ u8 -> u16 -> u32 -> u64 -> f64: 200
  ✓ u8 -> i16 -> i64 -> decimal: 100

[ 4 ] Operaciones aritméticas entre diferentes tipos:
  ✓ i8(10) + i32(20) = i32(30)
  ✓ i16(5) * i64(15) = i64(75)
  ✓ u8(8) - u32(4) = u32(4)
  ✓ f32(3.5) * f64(2.0) = f64(7)
  ✓ i32(100) + f32(2.5) = f32(102.5)

[ 5 ] Operaciones bitwise entre tipos enteros:
  ✓ i8(15) & i32(240) = i32(0)
  ✓ u8(12) | u16(10) = u16(14)
  ✓ i16(5) ^ i32(3) = i32(6)
  ✓ i8(4) << i32(2) = i32(16)
  ✓ i32(16) >> i8(2) = i32(4)

[ 6 ] Funciones con múltiples tipos de parámetros:
  ✓ i8: 50
  ✓ i32: 5000
  ✓ f64: 50.5
  ✓ decimal: 50.123456
  ✓ Sugar type en i8: i8: 25
  ✓ Sugar type en i32: i32: 2500
  ✓ Sugar type en f64: f64: 25.5

[ 7 ] Arrays con diferentes tipos numéricos:
  ✓ i8[]:  [1, 2, 3, 4, 5]
  ✓ i32[]: [100, 200, 300]
  ✓ u8[]:  [255, 128, 64]
  ✓ f32[]: [1.1, 2.2, 3.3]
  ✓ f64[]: [10.5, 20.5, 30.5]

[ 8 ] Operaciones con cero y números negativos:
  ✓ i8(0) + i8(-128) = -128
  ✓ i32(-1000000) + i32(1000000) = 0
  ✓ f64(-3.14159) * -1 = 3.14159
  ✓ Negación unaria: -42 = -42

[ 9 ] Conversiones explícitas entre tipos:
  Original i32: 256
  ✓ toI8():  127
  ✓ toI16(): 256
  ✓ toI64(): 256
  ✓ toF32(): 256
  ✓ toF64(): 256
  Original f64: 123.456
  ✓ toInt():     123
  ✓ toDecimal(): 123.456

[ 10 ] Comparaciones entre diferentes tipos:
  ✓ i8(10) == i32(10): true
  ✓ u8(50) == i16(50): true
  ✓ f32(3.14) == f64(3.14): true
  ✓ i32(100) < i64(200): true

[ 11 ] Expresiones complejas con múltiples tipos:
  ✓ i8(2) + i16(3) * i32(4) - i64(5) = i64(9)
  ✓ u8(10) * u16(20) + u32(30) = u32(230)
  ✓ f32(2.5) * f64(3.0) + decimal(1.5) = decimal(9)

[ 12 ] Operaciones con arrays de tipos mixtos:
  ✓ Array i8 length: [1, 2, 3]
  ✓ Array i32 length: [10, 20, 30]
  ✓ Array f64 length: [1.5, 2.5, 3.5]
  ✓ Después de push: i8[] = [1, 2, 3, 4]
  ✓ Después de push: i32[] = [10, 20, 30, 40]
  ✓ Después de push: f64[] = [1.5, 2.5, 3.5, 4.5]

[ 13 ] Valores decimales de alta precisión:
  ✓ decimal(0.1) + decimal(0.2) = 0.30000000000000004
  ✓ Suma de alta precisión: 999999999.1234568

[ 14 ] Funciones recursivas con tipos específicos:
Running test: arrow_functions_demo.rcc
Running test: async_and_arrays.rcc
Running test: comprehensive_stdlib_demo.rcc
Running test: demo_colors.rcc
Running test: feature_array_destructuring.rcc
Running test: feature_arrow_functions.rcc
Running test: feature_decorators.rcc
Running test: feature_object_destructuring.rcc
Running test: REFERENCE_COMPATIBILITY_TEST.rcc
Running test: syntax_arrays.rcc
Running test: syntax_arrow_functions.rcc
Running test: syntax_bitwise_operators.rcc
Running test: syntax_classes.rcc
Running test: syntax_control_flow.rcc
Running test: syntax_error_handling.rcc
Running test: syntax_functions.rcc
Running test: syntax_interfaces.rcc
Running test: syntax_interfaces_simple.rcc
Running test: syntax_operators.rcc
Running test: syntax_primitive_types.rcc
Running test: syntax_scopes.rcc
Running test: syntax_static_methods.rcc
Running test: syntax_templates.rcc
Running test: test_array_methods.rcc
Running test: test_arrow.rcc
Running test: .\tests\arrow_functions_demo.rcc
=== ARROW FUNCTIONS AS PARAMETERS ===

Original array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

--- MAP with arrow functions ---
Doubled (arrow): [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared (arrow): [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
Transformed (x*3+1): [4, 7, 10, 13, 16, 19, 22, 25, 28, 31]

--- FILTER with arrow functions ---
Even numbers: [2, 4, 6, 8, 10]
Greater than 5: [6, 7, 8, 9, 10]
Multiples of 3: [3, 6, 9]

--- REDUCE with arrow functions ---
Sum: 55
Product: 3628800

--- FIND/SOME/EVERY with arrow functions ---
First > 7: 8
Has evens? true
All positive? true

--- METHOD CHAINING with arrow functions ---
Evens * 3, > 10: [12, 18, 24, 30]

=== ASYNC FUNCTIONS ===
Future: [Future: Resolved(User_42)]
User: User_42

=== ASYNC + ARROW FUNCTIONS ===
Processed users: [Processed_1, Processed_2, Processed_3, Processed_4, Processed_5]

=== HIGHER-ORDER FUNCTIONS ===
Original: [1, 2, 3]
Doubled with func type: [2, 4, 6]
Tripled with func type: [3, 6, 9]
Doubled: [2, 4, 6]
Tripled: [3, 6, 9]

=== ALL TESTS COMPLETED ===
null
Running test: .\tests\async_and_arrays.rcc
=== TESTING ASYNC FUNCTIONS ===
Future: [Future: Resolved(Data fetched!)]
Result after await: Data fetched!

=== TESTING ARRAY METHODS ===
Original array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

--- map() ---
Doubled: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

--- filter() ---
Even numbers: [2, 4, 6, 8, 10]
Greater than 5: [6, 7, 8, 9, 10]

--- reduce() ---
Sum of all numbers: 55
Product of all numbers: 3628800

--- find() ---
First number > 7: 8
First number > 100: null

--- findIndex() ---
Index of 5: 4
Index of > 100: -1

--- some() ---
Has even numbers? true
Has negative numbers? false

--- every() ---
All positive? true
All even? false

=== METHOD CHAINING ===
Even numbers * 3, > 10: [12, 18, 24, 30]

--- Small array composition ---
Original: [1, 2, 3]
Doubled: [2, 4, 6]
Tripled: [3, 6, 9]

=== ALL TESTS COMPLETED ===
null
Running test: .\tests\comprehensive_stdlib_demo.rcc
Running test: .\tests\demo_colors.rcc
╔══════════════════════════════════════════════╗
║   DEMOSTRACIÓN DE COLORES EN RACCOON        ║
╔══════════════════════════════════════════════╗

🎨 TIPOS DE DATOS CON COLORES:
─────────────────────────────────────────────

📝 Strings (Verde):
  → Hola Mundo Raccoon

🔢 Números (Amarillo):
  Enteros → 42 100 -5
  Decimales → 3.14 2.71 0.5

✓ Booleanos (Amarillo):
  → true false

∅ Null (Gris):
  → null

📋 Listas (Magenta):
  → [1, 2, 3, 4, 5]
  → [a, b, c]


🎯 OPERACIONES Y RESULTADOS:
─────────────────────────────────────────────

a = 10 │ b = 20
a + b = 30
a * b = 200
a < b = true


✨ COMPARACIÓN:
─────────────────────────────────────────────

👤 Usuario: Alice
   Edad: 30 años
   Estado: Activo


╚══════════════════════════════════════════════╝
║   FIN DE LA DEMOSTRACIÓN                     ║
╚══════════════════════════════════════════════╝
null
Running test: .\tests\feature_array_destructuring.rcc
a = 1
b = 2
c = 3
Test 2: String Array Destructuring
hello world raccoon
Test 3: Float Array Destructuring
Pi: 3.14
E: 2.71
Sqrt2: 1.41
Test 4: Function Parameter Destructuring
Sum of first two: 30
Values: 100, 200, 300
Test 5: Block Scope Destructuring
First: 7, Second: 14
Test 6: Destructuring in Loops
[0] = 0
[1] = 10
[2] = 20
Test 7: Destructuring in Class Methods
AP-1 processing: 5, 10, 15
Result: 30
First value: 99
Test 8: Destructuring in Conditionals
Conditional values: 50, 60
Test 9: Nested Function Calls
Multiplied: 2, 4, 6
Test 10: Let vs Const Destructuring
Let: 1, 2
Const: 10, 20, 30
Test 11: For-in Loop with Arrays
Item: 100
Item: 200
Item: 300
Test 12: Range with Destructuring
Range values: 1, 2, 3
Test 13: Constructor with Destructuring
Vector created: [1, 2, 3]
Components: 3 items
=== ALL ARRAY DESTRUCTURING TESTS PASSED ===
✅ Basic array destructuring
✅ String/Float arrays
✅ Function parameters
✅ Block scope
✅ Loops
✅ Class methods
✅ Conditionals
✅ Nested calls
✅ Let vs Const
✅ For-in loops
✅ Ranges
✅ Constructors
null
Running test: .\tests\feature_arrow_functions.rcc
8
14
42
42
true
false
Hello, World!
12
19
true
8
15
All arrow function tests passed!
null
Running test: .\tests\feature_decorators.rcc
=== DECORATORS FEATURE TEST ===
Test 1: Simple Class Decorator
Value: 42
Test 2: Decorator with Single Argument
Test 3: Decorator with Multiple Arguments
Test 4: Multiple Decorators
Test 5: Decorators with String Arguments
Test 6: Decorators with Float Arguments
Test 7: Decorators with Inheritance
Test 8: Complex Decorator Logic
Size: 5
Test 9: Boolean Argument Decorators
Test 10: Decorator Ordering Test
=== ALL DECORATOR TESTS PASSED ===
✅ Simple decorators
✅ Decorators with arguments
✅ Multiple decorators
✅ String arguments
✅ Float arguments
✅ With inheritance
✅ Complex logic
✅ Boolean arguments
✅ Decorator ordering
null
Running test: .\tests\feature_object_destructuring.rcc
=== OBJECT DESTRUCTURING FEATURE TEST ===
Test 1: Global Variable Destructuring
x = 10
y = 20
x2 = 100
y2 = 200
Test 2: Block Scope Destructuring
Name: Alice
Age: 30
Test 3: Function Parameter Destructuring
Point(5, 15)
Hello Bob, you are 25 years old
Test 4: Destructuring in Conditionals
Resolution: 800x600
Test 5: Destructuring in While Loop
Loop 0: (0, 0)
Loop 1: (10, 20)
Test 6: Destructuring in Class Methods
Processor-1 processing: x=7, y=14
Combined result: 10
Test 7: Multiple Destructurings
User 1: user1 (user1@test.com)
User 2: user2 (user2@test.com)
Test 8: Destructuring with Function Returns
Created point: (42, 84)
Test 9: Nested Function Calls
Original: (3, 4)
Doubled sum: 14
Test 10: Destructuring in Try-Catch
Try block: Test, 99
=== ALL OBJECT DESTRUCTURING TESTS PASSED ===
✅ Global scope destructuring
✅ Block scope destructuring
✅ Function parameter destructuring
✅ Conditional destructuring
✅ Loop destructuring
✅ Class method destructuring
✅ Multiple destructurings
✅ With function returns
✅ Nested calls
✅ Try-catch blocks
null
Running test: .\tests\REFERENCE_COMPATIBILITY_TEST.rcc
╔════════════════════════════════════════════╗
║  RACCOON COMPATIBILITY REFERENCE TEST     ║
║  Version: 1.0.0                           ║
║  Date: October 2025                       ║
╚════════════════════════════════════════════╝
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 1: Object Destructuring
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1.1 Basic Object Destructuring
  ✓ x=10, y=20
1.2 Object Destructuring in Functions
  ✓ sum=20
1.3 Object Destructuring in Class Methods
  ✓ result=14
  ✅ Object Destructuring: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 2: Array Destructuring
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2.1 Basic Array Destructuring
  ✓ a=1, b=2, c=3
2.2 String Array Destructuring
  ✓ w1=hello, w2=world
2.3 Array Destructuring in Function Parameters
  ✓ arraySum=60
2.4 Array Destructuring in Constructors
  ✓ magnitude=7
  ✅ Array Destructuring: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 3: Decorators
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3.1 Simple Class Decorator
3.2 Decorator with Arguments
3.3 Multiple Decorators
  ✅ Decorators: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 4: Feature Integration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4.1 Decorators + Object Destructuring
  ✓ Authenticating user: admin
4.2 Decorators + Array Destructuring
  ✓ Processing: min=10, max=100, avg=55
4.3 All Features Combined
  ✓ Querying users with limit 100
  ✓ Batch inserting 10 records of size 1024
  ✓ Total bytes: 10240
  ✅ Feature Integration: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 5: Edge Cases & Stress Tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
5.1 Nested Object Access
  ✓ Location: NYC, USA
5.2 Destructuring in Loops
  ✓ Loop[0]: 0
  ✓ Loop[1]: 10
5.3 Destructuring with Ranges
  ✓ Range values: 1, 2, 3
5.4 Multiple Destructurings
  ✓ P1(1,2), P2(3,4)
5.5 Decorator with Complex Arguments
  ✓ Service call executed
  ✅ Edge Cases: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 6: Backward Compatibility
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
6.1 Regular Classes
  ✓ Old style class value: 100
6.2 Regular Functions
  ✓ Regular function result: 15
6.3 Regular Variables
  ✓ Normal var: 42, const: constant
  ✅ Backward Compatibility: PASSED
╔════════════════════════════════════════════╗
║           COMPATIBILITY REPORT            ║
╚════════════════════════════════════════════╝
✅ SECTION 1: Object Destructuring ......... PASSED
✅ SECTION 2: Array Destructuring .......... PASSED
✅ SECTION 3: Decorators ................... PASSED
✅ SECTION 4: Feature Integration .......... PASSED
✅ SECTION 5: Edge Cases ................... PASSED
✅ SECTION 6: Backward Compatibility ....... PASSED
╔════════════════════════════════════════════╗
║    ALL COMPATIBILITY TESTS PASSED ✅      ║
║                                           ║
║  Object Destructuring:    100% ✅         ║
║  Array Destructuring:     100% ✅         ║
║  Decorators:              100% ✅         ║
║  Feature Integration:     100% ✅         ║
║  Edge Cases:              100% ✅         ║
║  Backward Compatibility:  100% ✅         ║
║                                           ║
║  OVERALL STATUS:          EXCELLENT ✅    ║
╚════════════════════════════════════════════╝
null
Running test: .\tests\syntax_arrays.rcc
=== ARRAYS TEST ===
Test 1: Array Declaration
Int array length: 5
String array length: 3
Float array length: 3
Bool array length: 3
Test 2: Array Access
First element: 10
Second element: 20
Last element: 50
Test 3: Array Modification
Before: 1
After: 100
Test 4: Empty Array
Empty array length: 0
Test 5: Array with Variables
Array from vars: 1, 2, 3
Test 6: Nested Arrays
Matrix[0][0]: 1
Matrix[1][1]: 4
Matrix[2][0]: 5
Test 7: Array Iteration
Fruit: apple
Fruit: banana
Fruit: cherry
Test 8: Array from Range
Range 1..5 length: 5
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
Test 9: Array Push
Before push: 2
After push: 3
Last element: 3
Test 10: Array Pop
Before pop: 3
Popped value: 30
After pop: 2
Test 11: String as Character Array
String length: 5
First char: h
Last char: o
Test 12: Array Operations
Combined length: 4
Test 13: Arrays in Functions
First: 100
Last: 300
Test 14: Multi-dimensional Arrays
Grid[0][0]: 1
Grid[1][1]: 5
Grid[2][2]: 9
Test 15: Length Property
Empty: 0
After 1 push: 1
After 3 pushes: 3
=== ALL ARRAYS TESTS PASSED ===
✅ Array declaration
✅ Array access
✅ Array modification
✅ Empty arrays
✅ Arrays from variables
✅ Nested arrays
✅ Array iteration
✅ Range creation
✅ Push method
✅ Pop method
✅ String indexing
✅ Array concatenation
✅ Arrays in functions
✅ Multi-dimensional access
✅ Length property
null
Running test: .\tests\syntax_arrow_functions.rcc
10
30
42
42
75
true
false
Hello, TypeScript
25
11
13
true
false
8
15
15
25
49
103
35
7
12
All TypeScript-style arrow function tests passed!
null
Running test: .\tests\syntax_bitwise_operators.rcc
8
14
6
-6
20
5
4611686018427387902
8
512
8
14
6
20
5
8
null
Running test: .\tests\syntax_classes.rcc
=== CLASSES TEST ===
Test 1: Basic Class
Name: Alice
Age: 30
Test 2: Class Methods
Initial: 10
After add(5): 15
Test 3: Multiple Methods
After 2 increments: 2
After 1 decrement: 1
Test 4: String Fields
1984 by Orwell
Pages: 328
Test 5: Boolean Fields
Status: OFF
After toggle: ON
Test 6: Array Fields
Student count: 2
Test 7: Method Return Values
Area: 15
Perimeter: 16
Test 8: Inheritance
Dog name: Rex
Breed: Labrador
Sound: Woof!
Test 9: Methods with Parameters
max(10, 20): 20
min(10, 20): 10
Test 10: Multiple Instances
p1: (0, 0)
p2: (10, 20)
=== ALL CLASSES TESTS PASSED ===
✅ Basic class creation
✅ Class methods
✅ Multiple methods
✅ String fields
✅ Boolean fields
✅ Array fields
✅ Method returns
✅ Inheritance
✅ Static methods
✅ Multiple instances
null
Running test: .\tests\syntax_control_flow.rcc
=== CONTROL FLOW TEST ===
Test 1: If Statement
x is greater than 5
Test 2: If-Else Statement
Adult
18 or older
Test 3: If-Else Chain
Grade: B
Test 4: Nested If
Positive and greater than 10
Test 5: While Loop
Counter: 0
Counter: 1
Counter: 2
Test 6: While with Break
i: 0
i: 1
i: 2
Test 7: While with Continue
j: 1
j: 2
j: 4
j: 5
Test 8: For-In Loop with Array
Number: 10
Number: 20
Number: 30
Test 9: For-In Loop with Range
Range: 1
Range: 2
Range: 3
Range: 4
Range: 5
Test 10: Nested Loops
(0, 0)
(0, 1)
(1, 0)
(1, 1)
Test 11: Complex Conditions
a < b < c is true
At least one condition is true
a is not greater than b
Test 12: Early Return Pattern
checkValue(-5): negative
checkValue(0): zero
checkValue(10): positive
=== ALL CONTROL FLOW TESTS PASSED ===
✅ If statement
✅ If-else statement
✅ If-else chain
✅ Nested if
✅ While loop
✅ Break statement
✅ Continue statement
✅ For-in with arrays
✅ For-in with ranges
✅ Nested loops
✅ Complex conditions
✅ Early returns
null
Running test: .\tests\syntax_error_handling.rcc
=== ERROR HANDLING TEST ===
Test 1: Basic Try-Catch
Inside try block
Caught: Basic error
Test 2: Normal Flow
This executes normally
x = 10
Test 3: String Error
Error message: Something went wrong!
Test 4: Try-Catch in Function
Result: error: Function error
Test 5: Nested Try-Catch
Outer try
Inner try
Inner catch: Inner error
After inner try-catch
Test 6: Multiple Statements
Sum: 3
Caught: Error after calculations
Test 7: Try-Catch with Return
10 % 2 = 0
Error: Division by zero
10 % 0 = 0
Test 8: Variable Scope
Status: error
Test 9: Try-Catch in Loop
i = 0
Caught: Error at i=1
i = 2
Test 10: Conditional Throw
Valid age
Invalid: Age cannot be negative
Invalid: Age too high
Test 11: Multiple Throws
Value is valid
Validation error: Negative
Validation error: Zero
Validation error: Too large
Test 12: Error Propagation
caught: Inner function error
=== ALL ERROR HANDLING TESTS PASSED ===
✅ Basic try-catch
✅ Normal flow
✅ String errors
✅ Try-catch in functions
✅ Nested try-catch
✅ Multiple statements
✅ Try-catch with return
✅ Variable scope
✅ Try-catch in loops
✅ Conditional throws
✅ Multiple throws
✅ Error propagation
null
Running test: .\tests\syntax_functions.rcc
=== FUNCTIONS TEST ===
Test 1: Basic Function
Hello from function!
Test 2: Function with Parameters
5 + 3 = 8
Test 3: Multiple Parameters
Name: Alice, Age: 25, City: NYC
Test 4: Return Values
6 * 7 = 42
20 / 4 = 5
Test 5: Default Parameters
Hello, Alice!
Hello, Guest!
Test 6: Recursive Function
factorial(5) = 120
Test 7: Function Composition
quadruple(5) = 20
Test 8: String Return Functions
Full name: John Doe
Shouted: HELLO
Test 9: Boolean Return Functions
isEven(4): true
isEven(7): false
isPositive(-5): false
isPositive(10): true
Test 10: Array Parameters
Sum of array: 15
Test 11: Array Return
Range length: 5
Test 12: Nested Calls
process(5) = 8
Test 13: Float Functions
average(5.0, 10.0) = 7.5
Test 14: Void Functions
LOG: Test message
=== ALL FUNCTIONS TESTS PASSED ===
✅ Basic functions
✅ Parameters
✅ Multiple parameters
✅ Return values
✅ Default parameters
✅ Recursion
✅ Function composition
✅ String returns
✅ Boolean returns
✅ Array parameters
✅ Array returns
✅ Nested calls
✅ Float functions
✅ Void functions
null
Running test: .\tests\syntax_interfaces.rcc
=== INTERFACES TEST ===
Test 1: Interface Definition
Name: Alice
Test 2: Interface with Methods
Drawing circle with radius 5
Test 3: Multiple Members
Area: 20
Test 4: Multiple Interfaces
ID: 1
Product: Laptop
Test 5: Interface with Boolean
Initial: false
After toggle: true
Test 6: Multiple Methods
Counter value: 2
Test 7: Return Types
Result: 30
Test 8: Empty Interface
Marked value: 42
Test 9: String Methods
Formatted: 15/3/2024
Test 10: Implementation Chain
Bob, 35, Developer
=== ALL INTERFACES TESTS PASSED ===
✅ Basic interface
✅ Interface with methods
✅ Multiple members
✅ Multiple interfaces
✅ Boolean interfaces
✅ Multiple methods
✅ Return types
✅ Empty interfaces
✅ String methods
✅ Implementation chains
null
Running test: .\tests\syntax_interfaces_simple.rcc
=== INTERFACES TEST (SIMPLIFIED) ===
Test 1: Basic Interface
✓ Interface Point defined
Test 2: Multiple Interfaces
✓ Multiple interfaces defined
Test 3: Interface with Different Types
✓ Interface with multiple types defined
Test 4: Classes Following Interface Structure
Area: 50
=== ALL INTERFACE TESTS PASSED ===
✅ Basic interface definitions
✅ Multiple interfaces
✅ Interfaces with multiple types
✅ Classes following interface structure
Note: Interfaces only support properties, not methods
Note: 'implements' keyword is not currently enforced
null
Running test: .\tests\syntax_operators.rcc
=== OPERATORS TEST ===
Test 1: Arithmetic Operators
Addition: 13
Subtraction: 7
Multiplication: 30
Division: 3.3333333333333335
Modulo: 1
Test 2: Comparison Operators
Equal: true
Not equal: true
Less than: true
Greater than: true
Less or equal: true
Greater or equal: true
Test 3: Logical Operators
AND (true && true): true
AND (true && false): false
OR (false || true): true
OR (false || false): false
NOT (!true): false
NOT (!false): true
Test 4: Assignment Operators
Initial: 10
After +5: 15
After -3: 12
After *2: 24
After -4: 20
Test 5: Compound Assignment
num += 10: 110
num -= 5: 105
num *= 2: 210
num -= 10: 200
Test 6: Unary Operators
Negation: -42
Logical NOT: false
Test 7: String Concatenation
Concatenation: Hello World
Mixed: Raccoon v1
Test 8: Operator Precedence
2 + 3 * 4 = 14
(2 + 3) * 4 = 20
10 - 2 + 3 = 11
Test 9: Boolean Expressions
Is adult (25 >= 18): true
Passed (85 in range): true
Can edit: true
Test 10: Null Comparison
Is null: true
Is not null: true
=== ALL OPERATORS TESTS PASSED ===
✅ Arithmetic operators
✅ Comparison operators
✅ Logical operators
✅ Assignment operators
✅ Compound assignment
✅ Unary operators
✅ String concatenation
✅ Operator precedence
✅ Boolean expressions
✅ Null comparison
null
Running test: .\tests\syntax_primitive_types.rcc
=== PRIMITIVE TYPES TEST ===
Test 1: Integer Type
Positive: 42
Negative: -100
Zero: 0
Max: 2147483647
Min: -2147483648
Test 2: Float Type
Pi: 3.14
Negative: -2.71
Zero: 0
E: 2.71828
Test 3: String Type
String 1: Hello World
String 2: Raccoon
Empty: ''
Greeting: ¡Hola!
Test 4: Boolean Type
True: true
False: false
Constant: true
Test 5: Null Type
Null value created
Assigned value: 42
Test 6: Type Conversions
Int to Float: 10
Int to String: 10
Test 7: String Operations
Upper: HELLO
Lower: hello
Split length: 3
Test 8: Type Checking
typeof num: int
typeof msg: str
Test 9: Constants
PI: 3.14159
Version: 1
App: Raccoon
Test 10: Mixed Type Operations
Int + Float: 7.5
Number: 42
=== ALL PRIMITIVE TYPES TESTS PASSED ===
✅ Integer type
✅ Float type
✅ String type
✅ Boolean type
✅ Null type
✅ Type conversions
✅ String operations
✅ Type checking
✅ Constants
✅ Mixed operations
null
Running test: .\tests\syntax_scopes.rcc
=== SCOPES TEST ===
Test 1: Global Scope
Global: 100
Test 2: Block Scope
Inside block: 2
Access outer: 1
Outside block: 1
Test 3: Function Scope
Inside function: 20
Access outer: 10
Outside function: 10
Test 4: Variable Shadowing
Outer x: 1
Inner x: 2
Outer x again: 1
Test 5: Loop Scope
Loop iteration 0: 0
Loop iteration 1: 10
Loop iteration 2: 20
After loop: 5
Test 6: If Statement Scope
Inside if: 42
After if block
Test 7: Nested Blocks
Level 3: 3
Access level 2: 2
Access level 1: 1
Level 2: 2
Level 1: 1
Test 8: Reassignment
Initial: 10
After reassignment: 20
After calculation: 25
Test 9: Constants
Constant value: 100
PI: 3.14159
Test 10: For-In Scope
Number: 10
Number: 20
Number: 30
After for-in loop
Test 11: Class Field Scope
Counter: 1
Test 12: Try-Catch Scope
Try block: 100
Catch block: caught
Test 13: Multiple Variables
a=1, b=2, c=3
Test 14: Variable Lifetime
First call: 1
Second call: 1
Test 15: Scope Chain
Access all: 1, 2, 3
=== ALL SCOPES TESTS PASSED ===
✅ Global scope
✅ Block scope
✅ Function scope
✅ Variable shadowing
✅ Loop scope
✅ If statement scope
✅ Nested blocks
✅ Reassignment
✅ Constants
✅ For-in scope
✅ Class field scope
✅ Try-catch scope
✅ Multiple variables
✅ Variable lifetime
✅ Scope chain
null
Running test: .\tests\syntax_static_methods.rcc
=== STATIC METHODS TEST ===
Test 1: Basic Static Method
max(10, 20): 20
min(10, 20): 10
Test 2: Multiple Parameters
add(1, 2, 3): 6
multiply(2, 3, 4): 24
Test 3: String Return
HelloWorld
xxx
Test 4: Mix Static and Instance
Counter value: 1
Test 5: Factory Pattern
Origin: (0, 0)
From array: (10, 20)
=== ALL STATIC METHOD TESTS PASSED ===
✅ Basic static methods
✅ Multiple parameters
✅ String return types
✅ Mixed static and instance
✅ Factory pattern
null
Running test: .\tests\syntax_templates.rcc
=== TEMPLATE STRINGS TEST ===
Test 1: Basic Template
Hello, World!
Test 2: Integer Interpolation
I am 25 years old
Test 3: Float Interpolation
Price: $19.99
Test 4: Boolean Interpolation
Status: true
Test 5: Multiple Variables
Name: John Doe, Age: 30
Test 6: Expression Interpolation
5 + 3 = 8
Test 7: Templates in Functions
Hello Alice, you are 28 years old
Test 8: Object Properties
My name is Bob and I'm 35
Test 9: Array Properties
Array has 5 items
Test 10: Comparison Results
x < y is true
Test 11: Complex Expressions
Sum of 5 and 10 is 15
Test 12: Function Call Interpolation
Double of 7 is 14
Test 13: Template Concatenation
Hello World
Test 14: Empty Templates
Empty string length: 0
Test 15: Templates in Conditionals
Grade: B (85)
=== ALL TEMPLATE STRINGS TESTS PASSED ===
✅ Basic templates
✅ Integer interpolation
✅ Float interpolation
✅ Boolean interpolation
✅ Multiple variables
✅ Expressions
✅ Function templates
✅ Object properties
✅ Array properties
✅ Comparisons
✅ Complex expressions
✅ Function calls
✅ Concatenation
✅ Empty templates
✅ Conditional templates
null
Running test: .\tests\test_array_methods.rcc
Original array:
Doubled:
Evens:
Sum:
ForEach:
  -
  -
  -
  -
  -
First > 3:
Index of first > 3:
Has > 10:
Has < 3:
All positive:
All > 10:
All tests completed!
null
Running test: .\tests\test_arrow.rcc
[2, 4, 6]
null
Running test: .\tests\test_arrow2.rcc
[2, 4, 6]
null
Running test: .\tests\test_arrow_complete.rcc
=== ARROW FUNCTIONS TEST ===

1. Arrow function with map:
Doubled: [2, 4, 6, 8, 10]

2. Arrow function with filter:
Even numbers: [2, 4]

3. Arrow function with reduce:
Sum: 15

4. Arrow function with type annotations:
Squared: [1, 4, 9, 16, 25]

5. Method chaining:
Filtered > 2, then * 10: [30, 40, 50]

6. Arrow function as variable:
Tripled: [3, 6, 9, 12, 15]

=== ALL TESTS PASSED ===
null
Running test: .\tests\test_arrow_notype.rcc
14
null
Running test: .\tests\test_arrow_simple.rcc
8
null
Running test: .\tests\test_arrow_single.rcc
14
null
Running test: .\tests\test_assign_error.rcc
Running test: .\tests\test_builtins.rcc
Running test: .\tests\test_classes_advanced.rcc
🦝 === ADVANCED CLASSES TEST ===
✅ Test 1: Static Methods
  max(10, 20): 20
  min(10, 20): 10
  abs(-15): 15
  abs(15): 15
✅ Test 2: Getters and Setters
  Celsius: 25
  Fahrenheit: 77
  Kelvin: 298.15
  After setting to 68°F:
  Celsius: 20
✅ Test 3: Method Overriding
  Circle with radius 5
  Area: 78.53975
  Rectangle 4x6
  Area: 24
✅ Test 4: Encapsulation Pattern
  Initial: 0/3
  After increment: 1
  After 2 more increments: 3
  Can increment more: false
  After reset: 0
✅ Test 5: Complex Constructor
  User: john_doe
  Email: john@example.com
  Status: Active (0 logins)
  After 3 logins: Active (3 logins)
  After deactivation: Inactive
✅ Test 6: Two-Level Inheritance
  Car: Toyota (4 doors)
  Brand: Toyota
  Doors: 4
✅ Test 7: Complex State Management
  Cart empty: true
  Items in cart: 3
  Total: $12.25
✅ Test 8: Methods with Multiple Returns
  Empty string: Error: Value is empty
  'ab': Error: Value too short
  'hello': Valid
  Long string: Error: Value too long
✅ Test 9: Instance Type Checking
  Animal name: Generic
  Cat name: Whiskers
  Cat says: Meow!
✅ Test 10: Multiple Instances Stress Test
  Created 5 points
  Point 0: (0, 0)
  Point 1: (3, 4)
  Point 2: (5, 12)
🎉 === ALL ADVANCED TESTS PASSED ===
✅ Static methods
✅ Getters and setters
✅ Method overriding in inheritance
✅ Encapsulation patterns
✅ Complex constructor logic
✅ Two-level inheritance
✅ Complex state management
✅ Multiple return points
✅ Instance type checking
✅ Multiple instances stress test
🦝 Raccoon Classes: ALL ADVANCED FEATURES WORKING!
null
Running test: .\tests\test_classes_comprehensive.rcc
🦝 === COMPREHENSIVE CLASSES TEST ===
✅ Test 1: Basic Class with Constructor
  Name: Alice
  Age: 30
✅ Test 2: Class with Methods
  Initial value: 10
  After add(5): 15
  After multiply(2): 30
  After subtract(10): 20
  After reset: 0
✅ Test 3: Class with Different Data Types
  Laptop - $999.99 (Stock: 5)
  Total value: $4999.95
  In stock: true
  After selling 3: Laptop - $999.99 (Stock: 2)
  After selling 2 more: Laptop - $999.99 (Stock: 0)
  In stock: false
✅ Test 4: Class with Array Properties
  Classroom: Math 101
  Students: 3
  Average grade: 91.33333333333333
✅ Test 5: Class Inheritance
  Rex is 3 years old
  Rex is a Labrador
  Sound: Woof! Woof!
✅ Test 6: Multiple Instances Independence
  Counter A: 3
  Counter B: 1
✅ Test 7: Complex Methods with Logic
  Account: ACC001
  Initial balance: $1000
  After deposit $500: $1500
  After withdraw $200: $1300
✅ Test 8: Nested Method Calls
  Initial: 5
  After add(3) then multiply(2): 16
✅ Test 9: Class with String Operations
  Original: Hello Raccoon
  Uppercase: HELLO RACCOON
  Lowercase: hello raccoon
  Length: 13
  Is empty: false
✅ Test 10: Class with Object Composition
  John Doe (ID: 1001)
  John Doe (ID: 1001) - 123 Main St, Springfield 12345
🎉 === ALL TESTS PASSED ===
✅ Basic class with constructor
✅ Class with methods
✅ Different data types (str, int, float, bool)
✅ Array properties
✅ Class inheritance (extends, super)
✅ Multiple instances independence
✅ Complex methods with logic
✅ Nested method calls
✅ String operations in classes
✅ Object composition
🦝 Raccoon Classes: FULLY FUNCTIONAL!
null
Running test: .\tests\test_debugging_features.rcc
=== TEST DE DEBUGGING Y COLORES ===

Test 1: print() con colores para diferentes tipos
Strings en verde: Hello World
Números en amarillo: 42 3.14
Booleanos en amarillo: true false
Null en gris: null
Lista en magenta: [1, 2, 3]

Test 2: función println()
Primera línea
Segunda línea
Tercera línea

Test 3: Mezcla de tipos en una sola llamada
Usuario: Bob Edad: 25 Activo: true Score: 95.5

Test 4: Resultados de operaciones
x = 10 y = 5
x + y = 15
x * y = 50
x > y = true

Test 5: Estructuras de datos
Lista de números: [10, 20, 30, 40, 50]
Lista mixta: [1, two, 3, four]

Test 6: Funciones
Resultado: Hello, World

=== TODOS LOS TESTS COMPLETADOS ===
null
Running test: .\tests\test_decorators_advanced.rcc
🦝 === ADVANCED DECORATORS TEST ===
✅ Test 1: Method Decorators
  Calculator value: 15
✅ Test 2: Timing Decorator Pattern
  [TIMER END] Result: 5050
✅ Test 3: API Route Decorator Pattern
  List of users
  User created
✅ Test 4: Database Entity Decorator
  Product: Laptop - $999.99
✅ Test 5: Validation Decorator
  Age set to 25
  Name set to John
✅ Test 6: Memoization Pattern
  Result 1: 25
  Result 2: 169
✅ Test 7: Singleton Pattern
  Connected to: localhost:5432
✅ Test 8: Rate Limiting Decorator
  API response from /api/data
✅ Test 9: Transaction Decorator
  Transferred $100
  [TRANSACTION] Commit
✅ Test 10: Error Handling Decorator
  Success
✅ Test 11: AOP Pattern - Before/After
  [BEFORE] Executing...
  Processed: important data
  [AFTER] Done
✅ Test 12: Feature Flag Decorator
  Displaying new UI
✅ Test 13: Dependency Injection
  UserCtrl handling request
✅ Test 14: Event Emitter Pattern
  Created user: alice
✅ Test 15: Multi-Decorator Composition
  Executed: delete_database
🎉 === ALL ADVANCED DECORATOR TESTS PASSED ===
✅ Method decorators
✅ Timing decorator pattern
✅ API route decorator
✅ Database entity decorator
✅ Validation decorator
✅ Memoization pattern
✅ Singleton pattern
✅ Rate limiting
✅ Transaction decorator
✅ Error handling
✅ AOP pattern
✅ Feature flags
✅ Dependency injection
✅ Event emitters
✅ Multi-decorator composition
🦝 Raccoon Advanced Decorators: PRODUCTION READY!
null
Running test: .\tests\test_decorators_comprehensive.rcc
🦝 === COMPREHENSIVE DECORATORS TEST ===
✅ Test 1: Basic Function Decorator
  Result: Hello, Alice
✅ Test 2: Function Decorator with Arguments
  Sum: 8
✅ Test 3: Class Decorator
  User greeting: Hello, Bob
✅ Test 4: Multiple Decorators on Function
  Result: Function with multiple decorators
✅ Test 5: Decorator with Multiple Arguments
  Percentage: 75%
✅ Test 6: Class Decorator with Arguments
  Charlie is 30 years old
✅ Test 7: Performance Tracking Decorator
Running test: .\tests\test_error_display.rcc
Iniciando test...
Running test: .\tests\test_error_positions.rcc
Running test: .\tests\test_explicit_generics.rcc
42
100
400
25
Hello
All tests passed!
null
Running test: .\tests\test_func_type.rcc
=== FUNC TYPE TESTS ===

[ 1 ] Basic func type assignment
  Result: Hello, Alice

[ 2 ] func as function parameter
  square(5) = 25
  double(5) = 10

[ 3 ] Arrays of func
  Applying operations to 3:
    operations[0](3) = 9
    operations[1](3) = 6

[ 4 ] func with arrow functions
  add(10, 5) = 15
  subtract(10, 5) = 5

[ 5 ] func returning func
  addOp(4, 3) = 7
  mulOp(4, 3) = 12

[ 6 ] func with different return types
  toString(42) = 42
  toBool(5) = true
  toBool(-1) = false

[ 7 ] func with higher-order array methods
  Original: [1, 2, 3, 4, 5]
  Mapped (increment): [2, 3, 4, 5, 6]
  Filtered (isEven): [2, 4]
  Reduced (sum): 15

[ 8 ] Multiple func variables
  addFunc(10, 5) = 15
  subFunc(10, 5) = 5
  mulFunc(10, 5) = 50

[ 9 ] func with async functions
  Async result: Data_123

==================================================
✅ ALL FUNC TYPE TESTS PASSED
==================================================

The 'func' type works as a generic function type!
It can represent any function signature dynamically.
null
Running test: .\tests\test_generics.rcc
=== GENERICS SYSTEM TEST ===
Test 1: Generic Box<T>
Int box: 42
String box: Hello
Updated int box: 100
Test 2: Generic Pair<K, V>
Pair: age = 25
Test 3: Generic with Constraint
Container size: 3
First item: 10
Test 4: Optional<T> Type
Some has value: true
None has value: false
Some value: 42
None default: 0
Test 5: Result<T> Type
Good result success: true
Good result value: 5
Bad result success: false
Bad result error: Division by zero
Test 6: Stack<T>
Top: third
Pop: third
Pop: second
Is empty: false
=== ALL GENERIC TESTS PASSED ===
null
Running test: .\tests\test_generics_simple.rcc
=== GENERICS SYSTEM TEST (Simplified) ===
Test 1: Generic Box<T>
Int box: 42
String box: Hello
Updated int box: 100
Test 2: Generic Pair<K, V>
Pair: age = 25
Test 3: Generic Container<T>
Container size: 3
First item: 10
Test 4: Optional<T> Type
Some has value: true
None has value: false
Some value: 42
None default: 0
Test 5: Stack<T>
Top: third
Pop: third
Pop: second
Is empty: false
=== ALL GENERIC TESTS PASSED ===
null
Running test: .\tests\test_generics_vs_bitwise.rcc
42
32
10
20
All tests passed!
null
Running test: .\tests\test_http_client.rcc
Running test: .\tests\test_http_debug.rcc
Running test: .\tests\test_http_final.rcc
Running test: .\tests\test_http_minimal.rcc
Running test: .\tests\test_http_net.rcc
Running test: .\tests\test_http_simple.rcc
Testing HTTP with default import...
Response: { url: https://coatl.in/, headers: Map { content-type: text/html; charset=utf-8, server: Vercel, strict-transport-security: max-age=63072000, etag: "a096f8b2a3cc43cd18ec5f9eb5ebd7ba", access-control-allow-origin: *, age: 2301750, date: Tue, 21 Oct 2025 04:42:05 GMT, last-modified: Wed, 24 Sep 2025 13:19:34 GMT, cache-control: public, max-age=0, must-revalidate, content-disposition: inline, x-vercel-id: sfo1::kcs96-1761021725844-2b5c2b8b5dea, accept-ranges: bytes, x-vercel-cache: HIT, content-length: 17874 }, body: null, status: 200, statusText: OK }

Success! Default import working!
null
Running test: .\tests\test_import_default.rcc
Testing default import...
Http: class Http
null
Running test: .\tests\test_index_access_assignment.rcc
Testing index access and assignment for objects...

Test 1: Reading properties with index notation
  config["api-key"]: secret123
  config["base-url"]: https://api.example.com
  config["timeout"]: 5000
  ✓ Index access works!

Test 2: Assigning properties with index notation
  After assignment:
    config["api-key"]: new-secret-456
    config["version"]: v2.0
    config["timeout"]: 10000
  ✓ Index assignment works!

Test 3: Mixed access patterns
  user.name: Carlos
  user["user-id"]: 12345
  user.email: carlos@example.com

  After updates:
  user.name: Carlos López
  user["user-id"]: 99999
  user.email: new@example.com
  ✓ Mixed access works!

Test 4: Dynamic property access
  data[propName]: value2
  After update: updated-value2
  ✓ Dynamic property access works!

All tests passed! Index access and assignment work correctly.
null
Running test: .\tests\test_interface.rcc
10
null
Running test: .\tests\test_map_complete.rcc
=== Map<K,V> Implementation Tests ===

[ 1 ] Basic Map<str, int> operations:
  Set 3 values
  Get 'one': 1
  Get 'two': 2
  Has 'one': true
  Has 'missing': false
  Size: 3
  ✓ Test 1 passed

[ 2 ] Map delete operation:
  Initial size: 3
  Deleted 'b': true
  Size after delete: 2
  Has 'b': false
  Deleted non-existent 'xyz': false
  ✓ Test 2 passed

[ 3 ] Map clear operation:
  Size before clear: 3
  Size after clear: 0
  Has 'x': false
  ✓ Test 3 passed

[ 4 ] Map with float values:
  pi = 3.14159
  ✓ Test 4 passed

[ 5 ] Map<int, str> with int keys:
  1 -> one
  2 -> two
  Size: 3
  ✓ Test 5 passed

[ 6 ] Map overwrites existing values:
  Initial value: 100
  After overwrite: 200
  Size (should be 1): 1
  ✓ Test 6 passed

[ 7 ] Get non-existent key returns null:
  Non-existent key returns null ✓
  ✓ Test 7 passed

[ 8 ] Map toStr() method:
  Map as string: Map { b: 2, a: 1 }
  ✓ Test 8 passed

[ 9 ] Map with typed integers:
  i32 value: 12345
  u64 value: 999999
  ✓ Test 9 passed

[ 10 ] Null coalescing with Map.get():
  Existing key (with ??): 123
  Missing key (with ??): -1
  ✓ Test 10 passed

[ 11 ] Complex operation sequence:
  Added 5 elements, size: 5
  Deleted 2 elements, size: 3
  Added 1 more element, size: 4
  Has 'a': true
  Has 'b': false
  Has 'f': true
  ✓ Test 11 passed

[ 12 ] Map with float types:
  f32 size: 2
  f64 size: 2
  ✓ Test 12 passed

==================================================
✅ ALL MAP TESTS PASSED SUCCESSFULLY!
==================================================

Types tested:
  • Map<str, int>
  • Map<str, float>
  • Map<int, str>
  • Map<str, i32>
  • Map<str, u64>
  • Map<str, f32>
  • Map<str, f64>

Operations tested:
  • set(key, value)
  • get(key) -> value?
  • has(key) -> bool
  • delete(key) -> bool
  • clear()
  • size() -> int
  • toStr() -> str

🎉 Map<K,V> implementation working perfectly!
null
Running test: .\tests\test_map_i32.rcc
Creating Map<str, int>...
Success with int!
Creating Map<str, i32>...
Success with i32!
null
Running test: .\tests\test_map_minimal.rcc
null
Running test: .\tests\test_map_no_main.rcc
Testing Map basic operations

Map created
Value set
v1: 1
v2: 2
v3: 3
Has 'one': true
Has 'missing': false
Size: 3
Deleted 'two': true
Size after delete: 2
Size after clear: 0

✓ All Map tests passed!
null
Running test: .\tests\test_modules.rcc
Running test: .\tests\test_named_args.rcc
=== Test de argumentos nombrados ===

Llamada posicional:
Name: Alice
Age: 30
City: Madrid

Llamada con argumentos nombrados:
Name: Bob
Age: 25
City: Barcelona

Llamada mixta:
Name: Charlie
Age: 35
City: Valencia

=== Test completado ===
null
Running test: .\tests\test_named_args_comprehensive.rcc
=== Test Completo de Argumentos Nombrados ===

Test 1: Argumentos nombrados en orden diferente
Host: localhost Port: 8080 SSL: true
Host: example.com Port: 3000 SSL: false

Test 2: Valores por defecto
Name: Alice Age: 18 Active: true
Name: Bob Age: 25 Active: true
Name: Charlie Age: 18 Active: false
Name: Diana Age: 30 Active: false

Test 3: Mezcla de posicionales y nombrados
From: Madrid To: Barcelona Date: 2024-01-15 Class: economy
From: Madrid To: Barcelona Date: 2024-01-15 Class: business
From: Madrid To: Barcelona Date: 2024-01-15 Class: economy

Test 4: Funciones con retorno
5 + 3 = 8
5 * 3 = 15
5 * 3 = 15

Test 5: Diferentes tipos de datos
Message 1: HELLO HELLO HELLO 
Message 2: world world 

Test 6: Solo argumentos nombrados
Debug: true Verbose: false LogLevel: info
Debug: false Verbose: true LogLevel: debug

=== Todos los tests completados exitosamente ===
null
Running test: .\tests\test_net.rcc
Running test: .\tests\test_new_types.rcc
=== Tipos Enteros con Signo ===
i8:  -128
i16: -32768
i32: -2147483648
i64: -9223372036854776000

=== Tipos Enteros sin Signo ===
u8:  255
u16: 65535
u32: 4294967295
u64: 9223372036854776000

=== Tipos Punto Flotante ===
f32:     3.14159
f64:     2.718281828459045
decimal: 123.456789

=== Conversiones Automáticas ===
i8 -> i64:     -128
f32 -> f64:    3.14159
i32 -> decimal: -2147483648

=== Resultados de Funciones ===
addI32(10, 20):     30
multiplyF64(3.14, 2.0): 6.28
Decimal: 123.456789

=== Arrays con Nuevos Tipos ===
i32[]: [1, 2, 3, 4, 5]
f64[]: [1.1, 2.2, 3.3, 4.4, 5.5]
decimal[]: [10.5, 20.75, 30.25]

✅ Todos los tipos funcionan correctamente!
null
Running test: .\tests\test_no_conflict_generics_bitwise.rcc
16
64
8
6
40
10
All tests passed!
null
Running test: .\tests\test_null_safety_operators.rcc
====================================
NULL SAFETY OPERATORS TEST SUITE
====================================
✓ Test 1 passed: Null coalescing operator works
✓ Test 2 passed: Chained null coalescing works
⊘ Test 3 skipped: Inline object types not yet supported
⊘ Test 4 skipped: Inline object types not yet supported
✓ Test 5 passed: Null assertion on non-null value works
✓ Test 6 passed: Basic ternary operator works
✓ Test 7 passed: Nested ternary operators work
✓ Test 8 passed: Prefix increment works
✓ Test 9 passed: Postfix increment works
✓ Test 10 passed: Prefix decrement works
✓ Test 11 passed: Postfix decrement works
✓ Test 12 passed: Increment in for loop works
✓ Test 13 passed: Combined operators work
✓ Test 14 passed: Ternary in complex expression works
✓ Test 15 passed: Float increment/decrement works
====================================
TEST SUITE COMPLETE
====================================
null
Running test: .\tests\test_null_safety_simple.rcc
=== Test 1: Null Coalescing ===
default
=== Test 2: Conditional ===
adult
=== Test 3: Prefix Increment ===
6
6
=== Test 4: Postfix Increment ===
6
5
=== Test 5: Null Assertion ===
hello
null
Running test: .\tests\test_object_string_keys.rcc
Testing string literals as object keys...
✓ Created object with string literal keys
  Content-Type: application/json
  Authorization: Bearer token123
  X-Custom-Header: custom value

✓ Created object with mixed key types
  name: Carlos
  full-name: Carlos López
  age: 25
  user-id: 12345

✓ Created nested object with string keys
  API base URL: https://api.example.com
  API timeout: 5000
  Content-Type: application/json

All tests passed! String literals as object keys work correctly.
null
Running test: .\tests\test_raccoon_modules.rcc
null
Running test: .\tests\test_simple_array.rcc
[2, 4, 6]
Running test: .\tests\test_simple_modules.rcc
Test 1: print simple
Test 2: print con múltiples args 123 456
Running test: .\tests\test_stdlib_demo.rcc
null
Running test: .\tests\test_stdlib_math.rcc
Running test: .\tests\test_std_library.rcc
Running test: .\tests\test_sugar_types.rcc
=== Sugar Types Test ===

✓ int se asigna correctamente a i8, i16, i32, i64, u8, u16, u32, u64
✓ float se asigna correctamente a f32, f64, decimal
✓ Operaciones aritméticas funcionan: 1010, 8.53452
Valor i32: 42
Valor f64: 3.14159
✓ Conversión widening automática: i8 -> i32 -> i64 -> f64 = 50
✓ Arrays tipados: i8[], i32[], f64[] funcionan correctamente

✅ Todos los sugar types y conversiones funcionan perfectamente!
null
Running test: .\tests\test_toStr.rcc
42
Success!
null
Running test: .\tests\test_typescript_style_modules.rcc
╔════════════════════════════════════════════════════════╗
║   TEST: Sistema de Módulos Estilo TypeScript         ║
╚════════════════════════════════════════════════════════╝

📦 TEST 1: Named Imports
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running test: .\tests\test_types_edge_cases.rcc
=== EDGE CASES: Sistema de Tipos ===

[ 1 ] Valores límite de tipos enteros con signo:
  ✓ i8:  -128 a 127
  ✓ i16: -32768 a 32767
  ✓ i32: -2147483648 a 2147483647
  ✓ i64: -9223372036854776000 a 9223372036854776000

[ 2 ] Valores límite de tipos sin signo:
  ✓ u8:  0 a 255
  ✓ u16: 0 a 65535
  ✓ u32: 0 a 4294967295
  ✓ u64: 0 a 9223372036854776000

[ 3 ] Cadenas de conversión widening:
  ✓ i8 -> i16 -> i32 -> i64 -> f64 -> decimal: 42
  ✓ u8 -> u16 -> u32 -> u64 -> f64: 200
  ✓ u8 -> i16 -> i64 -> decimal: 100

[ 4 ] Operaciones aritméticas entre diferentes tipos:
  ✓ i8(10) + i32(20) = i32(30)
  ✓ i16(5) * i64(15) = i64(75)
  ✓ u8(8) - u32(4) = u32(4)
  ✓ f32(3.5) * f64(2.0) = f64(7)
  ✓ i32(100) + f32(2.5) = f32(102.5)

[ 5 ] Operaciones bitwise entre tipos enteros:
  ✓ i8(15) & i32(240) = i32(0)
  ✓ u8(12) | u16(10) = u16(14)
  ✓ i16(5) ^ i32(3) = i32(6)
  ✓ i8(4) << i32(2) = i32(16)
  ✓ i32(16) >> i8(2) = i32(4)

[ 6 ] Funciones con múltiples tipos de parámetros:
  ✓ i8: 50
  ✓ i32: 5000
  ✓ f64: 50.5
  ✓ decimal: 50.123456
  ✓ Sugar type en i8: i8: 25
  ✓ Sugar type en i32: i32: 2500
  ✓ Sugar type en f64: f64: 25.5

[ 7 ] Arrays con diferentes tipos numéricos:
  ✓ i8[]:  [1, 2, 3, 4, 5]
  ✓ i32[]: [100, 200, 300]
  ✓ u8[]:  [255, 128, 64]
  ✓ f32[]: [1.1, 2.2, 3.3]
  ✓ f64[]: [10.5, 20.5, 30.5]

[ 8 ] Operaciones con cero y números negativos:
  ✓ i8(0) + i8(-128) = -128
  ✓ i32(-1000000) + i32(1000000) = 0
  ✓ f64(-3.14159) * -1 = 3.14159
  ✓ Negación unaria: -42 = -42

[ 9 ] Conversiones explícitas entre tipos:
  Original i32: 256
  ✓ toI8():  127
  ✓ toI16(): 256
  ✓ toI64(): 256
  ✓ toF32(): 256
  ✓ toF64(): 256
  Original f64: 123.456
  ✓ toInt():     123
  ✓ toDecimal(): 123.456

[ 10 ] Comparaciones entre diferentes tipos:
  ✓ i8(10) == i32(10): true
  ✓ u8(50) == i16(50): true
  ✓ f32(3.14) == f64(3.14): true
  ✓ i32(100) < i64(200): true

[ 11 ] Expresiones complejas con múltiples tipos:
  ✓ i8(2) + i16(3) * i32(4) - i64(5) = i64(9)
  ✓ u8(10) * u16(20) + u32(30) = u32(230)
  ✓ f32(2.5) * f64(3.0) + decimal(1.5) = decimal(9)

[ 12 ] Operaciones con arrays de tipos mixtos:
  ✓ Array i8 length: [1, 2, 3]
  ✓ Array i32 length: [10, 20, 30]
  ✓ Array f64 length: [1.5, 2.5, 3.5]
  ✓ Después de push: i8[] = [1, 2, 3, 4]
  ✓ Después de push: i32[] = [10, 20, 30, 40]
  ✓ Después de push: f64[] = [1.5, 2.5, 3.5, 4.5]

[ 13 ] Valores decimales de alta precisión:
  ✓ decimal(0.1) + decimal(0.2) = 0.30000000000000004
  ✓ Suma de alta precisión: 999999999.1234568

[ 14 ] Funciones recursivas con tipos específicos:
Running test: .\tests\arrow_functions_demo.rcc
=== ARROW FUNCTIONS AS PARAMETERS ===

Original array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

--- MAP with arrow functions ---
Doubled (arrow): [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared (arrow): [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
Transformed (x*3+1): [4, 7, 10, 13, 16, 19, 22, 25, 28, 31]

--- FILTER with arrow functions ---
Even numbers: [2, 4, 6, 8, 10]
Greater than 5: [6, 7, 8, 9, 10]
Multiples of 3: [3, 6, 9]

--- REDUCE with arrow functions ---
Sum: 55
Product: 3628800

--- FIND/SOME/EVERY with arrow functions ---
First > 7: 8
Has evens? true
All positive? true

--- METHOD CHAINING with arrow functions ---
Evens * 3, > 10: [12, 18, 24, 30]

=== ASYNC FUNCTIONS ===
Future: [Future: Resolved(User_42)]
User: User_42

=== ASYNC + ARROW FUNCTIONS ===
Processed users: [Processed_1, Processed_2, Processed_3, Processed_4, Processed_5]

=== HIGHER-ORDER FUNCTIONS ===
Original: [1, 2, 3]
Doubled with func type: [2, 4, 6]
Tripled with func type: [3, 6, 9]
Doubled: [2, 4, 6]
Tripled: [3, 6, 9]

=== ALL TESTS COMPLETED ===
null
Running test: .\tests\async_and_arrays.rcc
=== TESTING ASYNC FUNCTIONS ===
Future: [Future: Resolved(Data fetched!)]
Result after await: Data fetched!

=== TESTING ARRAY METHODS ===
Original array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

--- map() ---
Doubled: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

--- filter() ---
Even numbers: [2, 4, 6, 8, 10]
Greater than 5: [6, 7, 8, 9, 10]

--- reduce() ---
Sum of all numbers: 55
Product of all numbers: 3628800

--- find() ---
First number > 7: 8
First number > 100: null

--- findIndex() ---
Index of 5: 4
Index of > 100: -1

--- some() ---
Has even numbers? true
Has negative numbers? false

--- every() ---
All positive? true
All even? false

=== METHOD CHAINING ===
Even numbers * 3, > 10: [12, 18, 24, 30]

--- Small array composition ---
Original: [1, 2, 3]
Doubled: [2, 4, 6]
Tripled: [3, 6, 9]

=== ALL TESTS COMPLETED ===
null
Running test: .\tests\comprehensive_stdlib_demo.rcc
Running test: .\tests\demo_colors.rcc
╔══════════════════════════════════════════════╗
║   DEMOSTRACIÓN DE COLORES EN RACCOON        ║
╔══════════════════════════════════════════════╗

🎨 TIPOS DE DATOS CON COLORES:
─────────────────────────────────────────────

📝 Strings (Verde):
  → Hola Mundo Raccoon

🔢 Números (Amarillo):
  Enteros → 42 100 -5
  Decimales → 3.14 2.71 0.5

✓ Booleanos (Amarillo):
  → true false

∅ Null (Gris):
  → null

📋 Listas (Magenta):
  → [1, 2, 3, 4, 5]
  → [a, b, c]


🎯 OPERACIONES Y RESULTADOS:
─────────────────────────────────────────────

a = 10 │ b = 20
a + b = 30
a * b = 200
a < b = true


✨ COMPARACIÓN:
─────────────────────────────────────────────

👤 Usuario: Alice
   Edad: 30 años
   Estado: Activo


╚══════════════════════════════════════════════╝
║   FIN DE LA DEMOSTRACIÓN                     ║
╚══════════════════════════════════════════════╝
null
Running test: .\tests\feature_array_destructuring.rcc
a = 1
b = 2
c = 3
Test 2: String Array Destructuring
hello world raccoon
Test 3: Float Array Destructuring
Pi: 3.14
E: 2.71
Sqrt2: 1.41
Test 4: Function Parameter Destructuring
Sum of first two: 30
Values: 100, 200, 300
Test 5: Block Scope Destructuring
First: 7, Second: 14
Test 6: Destructuring in Loops
[0] = 0
[1] = 10
[2] = 20
Test 7: Destructuring in Class Methods
AP-1 processing: 5, 10, 15
Result: 30
First value: 99
Test 8: Destructuring in Conditionals
Conditional values: 50, 60
Test 9: Nested Function Calls
Multiplied: 2, 4, 6
Test 10: Let vs Const Destructuring
Let: 1, 2
Const: 10, 20, 30
Test 11: For-in Loop with Arrays
Item: 100
Item: 200
Item: 300
Test 12: Range with Destructuring
Range values: 1, 2, 3
Test 13: Constructor with Destructuring
Vector created: [1, 2, 3]
Components: 3 items
=== ALL ARRAY DESTRUCTURING TESTS PASSED ===
✅ Basic array destructuring
✅ String/Float arrays
✅ Function parameters
✅ Block scope
✅ Loops
✅ Class methods
✅ Conditionals
✅ Nested calls
✅ Let vs Const
✅ For-in loops
✅ Ranges
✅ Constructors
null
Running test: .\tests\feature_arrow_functions.rcc
8
14
42
42
true
false
Hello, World!
12
19
true
8
15
All arrow function tests passed!
null
Running test: .\tests\feature_decorators.rcc
=== DECORATORS FEATURE TEST ===
Test 1: Simple Class Decorator
Value: 42
Test 2: Decorator with Single Argument
Test 3: Decorator with Multiple Arguments
Test 4: Multiple Decorators
Test 5: Decorators with String Arguments
Test 6: Decorators with Float Arguments
Test 7: Decorators with Inheritance
Test 8: Complex Decorator Logic
Size: 5
Test 9: Boolean Argument Decorators
Test 10: Decorator Ordering Test
=== ALL DECORATOR TESTS PASSED ===
✅ Simple decorators
✅ Decorators with arguments
✅ Multiple decorators
✅ String arguments
✅ Float arguments
✅ With inheritance
✅ Complex logic
✅ Boolean arguments
✅ Decorator ordering
null
Running test: .\tests\feature_object_destructuring.rcc
=== OBJECT DESTRUCTURING FEATURE TEST ===
Test 1: Global Variable Destructuring
x = 10
y = 20
x2 = 100
y2 = 200
Test 2: Block Scope Destructuring
Name: Alice
Age: 30
Test 3: Function Parameter Destructuring
Point(5, 15)
Hello Bob, you are 25 years old
Test 4: Destructuring in Conditionals
Resolution: 800x600
Test 5: Destructuring in While Loop
Loop 0: (0, 0)
Loop 1: (10, 20)
Test 6: Destructuring in Class Methods
Processor-1 processing: x=7, y=14
Combined result: 10
Test 7: Multiple Destructurings
User 1: user1 (user1@test.com)
User 2: user2 (user2@test.com)
Test 8: Destructuring with Function Returns
Created point: (42, 84)
Test 9: Nested Function Calls
Original: (3, 4)
Doubled sum: 14
Test 10: Destructuring in Try-Catch
Try block: Test, 99
=== ALL OBJECT DESTRUCTURING TESTS PASSED ===
✅ Global scope destructuring
✅ Block scope destructuring
✅ Function parameter destructuring
✅ Conditional destructuring
✅ Loop destructuring
✅ Class method destructuring
✅ Multiple destructurings
✅ With function returns
✅ Nested calls
✅ Try-catch blocks
null
Running test: .\tests\REFERENCE_COMPATIBILITY_TEST.rcc
╔════════════════════════════════════════════╗
║  RACCOON COMPATIBILITY REFERENCE TEST     ║
║  Version: 1.0.0                           ║
║  Date: October 2025                       ║
╚════════════════════════════════════════════╝
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 1: Object Destructuring
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1.1 Basic Object Destructuring
  ✓ x=10, y=20
1.2 Object Destructuring in Functions
  ✓ sum=20
1.3 Object Destructuring in Class Methods
  ✓ result=14
  ✅ Object Destructuring: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 2: Array Destructuring
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2.1 Basic Array Destructuring
  ✓ a=1, b=2, c=3
2.2 String Array Destructuring
  ✓ w1=hello, w2=world
2.3 Array Destructuring in Function Parameters
  ✓ arraySum=60
2.4 Array Destructuring in Constructors
  ✓ magnitude=7
  ✅ Array Destructuring: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 3: Decorators
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3.1 Simple Class Decorator
3.2 Decorator with Arguments
3.3 Multiple Decorators
  ✅ Decorators: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 4: Feature Integration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4.1 Decorators + Object Destructuring
  ✓ Authenticating user: admin
4.2 Decorators + Array Destructuring
  ✓ Processing: min=10, max=100, avg=55
4.3 All Features Combined
  ✓ Querying users with limit 100
  ✓ Batch inserting 10 records of size 1024
  ✓ Total bytes: 10240
  ✅ Feature Integration: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 5: Edge Cases & Stress Tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
5.1 Nested Object Access
  ✓ Location: NYC, USA
5.2 Destructuring in Loops
  ✓ Loop[0]: 0
  ✓ Loop[1]: 10
5.3 Destructuring with Ranges
  ✓ Range values: 1, 2, 3
5.4 Multiple Destructurings
  ✓ P1(1,2), P2(3,4)
5.5 Decorator with Complex Arguments
  ✓ Service call executed
  ✅ Edge Cases: PASSED
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SECTION 6: Backward Compatibility
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
6.1 Regular Classes
  ✓ Old style class value: 100
6.2 Regular Functions
  ✓ Regular function result: 15
6.3 Regular Variables
  ✓ Normal var: 42, const: constant
  ✅ Backward Compatibility: PASSED
╔════════════════════════════════════════════╗
║           COMPATIBILITY REPORT            ║
╚════════════════════════════════════════════╝
✅ SECTION 1: Object Destructuring ......... PASSED
✅ SECTION 2: Array Destructuring .......... PASSED
✅ SECTION 3: Decorators ................... PASSED
✅ SECTION 4: Feature Integration .......... PASSED
✅ SECTION 5: Edge Cases ................... PASSED
✅ SECTION 6: Backward Compatibility ....... PASSED
╔════════════════════════════════════════════╗
║    ALL COMPATIBILITY TESTS PASSED ✅      ║
║                                           ║
║  Object Destructuring:    100% ✅         ║
║  Array Destructuring:     100% ✅         ║
║  Decorators:              100% ✅         ║
║  Feature Integration:     100% ✅         ║
║  Edge Cases:              100% ✅         ║
║  Backward Compatibility:  100% ✅         ║
║                                           ║
║  OVERALL STATUS:          EXCELLENT ✅    ║
╚════════════════════════════════════════════╝
null
Running test: .\tests\syntax_arrays.rcc
=== ARRAYS TEST ===
Test 1: Array Declaration
Int array length: 5
String array length: 3
Float array length: 3
Bool array length: 3
Test 2: Array Access
First element: 10
Second element: 20
Last element: 50
Test 3: Array Modification
Before: 1
After: 100
Test 4: Empty Array
Empty array length: 0
Test 5: Array with Variables
Array from vars: 1, 2, 3
Test 6: Nested Arrays
Matrix[0][0]: 1
Matrix[1][1]: 4
Matrix[2][0]: 5
Test 7: Array Iteration
Fruit: apple
Fruit: banana
Fruit: cherry
Test 8: Array from Range
Range 1..5 length: 5
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
Test 9: Array Push
Before push: 2
After push: 3
Last element: 3
Test 10: Array Pop
Before pop: 3
Popped value: 30
After pop: 2
Test 11: String as Character Array
String length: 5
First char: h
Last char: o
Test 12: Array Operations
Combined length: 4
Test 13: Arrays in Functions
First: 100
Last: 300
Test 14: Multi-dimensional Arrays
Grid[0][0]: 1
Grid[1][1]: 5
Grid[2][2]: 9
Test 15: Length Property
Empty: 0
After 1 push: 1
After 3 pushes: 3
=== ALL ARRAYS TESTS PASSED ===
✅ Array declaration
✅ Array access
✅ Array modification
✅ Empty arrays
✅ Arrays from variables
✅ Nested arrays
✅ Array iteration
✅ Range creation
✅ Push method
✅ Pop method
✅ String indexing
✅ Array concatenation
✅ Arrays in functions
✅ Multi-dimensional access
✅ Length property
null
Running test: .\tests\syntax_arrow_functions.rcc
10
30
42
42
75
true
false
Hello, TypeScript
25
11
13
true
false
8
15
15
25
49
103
35
7
12
All TypeScript-style arrow function tests passed!
null
Running test: .\tests\syntax_bitwise_operators.rcc
8
14
6
-6
20
5
4611686018427387902
8
512
8
14
6
20
5
8
null
Running test: .\tests\syntax_classes.rcc
=== CLASSES TEST ===
Test 1: Basic Class
Name: Alice
Age: 30
Test 2: Class Methods
Initial: 10
After add(5): 15
Test 3: Multiple Methods
After 2 increments: 2
After 1 decrement: 1
Test 4: String Fields
1984 by Orwell
Pages: 328
Test 5: Boolean Fields
Status: OFF
After toggle: ON
Test 6: Array Fields
Student count: 2
Test 7: Method Return Values
Area: 15
Perimeter: 16
Test 8: Inheritance
Dog name: Rex
Breed: Labrador
Sound: Woof!
Test 9: Methods with Parameters
max(10, 20): 20
min(10, 20): 10
Test 10: Multiple Instances
p1: (0, 0)
p2: (10, 20)
=== ALL CLASSES TESTS PASSED ===
✅ Basic class creation
✅ Class methods
✅ Multiple methods
✅ String fields
✅ Boolean fields
✅ Array fields
✅ Method returns
✅ Inheritance
✅ Static methods
✅ Multiple instances
null
Running test: .\tests\syntax_control_flow.rcc
=== CONTROL FLOW TEST ===
Test 1: If Statement
x is greater than 5
Test 2: If-Else Statement
Adult
18 or older
Test 3: If-Else Chain
Grade: B
Test 4: Nested If
Positive and greater than 10
Test 5: While Loop
Counter: 0
Counter: 1
Counter: 2
Test 6: While with Break
i: 0
i: 1
i: 2
Test 7: While with Continue
j: 1
j: 2
j: 4
j: 5
Test 8: For-In Loop with Array
Number: 10
Number: 20
Number: 30
Test 9: For-In Loop with Range
Range: 1
Range: 2
Range: 3
Range: 4
Range: 5
Test 10: Nested Loops
(0, 0)
(0, 1)
(1, 0)
(1, 1)
Test 11: Complex Conditions
a < b < c is true
At least one condition is true
a is not greater than b
Test 12: Early Return Pattern
checkValue(-5): negative
checkValue(0): zero
checkValue(10): positive
=== ALL CONTROL FLOW TESTS PASSED ===
✅ If statement
✅ If-else statement
✅ If-else chain
✅ Nested if
✅ While loop
✅ Break statement
✅ Continue statement
✅ For-in with arrays
✅ For-in with ranges
✅ Nested loops
✅ Complex conditions
✅ Early returns
null
Running test: .\tests\syntax_error_handling.rcc
=== ERROR HANDLING TEST ===
Test 1: Basic Try-Catch
Inside try block
Caught: Basic error
Test 2: Normal Flow
This executes normally
x = 10
Test 3: String Error
Error message: Something went wrong!
Test 4: Try-Catch in Function
Result: error: Function error
Test 5: Nested Try-Catch
Outer try
Inner try
Inner catch: Inner error
After inner try-catch
Test 6: Multiple Statements
Sum: 3
Caught: Error after calculations
Test 7: Try-Catch with Return
10 % 2 = 0
Error: Division by zero
10 % 0 = 0
Test 8: Variable Scope
Status: error
Test 9: Try-Catch in Loop
i = 0
Caught: Error at i=1
i = 2
Test 10: Conditional Throw
Valid age
Invalid: Age cannot be negative
Invalid: Age too high
Test 11: Multiple Throws
Value is valid
Validation error: Negative
Validation error: Zero
Validation error: Too large
Test 12: Error Propagation
caught: Inner function error
=== ALL ERROR HANDLING TESTS PASSED ===
✅ Basic try-catch
✅ Normal flow
✅ String errors
✅ Try-catch in functions
✅ Nested try-catch
✅ Multiple statements
✅ Try-catch with return
✅ Variable scope
✅ Try-catch in loops
✅ Conditional throws
✅ Multiple throws
✅ Error propagation
null
Running test: .\tests\syntax_functions.rcc
=== FUNCTIONS TEST ===
Test 1: Basic Function
Hello from function!
Test 2: Function with Parameters
5 + 3 = 8
Test 3: Multiple Parameters
Name: Alice, Age: 25, City: NYC
Test 4: Return Values
6 * 7 = 42
20 / 4 = 5
Test 5: Default Parameters
Hello, Alice!
Hello, Guest!
Test 6: Recursive Function
factorial(5) = 120
Test 7: Function Composition
quadruple(5) = 20
Test 8: String Return Functions
Full name: John Doe
Shouted: HELLO
Test 9: Boolean Return Functions
isEven(4): true
isEven(7): false
isPositive(-5): false
isPositive(10): true
Test 10: Array Parameters
Sum of array: 15
Test 11: Array Return
Range length: 5
Test 12: Nested Calls
process(5) = 8
Test 13: Float Functions
average(5.0, 10.0) = 7.5
Test 14: Void Functions
LOG: Test message
=== ALL FUNCTIONS TESTS PASSED ===
✅ Basic functions
✅ Parameters
✅ Multiple parameters
✅ Return values
✅ Default parameters
✅ Recursion
✅ Function composition
✅ String returns
✅ Boolean returns
✅ Array parameters
✅ Array returns
✅ Nested calls
✅ Float functions
✅ Void functions
null
Running test: .\tests\syntax_interfaces.rcc
=== INTERFACES TEST ===
Test 1: Interface Definition
Name: Alice
Test 2: Interface with Methods
Drawing circle with radius 5
Test 3: Multiple Members
Area: 20
Test 4: Multiple Interfaces
ID: 1
Product: Laptop
Test 5: Interface with Boolean
Initial: false
After toggle: true
Test 6: Multiple Methods
Counter value: 2
Test 7: Return Types
Result: 30
Test 8: Empty Interface
Marked value: 42
Test 9: String Methods
Formatted: 15/3/2024
Test 10: Implementation Chain
Bob, 35, Developer
=== ALL INTERFACES TESTS PASSED ===
✅ Basic interface
✅ Interface with methods
✅ Multiple members
✅ Multiple interfaces
✅ Boolean interfaces
✅ Multiple methods
✅ Return types
✅ Empty interfaces
✅ String methods
✅ Implementation chains
null
Running test: .\tests\syntax_interfaces_simple.rcc
=== INTERFACES TEST (SIMPLIFIED) ===
Test 1: Basic Interface
✓ Interface Point defined
Test 2: Multiple Interfaces
✓ Multiple interfaces defined
Test 3: Interface with Different Types
✓ Interface with multiple types defined
Test 4: Classes Following Interface Structure
Area: 50
=== ALL INTERFACE TESTS PASSED ===
✅ Basic interface definitions
✅ Multiple interfaces
✅ Interfaces with multiple types
✅ Classes following interface structure
Note: Interfaces only support properties, not methods
Note: 'implements' keyword is not currently enforced
null
Running test: .\tests\syntax_operators.rcc
=== OPERATORS TEST ===
Test 1: Arithmetic Operators
Addition: 13
Subtraction: 7
Multiplication: 30
Division: 3.3333333333333335
Modulo: 1
Test 2: Comparison Operators
Equal: true
Not equal: true
Less than: true
Greater than: true
Less or equal: true
Greater or equal: true
Test 3: Logical Operators
AND (true && true): true
AND (true && false): false
OR (false || true): true
OR (false || false): false
NOT (!true): false
NOT (!false): true
Test 4: Assignment Operators
Initial: 10
After +5: 15
After -3: 12
After *2: 24
After -4: 20
Test 5: Compound Assignment
num += 10: 110
num -= 5: 105
num *= 2: 210
num -= 10: 200
Test 6: Unary Operators
Negation: -42
Logical NOT: false
Test 7: String Concatenation
Concatenation: Hello World
Mixed: Raccoon v1
Test 8: Operator Precedence
2 + 3 * 4 = 14
(2 + 3) * 4 = 20
10 - 2 + 3 = 11
Test 9: Boolean Expressions
Is adult (25 >= 18): true
Passed (85 in range): true
Can edit: true
Test 10: Null Comparison
Is null: true
Is not null: true
=== ALL OPERATORS TESTS PASSED ===
✅ Arithmetic operators
✅ Comparison operators
✅ Logical operators
✅ Assignment operators
✅ Compound assignment
✅ Unary operators
✅ String concatenation
✅ Operator precedence
✅ Boolean expressions
✅ Null comparison
null
Running test: .\tests\syntax_primitive_types.rcc
=== PRIMITIVE TYPES TEST ===
Test 1: Integer Type
Positive: 42
Negative: -100
Zero: 0
Max: 2147483647
Min: -2147483648
Test 2: Float Type
Pi: 3.14
Negative: -2.71
Zero: 0
E: 2.71828
Test 3: String Type
String 1: Hello World
String 2: Raccoon
Empty: ''
Greeting: ¡Hola!
Test 4: Boolean Type
True: true
False: false
Constant: true
Test 5: Null Type
Null value created
Assigned value: 42
Test 6: Type Conversions
Int to Float: 10
Int to String: 10
Test 7: String Operations
Upper: HELLO
Lower: hello
Split length: 3
Test 8: Type Checking
typeof num: int
typeof msg: str
Test 9: Constants
PI: 3.14159
Version: 1
App: Raccoon
Test 10: Mixed Type Operations
Int + Float: 7.5
Number: 42
=== ALL PRIMITIVE TYPES TESTS PASSED ===
✅ Integer type
✅ Float type
✅ String type
✅ Boolean type
✅ Null type
✅ Type conversions
✅ String operations
✅ Type checking
✅ Constants
✅ Mixed operations
null
Running test: .\tests\syntax_scopes.rcc
=== SCOPES TEST ===
Test 1: Global Scope
Global: 100
Test 2: Block Scope
Inside block: 2
Access outer: 1
Outside block: 1
Test 3: Function Scope
Inside function: 20
Access outer: 10
Outside function: 10
Test 4: Variable Shadowing
Outer x: 1
Inner x: 2
Outer x again: 1
Test 5: Loop Scope
Loop iteration 0: 0
Loop iteration 1: 10
Loop iteration 2: 20
After loop: 5
Test 6: If Statement Scope
Inside if: 42
After if block
Test 7: Nested Blocks
Level 3: 3
Access level 2: 2
Access level 1: 1
Level 2: 2
Level 1: 1
Test 8: Reassignment
Initial: 10
After reassignment: 20
After calculation: 25
Test 9: Constants
Constant value: 100
PI: 3.14159
Test 10: For-In Scope
Number: 10
Number: 20
Number: 30
After for-in loop
Test 11: Class Field Scope
Counter: 1
Test 12: Try-Catch Scope
Try block: 100
Catch block: caught
Test 13: Multiple Variables
a=1, b=2, c=3
Test 14: Variable Lifetime
First call: 1
Second call: 1
Test 15: Scope Chain
Access all: 1, 2, 3
=== ALL SCOPES TESTS PASSED ===
✅ Global scope
✅ Block scope
✅ Function scope
✅ Variable shadowing
✅ Loop scope
✅ If statement scope
✅ Nested blocks
✅ Reassignment
✅ Constants
✅ For-in scope
✅ Class field scope
✅ Try-catch scope
✅ Multiple variables
✅ Variable lifetime
✅ Scope chain
null
Running test: .\tests\syntax_static_methods.rcc
=== STATIC METHODS TEST ===
Test 1: Basic Static Method
max(10, 20): 20
min(10, 20): 10
Test 2: Multiple Parameters
add(1, 2, 3): 6
multiply(2, 3, 4): 24
Test 3: String Return
HelloWorld
xxx
Test 4: Mix Static and Instance
Counter value: 1
Test 5: Factory Pattern
Origin: (0, 0)
From array: (10, 20)
=== ALL STATIC METHOD TESTS PASSED ===
✅ Basic static methods
✅ Multiple parameters
✅ String return types
✅ Mixed static and instance
✅ Factory pattern
null
Running test: .\tests\syntax_templates.rcc
=== TEMPLATE STRINGS TEST ===
Test 1: Basic Template
Hello, World!
Test 2: Integer Interpolation
I am 25 years old
Test 3: Float Interpolation
Price: $19.99
Test 4: Boolean Interpolation
Status: true
Test 5: Multiple Variables
Name: John Doe, Age: 30
Test 6: Expression Interpolation
5 + 3 = 8
Test 7: Templates in Functions
Hello Alice, you are 28 years old
Test 8: Object Properties
My name is Bob and I'm 35
Test 9: Array Properties
Array has 5 items
Test 10: Comparison Results
x < y is true
Test 11: Complex Expressions
Sum of 5 and 10 is 15
Test 12: Function Call Interpolation
Double of 7 is 14
Test 13: Template Concatenation
Hello World
Test 14: Empty Templates
Empty string length: 0
Test 15: Templates in Conditionals
Grade: B (85)
=== ALL TEMPLATE STRINGS TESTS PASSED ===
✅ Basic templates
✅ Integer interpolation
✅ Float interpolation
✅ Boolean interpolation
✅ Multiple variables
✅ Expressions
✅ Function templates
✅ Object properties
✅ Array properties
✅ Comparisons
✅ Complex expressions
✅ Function calls
✅ Concatenation
✅ Empty templates
✅ Conditional templates
null
Running test: .\tests\test_array_methods.rcc
Original array:
Doubled:
Evens:
Sum:
ForEach:
  -
  -
  -
  -
  -
First > 3:
Index of first > 3:
Has > 10:
Has < 3:
All positive:
All > 10:
All tests completed!
null
Running test: .\tests\test_arrow.rcc
[2, 4, 6]
null
Running test: .\tests\test_arrow2.rcc
[2, 4, 6]
null
Running test: .\tests\test_arrow_complete.rcc
=== ARROW FUNCTIONS TEST ===

1. Arrow function with map:
Doubled: [2, 4, 6, 8, 10]

2. Arrow function with filter:
Even numbers: [2, 4]

3. Arrow function with reduce:
Sum: 15

4. Arrow function with type annotations:
Squared: [1, 4, 9, 16, 25]

5. Method chaining:
Filtered > 2, then * 10: [30, 40, 50]

6. Arrow function as variable:
Tripled: [3, 6, 9, 12, 15]

=== ALL TESTS PASSED ===
null
Running test: .\tests\test_arrow_notype.rcc
14
null
Running test: .\tests\test_arrow_simple.rcc
8
null
Running test: .\tests\test_arrow_single.rcc
14
null
Running test: .\tests\test_assign_error.rcc
Running test: .\tests\test_builtins.rcc
Running test: .\tests\test_classes_advanced.rcc
🦝 === ADVANCED CLASSES TEST ===
✅ Test 1: Static Methods
  max(10, 20): 20
  min(10, 20): 10
  abs(-15): 15
  abs(15): 15
✅ Test 2: Getters and Setters
  Celsius: 25
  Fahrenheit: 77
  Kelvin: 298.15
  After setting to 68°F:
  Celsius: 20
✅ Test 3: Method Overriding
  Circle with radius 5
  Area: 78.53975
  Rectangle 4x6
  Area: 24
✅ Test 4: Encapsulation Pattern
  Initial: 0/3
  After increment: 1
  After 2 more increments: 3
  Can increment more: false
  After reset: 0
✅ Test 5: Complex Constructor
  User: john_doe
  Email: john@example.com
  Status: Active (0 logins)
  After 3 logins: Active (3 logins)
  After deactivation: Inactive
✅ Test 6: Two-Level Inheritance
  Car: Toyota (4 doors)
  Brand: Toyota
  Doors: 4
✅ Test 7: Complex State Management
  Cart empty: true
  Items in cart: 3
  Total: $12.25
✅ Test 8: Methods with Multiple Returns
  Empty string: Error: Value is empty
  'ab': Error: Value too short
  'hello': Valid
  Long string: Error: Value too long
✅ Test 9: Instance Type Checking
  Animal name: Generic
  Cat name: Whiskers
  Cat says: Meow!
✅ Test 10: Multiple Instances Stress Test
  Created 5 points
  Point 0: (0, 0)
  Point 1: (3, 4)
  Point 2: (5, 12)
🎉 === ALL ADVANCED TESTS PASSED ===
✅ Static methods
✅ Getters and setters
✅ Method overriding in inheritance
✅ Encapsulation patterns
✅ Complex constructor logic
✅ Two-level inheritance
✅ Complex state management
✅ Multiple return points
✅ Instance type checking
✅ Multiple instances stress test
🦝 Raccoon Classes: ALL ADVANCED FEATURES WORKING!
null
Running test: .\tests\test_classes_comprehensive.rcc
🦝 === COMPREHENSIVE CLASSES TEST ===
✅ Test 1: Basic Class with Constructor
  Name: Alice
  Age: 30
✅ Test 2: Class with Methods
  Initial value: 10
  After add(5): 15
  After multiply(2): 30
  After subtract(10): 20
  After reset: 0
✅ Test 3: Class with Different Data Types
  Laptop - $999.99 (Stock: 5)
  Total value: $4999.95
  In stock: true
  After selling 3: Laptop - $999.99 (Stock: 2)
  After selling 2 more: Laptop - $999.99 (Stock: 0)
  In stock: false
✅ Test 4: Class with Array Properties
  Classroom: Math 101
  Students: 3
  Average grade: 91.33333333333333
✅ Test 5: Class Inheritance
  Rex is 3 years old
  Rex is a Labrador
  Sound: Woof! Woof!
✅ Test 6: Multiple Instances Independence
  Counter A: 3
  Counter B: 1
✅ Test 7: Complex Methods with Logic
  Account: ACC001
  Initial balance: $1000
  After deposit $500: $1500
  After withdraw $200: $1300
✅ Test 8: Nested Method Calls
  Initial: 5
  After add(3) then multiply(2): 16
✅ Test 9: Class with String Operations
  Original: Hello Raccoon
  Uppercase: HELLO RACCOON
  Lowercase: hello raccoon
  Length: 13
  Is empty: false
✅ Test 10: Class with Object Composition
  John Doe (ID: 1001)
  John Doe (ID: 1001) - 123 Main St, Springfield 12345
🎉 === ALL TESTS PASSED ===
✅ Basic class with constructor
✅ Class with methods
✅ Different data types (str, int, float, bool)
✅ Array properties
✅ Class inheritance (extends, super)
✅ Multiple instances independence
✅ Complex methods with logic
✅ Nested method calls
✅ String operations in classes
✅ Object composition
🦝 Raccoon Classes: FULLY FUNCTIONAL!
null
Running test: .\tests\test_debugging_features.rcc
=== TEST DE DEBUGGING Y COLORES ===

Test 1: print() con colores para diferentes tipos
Strings en verde: Hello World
Números en amarillo: 42 3.14
Booleanos en amarillo: true false
Null en gris: null
Lista en magenta: [1, 2, 3]

Test 2: función println()
Primera línea
Segunda línea
Tercera línea

Test 3: Mezcla de tipos en una sola llamada
Usuario: Bob Edad: 25 Activo: true Score: 95.5

Test 4: Resultados de operaciones
x = 10 y = 5
x + y = 15
x * y = 50
x > y = true

Test 5: Estructuras de datos
Lista de números: [10, 20, 30, 40, 50]
Lista mixta: [1, two, 3, four]

Test 6: Funciones
Resultado: Hello, World

=== TODOS LOS TESTS COMPLETADOS ===
null
Running test: .\tests\test_decorators_advanced.rcc
🦝 === ADVANCED DECORATORS TEST ===
✅ Test 1: Method Decorators
  Calculator value: 15
✅ Test 2: Timing Decorator Pattern
  [TIMER END] Result: 5050
✅ Test 3: API Route Decorator Pattern
  List of users
  User created
✅ Test 4: Database Entity Decorator
  Product: Laptop - $999.99
✅ Test 5: Validation Decorator
  Age set to 25
  Name set to John
✅ Test 6: Memoization Pattern
  Result 1: 25
  Result 2: 169
✅ Test 7: Singleton Pattern
  Connected to: localhost:5432
✅ Test 8: Rate Limiting Decorator
  API response from /api/data
✅ Test 9: Transaction Decorator
  Transferred $100
  [TRANSACTION] Commit
✅ Test 10: Error Handling Decorator
  Success
✅ Test 11: AOP Pattern - Before/After
  [BEFORE] Executing...
  Processed: important data
  [AFTER] Done
✅ Test 12: Feature Flag Decorator
  Displaying new UI
✅ Test 13: Dependency Injection
  UserCtrl handling request
✅ Test 14: Event Emitter Pattern
  Created user: alice
✅ Test 15: Multi-Decorator Composition
  Executed: delete_database
🎉 === ALL ADVANCED DECORATOR TESTS PASSED ===
✅ Method decorators
✅ Timing decorator pattern
✅ API route decorator
✅ Database entity decorator
✅ Validation decorator
✅ Memoization pattern
✅ Singleton pattern
✅ Rate limiting
✅ Transaction decorator
✅ Error handling
✅ AOP pattern
✅ Feature flags
✅ Dependency injection
✅ Event emitters
✅ Multi-decorator composition
🦝 Raccoon Advanced Decorators: PRODUCTION READY!
null
Running test: .\tests\test_decorators_comprehensive.rcc
🦝 === COMPREHENSIVE DECORATORS TEST ===
✅ Test 1: Basic Function Decorator
  Result: Hello, Alice
✅ Test 2: Function Decorator with Arguments
  Sum: 8
✅ Test 3: Class Decorator
  User greeting: Hello, Bob
✅ Test 4: Multiple Decorators on Function
  Result: Function with multiple decorators
✅ Test 5: Decorator with Multiple Arguments
  Percentage: 75%
✅ Test 6: Class Decorator with Arguments
  Charlie is 30 years old
✅ Test 7: Performance Tracking Decorator
Running test: .\tests\test_error_display.rcc
Iniciando test...
Running test: .\tests\test_error_positions.rcc
Running test: .\tests\test_explicit_generics.rcc
42
100
400
25
Hello
All tests passed!
null
Running test: .\tests\test_func_type.rcc
=== FUNC TYPE TESTS ===

[ 1 ] Basic func type assignment
  Result: Hello, Alice

[ 2 ] func as function parameter
  square(5) = 25
  double(5) = 10

[ 3 ] Arrays of func
  Applying operations to 3:
    operations[0](3) = 9
    operations[1](3) = 6

[ 4 ] func with arrow functions
  add(10, 5) = 15
  subtract(10, 5) = 5

[ 5 ] func returning func
  addOp(4, 3) = 7
  mulOp(4, 3) = 12

[ 6 ] func with different return types
  toString(42) = 42
  toBool(5) = true
  toBool(-1) = false

[ 7 ] func with higher-order array methods
  Original: [1, 2, 3, 4, 5]
  Mapped (increment): [2, 3, 4, 5, 6]
  Filtered (isEven): [2, 4]
  Reduced (sum): 15

[ 8 ] Multiple func variables
  addFunc(10, 5) = 15
  subFunc(10, 5) = 5
  mulFunc(10, 5) = 50

[ 9 ] func with async functions
  Async result: Data_123

==================================================
✅ ALL FUNC TYPE TESTS PASSED
==================================================

The 'func' type works as a generic function type!
It can represent any function signature dynamically.
null
Running test: .\tests\test_generics.rcc
=== GENERICS SYSTEM TEST ===
Test 1: Generic Box<T>
Int box: 42
String box: Hello
Updated int box: 100
Test 2: Generic Pair<K, V>
Pair: age = 25
Test 3: Generic with Constraint
Container size: 3
First item: 10
Test 4: Optional<T> Type
Some has value: true
None has value: false
Some value: 42
None default: 0
Test 5: Result<T> Type
Good result success: true
Good result value: 5
Bad result success: false
Bad result error: Division by zero
Test 6: Stack<T>
Top: third
Pop: third
Pop: second
Is empty: false
=== ALL GENERIC TESTS PASSED ===
null
Running test: .\tests\test_generics_simple.rcc
=== GENERICS SYSTEM TEST (Simplified) ===
Test 1: Generic Box<T>
Int box: 42
String box: Hello
Updated int box: 100
Test 2: Generic Pair<K, V>
Pair: age = 25
Test 3: Generic Container<T>
Container size: 3
First item: 10
Test 4: Optional<T> Type
Some has value: true
None has value: false
Some value: 42
None default: 0
Test 5: Stack<T>
Top: third
Pop: third
Pop: second
Is empty: false
=== ALL GENERIC TESTS PASSED ===
null
Running test: .\tests\test_generics_vs_bitwise.rcc
42
32
10
20
All tests passed!
null
Running test: .\tests\test_http_debug.rcc
Running test: .\tests\test_http_final.rcc
Running test: .\tests\test_http_minimal.rcc
Running test: .\tests\test_http_net.rcc
Running test: .\tests\test_http_simple.rcc
Testing HTTP with default import...
Response: { status: 200, statusText: OK, headers: Map { date: Tue, 21 Oct 2025 04:45:47 GMT, age: 2301972, last-modified: Wed, 24 Sep 2025 13:19:34 GMT, x-vercel-cache: HIT, content-length: 17874, strict-transport-security: max-age=63072000, x-vercel-id: sfo1::zs7g6-1761021947651-6f0e56026cf6, access-control-allow-origin: *, content-type: text/html; charset=utf-8, accept-ranges: bytes, etag: "a096f8b2a3cc43cd18ec5f9eb5ebd7ba", server: Vercel, cache-control: public, max-age=0, must-revalidate, content-disposition: inline }, body: null, url: https://coatl.in/ }

Success! Default import working!
null
Running test: .\tests\test_import_default.rcc
Testing default import...
Http: class Http
null
Running test: .\tests\test_index_access_assignment.rcc
Testing index access and assignment for objects...

Test 1: Reading properties with index notation
  config["api-key"]: secret123
  config["base-url"]: https://api.example.com
  config["timeout"]: 5000
  ✓ Index access works!

Test 2: Assigning properties with index notation
  After assignment:
    config["api-key"]: new-secret-456
    config["version"]: v2.0
    config["timeout"]: 10000
  ✓ Index assignment works!

Test 3: Mixed access patterns
  user.name: Carlos
  user["user-id"]: 12345
  user.email: carlos@example.com

  After updates:
  user.name: Carlos López
  user["user-id"]: 99999
  user.email: new@example.com
  ✓ Mixed access works!

Test 4: Dynamic property access
  data[propName]: value2
  After update: updated-value2
  ✓ Dynamic property access works!

All tests passed! Index access and assignment work correctly.
null
Running test: .\tests\test_interface.rcc
10
null
Running test: .\tests\test_map_complete.rcc
=== Map<K,V> Implementation Tests ===

[ 1 ] Basic Map<str, int> operations:
  Set 3 values
  Get 'one': 1
  Get 'two': 2
  Has 'one': true
  Has 'missing': false
  Size: 3
  ✓ Test 1 passed

[ 2 ] Map delete operation:
  Initial size: 3
  Deleted 'b': true
  Size after delete: 2
  Has 'b': false
  Deleted non-existent 'xyz': false
  ✓ Test 2 passed

[ 3 ] Map clear operation:
  Size before clear: 3
  Size after clear: 0
  Has 'x': false
  ✓ Test 3 passed

[ 4 ] Map with float values:
  pi = 3.14159
  ✓ Test 4 passed

[ 5 ] Map<int, str> with int keys:
  1 -> one
  2 -> two
  Size: 3
  ✓ Test 5 passed

[ 6 ] Map overwrites existing values:
  Initial value: 100
  After overwrite: 200
  Size (should be 1): 1
  ✓ Test 6 passed

[ 7 ] Get non-existent key returns null:
  Non-existent key returns null ✓
  ✓ Test 7 passed

[ 8 ] Map toStr() method:
  Map as string: Map { a: 1, b: 2 }
  ✓ Test 8 passed

[ 9 ] Map with typed integers:
  i32 value: 12345
  u64 value: 999999
  ✓ Test 9 passed

[ 10 ] Null coalescing with Map.get():
  Existing key (with ??): 123
  Missing key (with ??): -1
  ✓ Test 10 passed

[ 11 ] Complex operation sequence:
  Added 5 elements, size: 5
  Deleted 2 elements, size: 3
  Added 1 more element, size: 4
  Has 'a': true
  Has 'b': false
  Has 'f': true
  ✓ Test 11 passed

[ 12 ] Map with float types:
  f32 size: 2
  f64 size: 2
  ✓ Test 12 passed

==================================================
✅ ALL MAP TESTS PASSED SUCCESSFULLY!
==================================================

Types tested:
  • Map<str, int>
  • Map<str, float>
  • Map<int, str>
  • Map<str, i32>
  • Map<str, u64>
  • Map<str, f32>
  • Map<str, f64>

Operations tested:
  • set(key, value)
  • get(key) -> value?
  • has(key) -> bool
  • delete(key) -> bool
  • clear()
  • size() -> int
  • toStr() -> str

🎉 Map<K,V> implementation working perfectly!
null
Running test: .\tests\test_map_i32.rcc
Creating Map<str, int>...
Success with int!
Creating Map<str, i32>...
Success with i32!
null
Running test: .\tests\test_map_minimal.rcc
null
Running test: .\tests\test_map_no_main.rcc
Testing Map basic operations

Map created
Value set
v1: 1
v2: 2
v3: 3
Has 'one': true
Has 'missing': false
Size: 3
Deleted 'two': true
Size after delete: 2
Size after clear: 0

✓ All Map tests passed!
null
Running test: .\tests\test_modules.rcc
Running test: .\tests\test_named_args.rcc
=== Test de argumentos nombrados ===

Llamada posicional:
Name: Alice
Age: 30
City: Madrid

Llamada con argumentos nombrados:
Name: Bob
Age: 25
City: Barcelona

Llamada mixta:
Name: Charlie
Age: 35
City: Valencia

=== Test completado ===
null
Running test: .\tests\test_named_args_comprehensive.rcc
=== Test Completo de Argumentos Nombrados ===

Test 1: Argumentos nombrados en orden diferente
Host: localhost Port: 8080 SSL: true
Host: example.com Port: 3000 SSL: false

Test 2: Valores por defecto
Name: Alice Age: 18 Active: true
Name: Bob Age: 25 Active: true
Name: Charlie Age: 18 Active: false
Name: Diana Age: 30 Active: false

Test 3: Mezcla de posicionales y nombrados
From: Madrid To: Barcelona Date: 2024-01-15 Class: economy
From: Madrid To: Barcelona Date: 2024-01-15 Class: business
From: Madrid To: Barcelona Date: 2024-01-15 Class: economy

Test 4: Funciones con retorno
5 + 3 = 8
5 * 3 = 15
5 * 3 = 15

Test 5: Diferentes tipos de datos
Message 1: HELLO HELLO HELLO 
Message 2: world world 

Test 6: Solo argumentos nombrados
Debug: true Verbose: false LogLevel: info
Debug: false Verbose: true LogLevel: debug

=== Todos los tests completados exitosamente ===
null
Running test: .\tests\test_net.rcc
Running test: .\tests\test_new_types.rcc
=== Tipos Enteros con Signo ===
i8:  -128
i16: -32768
i32: -2147483648
i64: -9223372036854776000

=== Tipos Enteros sin Signo ===
u8:  255
u16: 65535
u32: 4294967295
u64: 9223372036854776000

=== Tipos Punto Flotante ===
f32:     3.14159
f64:     2.718281828459045
decimal: 123.456789

=== Conversiones Automáticas ===
i8 -> i64:     -128
f32 -> f64:    3.14159
i32 -> decimal: -2147483648

=== Resultados de Funciones ===
addI32(10, 20):     30
multiplyF64(3.14, 2.0): 6.28
Decimal: 123.456789

=== Arrays con Nuevos Tipos ===
i32[]: [1, 2, 3, 4, 5]
f64[]: [1.1, 2.2, 3.3, 4.4, 5.5]
decimal[]: [10.5, 20.75, 30.25]

✅ Todos los tipos funcionan correctamente!
null
Running test: .\tests\test_no_conflict_generics_bitwise.rcc
16
64
8
6
40
10
All tests passed!
null
Running test: .\tests\test_null_safety_operators.rcc
====================================
NULL SAFETY OPERATORS TEST SUITE
====================================
✓ Test 1 passed: Null coalescing operator works
✓ Test 2 passed: Chained null coalescing works
⊘ Test 3 skipped: Inline object types not yet supported
⊘ Test 4 skipped: Inline object types not yet supported
✓ Test 5 passed: Null assertion on non-null value works
✓ Test 6 passed: Basic ternary operator works
✓ Test 7 passed: Nested ternary operators work
✓ Test 8 passed: Prefix increment works
✓ Test 9 passed: Postfix increment works
✓ Test 10 passed: Prefix decrement works
✓ Test 11 passed: Postfix decrement works
✓ Test 12 passed: Increment in for loop works
✓ Test 13 passed: Combined operators work
✓ Test 14 passed: Ternary in complex expression works
✓ Test 15 passed: Float increment/decrement works
====================================
TEST SUITE COMPLETE
====================================
null
Running test: .\tests\test_null_safety_simple.rcc
=== Test 1: Null Coalescing ===
default
=== Test 2: Conditional ===
adult
=== Test 3: Prefix Increment ===
6
6
=== Test 4: Postfix Increment ===
6
5
=== Test 5: Null Assertion ===
hello
null
Running test: .\tests\test_object_string_keys.rcc
Testing string literals as object keys...
✓ Created object with string literal keys
  Content-Type: application/json
  Authorization: Bearer token123
  X-Custom-Header: custom value

✓ Created object with mixed key types
  name: Carlos
  full-name: Carlos López
  age: 25
  user-id: 12345

✓ Created nested object with string keys
  API base URL: https://api.example.com
  API timeout: 5000
  Content-Type: application/json

All tests passed! String literals as object keys work correctly.
null
Running test: .\tests\test_raccoon_modules.rcc
null
Running test: .\tests\test_simple_array.rcc
[2, 4, 6]
Running test: .\tests\test_simple_modules.rcc
Test 1: print simple
Test 2: print con múltiples args 123 456
Running test: .\tests\test_stdlib_demo.rcc
null
Running test: .\tests\test_stdlib_math.rcc
Running test: .\tests\test_std_library.rcc
Running test: .\tests\test_sugar_types.rcc
=== Sugar Types Test ===

✓ int se asigna correctamente a i8, i16, i32, i64, u8, u16, u32, u64
✓ float se asigna correctamente a f32, f64, decimal
✓ Operaciones aritméticas funcionan: 1010, 8.53452
Valor i32: 42
Valor f64: 3.14159
✓ Conversión widening automática: i8 -> i32 -> i64 -> f64 = 50
✓ Arrays tipados: i8[], i32[], f64[] funcionan correctamente

✅ Todos los sugar types y conversiones funcionan perfectamente!
null
Running test: .\tests\test_toStr.rcc
42
Success!
null
Running test: .\tests\test_typescript_style_modules.rcc
╔════════════════════════════════════════════════════════╗
║   TEST: Sistema de Módulos Estilo TypeScript         ║
╚════════════════════════════════════════════════════════╝

📦 TEST 1: Named Imports
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running test: .\tests\test_types_edge_cases.rcc
=== EDGE CASES: Sistema de Tipos ===

[ 1 ] Valores límite de tipos enteros con signo:
  ✓ i8:  -128 a 127
  ✓ i16: -32768 a 32767
  ✓ i32: -2147483648 a 2147483647
  ✓ i64: -9223372036854776000 a 9223372036854776000

[ 2 ] Valores límite de tipos sin signo:
  ✓ u8:  0 a 255
  ✓ u16: 0 a 65535
  ✓ u32: 0 a 4294967295
  ✓ u64: 0 a 9223372036854776000

[ 3 ] Cadenas de conversión widening:
  ✓ i8 -> i16 -> i32 -> i64 -> f64 -> decimal: 42
  ✓ u8 -> u16 -> u32 -> u64 -> f64: 200
  ✓ u8 -> i16 -> i64 -> decimal: 100

[ 4 ] Operaciones aritméticas entre diferentes tipos:
  ✓ i8(10) + i32(20) = i32(30)
  ✓ i16(5) * i64(15) = i64(75)
  ✓ u8(8) - u32(4) = u32(4)
  ✓ f32(3.5) * f64(2.0) = f64(7)
  ✓ i32(100) + f32(2.5) = f32(102.5)

[ 5 ] Operaciones bitwise entre tipos enteros:
  ✓ i8(15) & i32(240) = i32(0)
  ✓ u8(12) | u16(10) = u16(14)
  ✓ i16(5) ^ i32(3) = i32(6)
  ✓ i8(4) << i32(2) = i32(16)
  ✓ i32(16) >> i8(2) = i32(4)

[ 6 ] Funciones con múltiples tipos de parámetros:
  ✓ i8: 50
  ✓ i32: 5000
  ✓ f64: 50.5
  ✓ decimal: 50.123456
  ✓ Sugar type en i8: i8: 25
  ✓ Sugar type en i32: i32: 2500
  ✓ Sugar type en f64: f64: 25.5

[ 7 ] Arrays con diferentes tipos numéricos:
  ✓ i8[]:  [1, 2, 3, 4, 5]
  ✓ i32[]: [100, 200, 300]
  ✓ u8[]:  [255, 128, 64]
  ✓ f32[]: [1.1, 2.2, 3.3]
  ✓ f64[]: [10.5, 20.5, 30.5]

[ 8 ] Operaciones con cero y números negativos:
  ✓ i8(0) + i8(-128) = -128
  ✓ i32(-1000000) + i32(1000000) = 0
  ✓ f64(-3.14159) * -1 = 3.14159
  ✓ Negación unaria: -42 = -42

[ 9 ] Conversiones explícitas entre tipos:
  Original i32: 256
  ✓ toI8():  127
  ✓ toI16(): 256
  ✓ toI64(): 256
  ✓ toF32(): 256
  ✓ toF64(): 256
  Original f64: 123.456
  ✓ toInt():     123
  ✓ toDecimal(): 123.456

[ 10 ] Comparaciones entre diferentes tipos:
  ✓ i8(10) == i32(10): true
  ✓ u8(50) == i16(50): true
  ✓ f32(3.14) == f64(3.14): true
  ✓ i32(100) < i64(200): true

[ 11 ] Expresiones complejas con múltiples tipos:
  ✓ i8(2) + i16(3) * i32(4) - i64(5) = i64(9)
  ✓ u8(10) * u16(20) + u32(30) = u32(230)
  ✓ f32(2.5) * f64(3.0) + decimal(1.5) = decimal(9)

[ 12 ] Operaciones con arrays de tipos mixtos:
  ✓ Array i8 length: [1, 2, 3]
  ✓ Array i32 length: [10, 20, 30]
  ✓ Array f64 length: [1.5, 2.5, 3.5]
  ✓ Después de push: i8[] = [1, 2, 3, 4]
  ✓ Después de push: i32[] = [10, 20, 30, 40]
  ✓ Después de push: f64[] = [1.5, 2.5, 3.5, 4.5]

[ 13 ] Valores decimales de alta precisión:
  ✓ decimal(0.1) + decimal(0.2) = 0.30000000000000004
  ✓ Suma de alta precisión: 999999999.1234568

[ 14 ] Funciones recursivas con tipos específicos:
