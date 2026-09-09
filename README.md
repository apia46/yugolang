# Yugolang
Yugolang is a very serious programming language.<br>
Here are some of its features:
## Syntax
The symbols (, {, and [ are equivalent.
```
print("hi");
print{"hi"};
print["hi"];
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

