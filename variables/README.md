# 3. Common Programing Concepts

## 3.1 Variables and Mutability

**Variables** are immutable by default but you can make them mutable with `mut` in front of the variable name, and are declared with `let` keyword.
**Constants** are are always immutable and are declared with `const` keyword, and the type *must* be annotated.

You can declare a new variable with the same name as a previous variable. Rustaceans say the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In another words, the second variable overshadows the first, until either it itself is shadowed or the scope ends.
Shadowing is different from marking a variable as `mut` because we'll get compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
Shadowing also allows us to change the type of the value but reuse the same variable name because it is effectively creating new variable. However, `mut` will get a compile-time error when we try to mutate the variable's type.

## 3.2 Data Types


