# Rust Notes

## Variables
- Immutable
- must use key word, mut, when creating variable to make it mutable

## Function
- `fn foo () {}` - fn is function, foo name
- `fn add (x : i32, y: i32)` - parameters
- `fn add (x : i32, y: i32) -> i32 {}` - arrows followed by return type

**Stement**: Preform action, no return value, void\
**Expression**: Evaluate to value to be returned\
<br>
**Function Pointer**: Treat function as 'First class value' meaning can assign them to variables, pass as arguments or return them from other functions

## Own and Borrow Rules
- Each value has variable called its owner
- Value have only own owner at a time
- When owner go out of scope, value dropped

## String
- Mutable, growable, allocated on heap during runtime, size not known
- Owned version of string
- Use when need string data or to modify the string
- Use more memory
  
## &str - 'slice' (slice of memory)
- Immutable view or slice of a string
- Point to portion location of string in memory, does not contain the actual string value
- For fixed static strings who's data does not change
- When want to view part of string without taking ownership
<br>
<br>
`let name: &str = "Alice"` "Alice" exist in binary of compiled code, is at fixed location in memory on stack. Part of program static data, exist for life of program<br>The &str 'name' points to the location of "Alice" in memory

## Result
Enum in Rust library,represents outcome of an operation that can either pass or fail<br>
`enum Result<T, E> {
    Ok(T),
   Err(E),
}<br>
**T** - success
<br>
**E** - failure
<br>
Result is usually returned by a method, the result of the method is contained in **T** or if the method fails **E** is returned.
Access the value **T** using .unwrap() method, T can be of any type.
<br> 
Simply return value of method contained inResult 