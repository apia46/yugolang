# Yugolang
Yugolang is a statically typed superfunctional programming language.<br>
Here are some of its features:
## Syntax
Yugolang uses a C style syntax that you will probably be familiar with.
```
print("hi);
```
The symbols (, {, and [ are equivalent.
```
print("hi");
print{"hi"};
print["hi"];
print["hi"); // but you cant do this
```
They are also unnecessary for function calls, which simply consume the next expression.
```
print "hi";
```
Function calls can also consume the previous expression.
```
"hi" print;
```
Different functions can have different preferred calling directions.<br>
For example, the + function prefers to call left, and returns a function which prefers to call right.
In fact, all operators are functions.<br>
```
3 + 2;
// is parsed like so:
([3]+)[2];
```
If a function cannot find an argument in its direction, it will look in the other direction.<br>
You can configure the compiler lint `function_called_in_wrong_direction` to warn or deny this behaviour.
```
+ 3 2; // this works
2 3 +; // this also works
```
Functions have different priorities. If there is a chain of functions and expressions, they are resolved in order of decreasing priority.
```
// * and its return value have priority -5
// + and its return value have priority -6
// Therefore,
3 + 2 * 7;
// is parsed like so:
3 + [2]* 7;
3 + ([2]*)[7];
[3]+ ([2]*)[7];
([3]+)[([2]*)[7]];
```
## Declarations
Variables are declared like so:
```
let i = 4;
```
You can optionally add a type hint:
```
let i:int = 5;
```
Functions are declared like so, where type specifiers are necessary:
```
let add = func (a:int, b:int) -> int {
    a+b
}
```
## Parsing
In Yugolang, a statement is a set of expressions and functions, ending with a semicolon.<br>
```
if true "hello" else "bye";
```
A statement like this will be parsed by replacing functions and their arguments with their return values, in order of decreasing function priority.<br>
```
if true "hello" else "bye";
// if is a function that consumes rightwards
// it finds true and returns a function to evaluate the result
|x| {Some(x)} "hello" else "bye";
// this wraps the "hi" in an option enum,
// but if it found false then it would simply be replaced with the None variant
Some("hello") else "bye";
// else consumes leftwards, finds a Some,
// and returns a function to discard the else block
|x| {"hello"} "bye";
// finally, "bye" is discarded
"hello";
```
## Lazy Evaluation
Scopes are passed directly to functions, and are evaluated only when they are needed.<br>
This prevents functions with side-effects being called when they are to be skipped over.
```
if (2 + 3 == 4) {print("hello")};
// if recieves (2 + 3 == 4), which it evaluates
// note that if the parentheses were not there,
// the operators would evaluate first as they have a higher priority than if,
// and the if would simply recieve false
|x| {None} {print("hello")};
// the |x| {None} function recieves {print("hello")}, which it throws away without evaluating
None;
```
