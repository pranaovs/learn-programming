# Classes and Methods

## Method

A method is a names set consisting of statements that can be called from elsewhere of a program.

```java
public static void greet() {
  System.out.println("Greetings from the method greet!");
}
```

### Parameters

```java
public static void sum(int first, int second) {
  system.out.println("the sum of numbers " + first + " and " + second + " is " + (first + second));
}
```

> Parameter values are copied inside a method call
>
> Changing the method variables do not affect the original variable

### Returning

* The keyword `void` means the method does not return anything.

`return` keyword is used to return a value from a method.
The code execution flow in a method stops after the first `return` keyword.

Variables inside a method are discarded when the function is exited/returned.

```java
public static void sum(int first, int second) {
  return (first + second);
```
