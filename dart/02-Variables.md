# Variables

`var` can be used to define variables without types.
Type inference is used to determine the type using the initial value.

```dart
var name = 'Bob';
```

`Object` type can be used if an object isn't restricted to a single type.
`dynamic` type disables static type checking.

Explicit types can also be used.

```dart
String name = 'Bob';
```

## Null safety

Null safety prevents variables from being null unless specified.
The compiler does not allow values to be null unless explicitly declared.

```dart
String name = null; // Error

String? name = null; // OK
```

Values must be initialized before used because they cannot be null.
Uninitialized variables are not allowed. They will need to be assigned a value before used.

Methods or properties cannot be accessed with a nullable type.

Uninitialized variables of nullable type have an initial value of `null`.

```dart
int? lineCount;
assert(lineCount == null);
```

> `asset()` is used in debugging to disrupt execution if boolean condition is false.

## Late variables

Used to declare a non-nullable variable that's initialized after its declaration.

If we are sure that a variable is set before it's used, but the compiler disagrees, it can be marked `late`.

```dart
late String description;

void main() {
  description = 'Feijoada!';
  print(description);
}
```

If a variable is marked `late` but initialized at declaration, the initializer runs at the first time the variable is used.

> Lazy initializing a variable

```dart
// This is the program's only call to readThermometer().
late String temperature = readThermometer(); // Lazily initialized.
```

If the `temperature` variable is never used, the costly `readThermometer()` is never called.

## Final

`final` can be used to finalize a variable, if it will never be changed.

```dart
// final - runtime constant
final String name = getName(); // OK - value determined at runtime
final DateTime now = DateTime.now(); // OK - different each time
```

## Const

`const` is used for variable that are compile-time constants.

> `const` variables are implicitly `final`

```dart
// const - compile-time constant
const String greeting = 'Hello'; // OK - known at compile time
const DateTime now = DateTime.now(); // ERROR - not compile-time constant

// const variables are implicitly final
const pi = 3.14159; // This is both const and final
```

## Wildcard types

`_` can be used as a wildcard type to ignore specific types.

```dart
try {
  throw '!';
} catch (_) {
  print('oops');
}
```

```dart
Foo(_, this._, super._, void _()) {}

list.where((_) => true);

void f(void g(int _, bool _)) {}

typedef T = void Function(String _, String _);
```
