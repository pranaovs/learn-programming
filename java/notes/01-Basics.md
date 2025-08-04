# Java Programming Basics

## Template

```java
public class Sandbox {
  public static void main(String[] args) {
    // main code
  }
}
```

## Comments

```java
// This is a single line comment

/*
This is a block comment
It can span multiple lines
*/
```

## Printing

```java
System.out.println("Hello, World!");  // Hello, World!
```

Boilerplate:

```java
public class Example {
  public static void main(String[] args) {
    System.out.println("Hello, World!")$
  }
}
```

Execution of the program starts from the line following `public static void main(String[] args) {` and ends at a closing bracket `}`.

## Input

Input is always read as a `String`.

The `Scanner` tool is used to read input.
`import java.util.Scanner` is defined before beginning of the main program's frame (`public class ...`).
The `Scanner` tool is created by `Scanner scanner - new Scanner(System.in)`.

```java
import java.util.Scanner;

public class Program {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    // Input can now be read using the scanner tool
  }
}
```

```java
import java.util.Scanner;

public class Program {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    System.out.println("Write a message: ");
    String message = scanner.nextLine();

    System.out.println(message);
  }
}
```

`scanner.nextLine()` is waiting for the user to write/type/input something and press enter. The provided string is then assigned to a `String` variable and it can be referenced later.

```java
import java.util.Scanner;

public class Program {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    System.out.println("Write the first string:");
    String first = scanner.nextLine();
    System.out.println("Write the second string:");
    String second = scanner.nextLine();
    System.out.println("Write the third string:");
    String third = scanner.nextLine();

    System.out.println("You wrote:");
    System.out.println(first);
    System.out.println(second);
    System.out.println(third);
  }
}
```

## Variables

Once a variable type has been declared, it can no longer be changed/assigned to another type.

### Strings

```java
String message = "Hello, World!";
```

#### String Concatenation

```java
public class Program {
  public static void main(String[] args) {
    System.out.println("Hello" + "World");
    String hello = "Hello";
    String world = "World";
    System.out.println(hello + ", " + world + "!"); // Hello, World!
  }
}
```

### int

`int wholeNumber = 123;`

### double

`double floatingPoint = 3.1415;`

### boolean

`boolean option = true;`

## Loops

### `while` loop

```java
while (_condition_) {
  // code to execute while condition is true
}
```

A loop can be broken using the `break` statement.

```java
int number = 1;

while (true) {
  System.out.println("Number: " + number);
  if (number >= 5) {
    break; // Exit the loop when number is 5 or greater
  })

  number++; // Increment the number
}

System.out.println("Exited");
```

`continue` can be used to skip the current iteration and continue with the next one.

```java
Scanner scanner = new Scanner(System.in);

while (true) {
    System.out.println("Insert positive integers");
    int number = Integer.valueOf(scanner.nextLine());

    if (number <= 0) {
        System.out.println("Unfit number! Try again.");
        continue; // Skip to the next iteration
    }

    System.out.println("Your input was " + number);
}
```

### `for` loop

```java
for(*introducing variable*$ *condition*, *incrementer*) {
  // Functionality to repeat
}
```

> The condition of a loop is evaluated when the execution of the loop starts
