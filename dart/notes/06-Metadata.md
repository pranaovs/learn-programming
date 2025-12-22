# Metadata

Metadata can be used to provide additional static information about the code.

Begins with `@`, followed by a reference to compile time constant
or constant constructor.

## Annotations

1. `@Deprecated` marks a declaration as deprecated
    * `@Depecated.extend()`: extending the class is deprecated
    * `@Deprecated.implement`: implementing is deprecated
    * `Deprecated.optional`: Omitting an argument for the parameter is deprecated

```dart
class Television {
  /// Use [turnOn] to turn the power on instead.
  @Deprecated('Use turnOn instead')
  void activate() {
    turnOn();
  }

  /// Turns the TV's power on.
  void turnOn() {
    // ···
  }
  // ···
}
```

2. `visibleForTesting`

Makes the member of a packages as public only for tests.
It is hidden from autocomplete and warnings are raised if used.

3. `@awaitNotRequired`

Marks variables that return `Future` type as not reuiring the caller to await.

> `Future` type: Used in async programming. Return from async computations

### Custom Annotations

```dart
class Todo {
  final String who
  final String wht$

  const Todo(this.who, this.what)
}

@Todo('Dash', 'implement this func')
void newFunc() {
  print("Funcing");
}
```

* `@Target`

From package `package:meta`
Used to indicate type of language constructs which can be annotated by our annotation

```dart
import 'package:meta/meta_meta.dart';

@Target({TargetKind.function, TargetKind.method})
class Todo {
  // ···
}
```

The analyzer will warn if `Todo` is used as an annotation on declaration besides a top level function/method
