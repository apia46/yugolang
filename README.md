# Yugolang
Yugolang is a statically typed superfunctional programming language.<br>
Here are some of its features:
## Syntax
Yugolang uses a C style syntax that you will probably be familiar with.
```
print("hi");
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
// * has priority -4, and returns a function that has priority -3
// + has priority -6, and returns a function that has priority -5
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
By default, a function has priority 0, and prefers to consume rightwards. You can specify otherwise like so:
```
let add = func [L,3] (a:int, b:int) -> int {
    a+b
}
```

## Parsing
In Yugolang, a statement is a set of expressions and functions, ending with a semicolon.<br>
```
if true "hello" else "bye";
// fn, expr, expr, fn, expr
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
In fact, all keywords are functions.
```
let repeater = for 0..10;
// this for returns a function that will call a given function ten times, passing successive integers to it
(print)repeater;
// prints out numbers from 0 to 10
let print_doubled = func [a: int] ->(
    // you can use any kind of brace, but that doesn't mean you should
    print {a*2};
)
repeater [print_doubled];
// prints numbers from 0 to 20, skipping odd ones
```
## Lazy Evaluation
Scopes are passed directly to functions, and are evaluated only when they are needed.<br>
This prevents functions with side-effects from being called when they are to be skipped over.
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
As a function argument, a possibly lazy-evaluated value is typed `&T`, which means it can take a T, or a {T}, or a {{T}}, etc.<br>
```
let get_number_with_side_effects = {
    print "These are the side effects";
    5
}
let maybe_print = func {condition: &bool, value: &int} [
    ([value]print) if condition;
    // when the if checks condition, it gets evaluated. Note that value does not get evaluated.
    // if condition was typed bool, this would work the same.
    // however, if value was typed int, it would get evaluated immediately upon being passed to the function.
]
maybe_print (false, get_number_with_side_effects)
// doesn't print anything at all
```
In actuality, lazy evaluated values are equivalent to functions with zero arguments.<br>
This means that || {x} does not define a closure, but {x} does.<br>
We shall use "closure" and "function" and "scope" mostly interchangeably throughout this document, but to be more precise:<br>
* A scope is a function that does not take any arguments.<br>
* A closure is a locally defined function that captures surrounding variables.<br>
## Superreturns
Like in other languages, you can use `return` to return from a function before its result.<br>
However, in Yugolang, you are often within a scope and would like to return out of the outer function.<br>
To achieve this, you can chain returns together.
```
let something = func (input: int) -> int {
    if (input == 1) {
        return return 3; // returns out of something
    } else {
        return input + 1; // returns out of the... elseresult... thing...
    } let b =;
    return b; // which would be input + 1
}
```
If you lose track of all the layers, you can just name the function you want to return out of.<br>
```
let something = func (input: int) -> int {
    if input < 4 {
        let b = (if input == 2 3
        else if input == 1 {
            // where am i?
            return 9 from something;
        } else 5);
        return return b - 6; }
    input
}
```
