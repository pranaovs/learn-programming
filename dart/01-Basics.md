# Dart Programming

Every program starts with a `main()` function.

```dart
void main() {
  print("Hello, World!")
}
```

## Comments

Dart supports three types of comments:

### Single-line comments

Use `//` for single-line comments:

```dart
// This is a single-line comment
void main() {
  print("Hello, World!"); // Comment at end of line
}
```

### Multi-line comments

Use `/* */` for multi-line comments:

```dart
/*
 * This is a multi-line comment
 * that spans multiple lines
 */
void main() {
  /* Inline multi-line comment */ print("Hello!");
}
```

### Documentation comments

Use `///` for documentation comments (similar to JSDoc):

```dart
/// This function greets the user
/// 
/// Takes a [name] parameter and prints a greeting message.
/// Returns nothing (void).
void greet(String name) {
  print("Hello, $name!");
}

/// Main entry point of the application
void main() {
  greet("Dart");
}
```
