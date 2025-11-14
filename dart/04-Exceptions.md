# Exceptions

Dart provides `Exception` and `Error` types, and numerous subtypes.
However, Dart programs can throw any non-null object as an exception.

```dart
throw FormatException("Some error");
throw "Oh noes!";
```

> Consider throwing types that implement `Error` or `Exception`

Exceptions are an expression.

```dart
void distanceTo(Point other) => throw UnimplementedError();
```

## Catching exceptions

```dart
try {
  method();
} on NewException {
  method2();
} on NewException2 catch (e) {
  print("Unknown exception: $e");
} catch (e) {
  print('Something really unknown: $e');
}
```

### Stack Trace

```dart
try {
  method();
} catch (e, stackTrace) {
  print("Error: $e");
  print("Stack trace: $stackTrace");
}
```

### Propogating exceptions

`rethrow` can be used to propagate exceptions after catching them to be caught by an outer `try-catch`.

```dart
void method() {
  try {
    method2();
  } catch (e) {
    // Do some logging or cleanup
    rethrow; // Propagate the exception
  }
}
```

### Finally

```dart
try {
  method();
} catch (e, stackTrace) {
  print("Error: $e");
  print("Stack trace: $stackTrace");
}
finally {
  // This block is always executed
  cleanMethod();
}
```

## Assert

Assert statements are used to disrupt normal execution if a boolean condition is false.

```dart
// Variable should not be null
assert(text != null);

assert(text != null, "Text should not be null");

assert(num < 100 && num > 0, "Number should be between 0 and 100");
```

> Asserts are only evaluated in debug mode.
> In production mode, they are ignored.
