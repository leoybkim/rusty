# 3. Common Programing Concepts

## 3.1 Variables and Mutability

**Variables** are immutable by default but you can make them mutable with `mut` in front of the variable name, and are declared with `let` keyword.
**Constants** are are always immutable and are declared with `const` keyword, and the type *must* be annotated.

You can declare a new variable with the same name as a previous variable. Rustaceans say the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In another words, the second variable overshadows the first, until either it itself is shadowed or the scope ends.
Shadowing is different from marking a variable as `mut` because we'll get compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
Shadowing also allows us to change the type of the value but reuse the same variable name because it is effectively creating new variable. However, `mut` will get a compile-time error when we try to mutate the variable's type.

## 3.2 Data Types

Rust is a *statically typed* language, which means tha it must know the types of all variables at compile time. The compiler can usually infer, but in cases many types are possible, we must add a type annotation like `let guess: u32 = "42".parse().expect("Not a number!");`.

### Scalar Types
A *scalar type* represents a single value. Rust has 4 primary scalar types:
- integers
- floating-point numbers
- Booleans
- characters

#### Integer Types
An *integer* is a number without a fractional component.

| Length    | Signed  | Unsigned |
| ----------| ------- |----------|
| 8-bit     | i8      | u8       |
| 16-bit    | i16     | u16      |
| 32-bit    | i32     | u32      |
| 64-bit    | i64     | u64      |
| 128-bit   | i128    | u128     |
| architecture dependent | isize| usize|

Each variant can be either signed or unsigned and has an explicit size. *Signed* and *unsigned* refer to whether it's possible for the number to be negative. Signed numbers are stored using two's complement representation. 

Each signed variant can store numbers from $-(2^(n-1))$ to $2^(n-1) - 1$ inclusive where n is the number of bits that variant uses. ex) `i8` can store from -128 to 127.
Unsigned variants can store numbers from $0$ to $2^(n) - 1$. ex) `u8` can store numbers from 0 to 255.

Additionally, the isize and usize types depend on the architecture of the computer your programming is running on. 64bits if you're on 64-bit architecture and 32 bits if you are on a 32-bit architecture.


| Number literals | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte(u8 only)   | b'A'        |

Rust's default integer type is `i32`.


##### Integer Overflow
*Integer overflow* will occur if you try to change the variable to a value outside allowed range. 

When compiling in debug mode, Rust includes checks for integer overflow that cause your program to *panic* (program exits with an error) at runtime.

When compiling in release mode with `--release` flag, Rust does *not* include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs *two's complement wrapping*. Values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type can hold.

To explicitly handle the possibility of overflow, you can use standard library methods for primitive numeric types:
- wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`
- return the `None` value if there is overflow the `checked_*` methods
- return the value and a Boolean indicating whether there was overflow with the `overflowing_*` methods
- saturate at the value's minimum or maximum values with the `saturating_*` methods


#### Floating-Point Types

Rust has 2 primitive types for *floating-point numbers*, whare are numbers with decimal points: `f32` and `f64`. The default type is `f64`. All floating points are signed.

#### Numeric Operations
Rust supports the basic mathematical operations: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer.


#### The Boolean Type
Boolean type has two possible values: `true` and `false` and is specified using `bool`. Booleans are one byte in size.

#### The Character Type
The `char` type is the most primitive alphabetic type. We specify `char` literals with single quotes, as opposed to string literals, which use double quotes. Rust's `char` type is four bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. 


### Compound Types
*Compound types* can group multiple values into one type. Rust has 2 primitive compoun types: typles and arrays.

#### The Tuple Type
A *tuple* is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

You can destructure a tuple to get individual values out like `let tup = (500, 6.4, 1);` then `let (x, y, z) = tup;`.
You can also access tuple elements directly by using a period `.` followed by the indiex of the value. `let five_hundred = tup.0;`.
The tuple without any values has a special name, *unit*. This value and its corresponding type are both written `()` and represent an empty value or an empty return type.
Expressions implicitly return the unit value if they don't return any other value.

#### The Array Type
Unlike a tuple, every element of an array must have the same type. Arrays in Rust have a fixed length. Arrays are useful when you want your data allocated on a stack rather than the heap, or if you want to ensure you always have a fixed number of elements. An array isn't as flexible as the vector type. A *vector* is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap.

You write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array: `let a: [i32; 5] = [1, 2, 3, 4, 5];`. You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array: `let a = [3; 5];`.

##### Accessing Array Elements
An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access element of an array using indexing: `a[1]`.

##### Invalid Array Element Access
The program results in a *runtime* error at the point of using an invalid value in the indexing operation. This check has to happen at runtime and Rust will panic :crab:.
This is an example of Rust's memory safety principles in action. In many low-level language, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. 


## 3.3 Functions

Rust code uses *snake case* as the conventional style for function and variable names. 
Define function with `fn` followed by a function name and a set of parentheses. Rust doesn't care where you define your functions, only that they're defined somewhere in a scope that can be seen by the caller.

### Parameters
When a function has a parameters, you can provide it with concrete values for those parameters called *arguments*.
In function signatures, you must declare the type of each parameter.

### Statement and Expressions
Function bodies are made up of a series of statements optionally ending in an expression. Rust is an expression-based language.
- statements are instructions that perform some action and do not return a values
    - creating a variable and assigning a value
        ex) `let x = (let y = 6);` is not possible because `let y = 6` statement does not return a value. This is different from C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write `x = y = 6` and have both `x` and `y` have the value `6`.
    - function definitions
- expressions evaluate to a resultant value
    - `5 + 6`
    - `6` in the statement `let y = 6;`
    - calling a function
    - calling a macro line 
    - a new scope block created with curly brackets
        - ```rust
            fn main() {
                let y = {
                    let x = 3;
                    x + 1
                };
            }
          ```
            - note that the `x + 1` line doesn't have a semicolon at the end
            - expressions do not include ending semicolons
            - if you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value

### Functions with Return Values

Functions can return values to the code that calls them. We don't name return values, but we must declare their type after an arrow `->`. In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}
```


## 3.4 Comments

In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line. For comments that extend beyond a single line, you’ll need to include `//` on each line. Comments can also be placed at the end of lines containing code.


## 3.5 Control Flow

### `if` Expressions
An `if` expression allows you to branch your code depending on conditions. All `if` expression start with the keyword `if`, followed by a condition. We place the block of code to execute if the conditon is `true` immediately after the condtion inside curly brackets. Blocks of code associated with the conditon in `if` expressions are sometimes called *arm*, just like the arms in `match` expressions. Optionally, we can also include an `else` expression to give the program an alternative block of code to exectue should the condition evalutes to `false`. If you don't provide an `else` expression and the condition is `false`, the program will just skip the `if` block and move on to the next bit of code.

If the condition isn't a `bool`, we'll get an error. Unlilke languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide `if` with a Boolean as its condition.

#### Handling Multiple Conditions with `else if`
You can use multiple conditions by combining `if` and `else` in an `else if` expression.

#### Using `if` in a `let` Statement
Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable: `let number = if condition { 5 } else { 6 };`. Both arms of the condition must match the type otherwise we'll get an error during compilation.

### Repetition with Loops
Rusts has 3 kinds of loops: `loop`, `while`, and `for`.

#### Repeating Code with `loop`
The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop. You can place the `break` keyword within the loop to tell the program when to stop executing the loop. You can also use `continue` to tell the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

#### Returning Values from Loops
You can also `return` from inside a loop. While `break` only exits the current loop, `return` always exits the current function.

#### Loop Labels to Disambiguate Between Multiple Loops
If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally specify a *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```


#### Conditional Loops with `while`
While the condition is `true`, the loop runs. When the condition ceases to be `true`, the program calls `break`, stopping the loop. The `while` construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else` and `break`.

#### Looping Through a Collection with `for`
You can use a `for` loop and execute some code for each item in a collection. This increases the safety of the code and eliminates the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items with using regular `while` loops like `while index < 5`. Machine code generated from `for` loops can be more efficient as well, because index doesn't need to be compared to the length of array at every iteration. 


