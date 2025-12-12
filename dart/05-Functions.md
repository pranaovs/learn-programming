# Functions

Functions are also objects, since dart is an _OOP_ language.

`Function` type.
It can be assigned to variables and passed as arguments to other functions.

```dart
// With types (recommended)
bool isNoble(int atomicNumber) {
  return _nobleGases[atomicNumber] != null;
}

// Without type annotation
isNoble(atomicNumber) {
  return _nobleGases[atomicNumber] != null;
}

// Shorthand syntax
bool isNoble(int atomicNumber) => _nobleGases[atomicNumber] != null;
```

## Parameters

A function can have any number of **required** positional parameters.
They are followed by named parameters **or** optional positional parameters.

```dart
/// Sets the [bold] and [hidden] flags ...
void enableFlags({bool? bold, bool? hidden}) {
  // Body
}

enableFlags(bold: true, hidden: false);
```

> Named parameters are wrapped in curly braces `{}`.

### Default values

```dart
/// Sets the [bold] and [hidden] flags ...
void enableFlags({bool bold = false, bool hidden = false}) {
  // body
}

enableFlags(bold: true); // uses hidden: false
enableFlags(); // Uses bold: false, hidden: false
```

### Required values

```dart
/// Sets the [bold] and [hidden] flags ...
void enableFlags({bool bold = false, required bool hidden}) {
  ...
}

enableFlags(bold: true); // Error
enableFlags(bold: true, hidden: false)
```

> `requried` is required if default value is not provided since it would be `null` by default

### Optional values

Wrapping a set of parameters in square brackets `[]` makes them optional positional parameters.
They must be nullable as they might be omitted.

```dart
String say(String from, String msg, [String? device]) {
  var result = '$from says $msg';
  if (device != null) {
    result = '$result with a $device';
  }
  return result;
}

say('Bob', 'Howdy'); // OK
say('Bob', 'Howdy', 'smartphone'); // OK
```

### Optional positional parameters with default values

```dart
String say(String from, String msg, [String device = 'unknown']) {
  return '$from says $msg with a $device';
}

say('Bob', 'Howdy'); // OK
say('Bob', 'Howdy', 'smartphone'); // OK
```

## `main()` function

```dart
void main() {
  print("Hello, World!")
}
```

### Command-line arguments

```dart
void main(List<String> args) {
  if (args.isEmpty) {
    print('No arguments provided.');
  } else {
    print('Arguments count: ${args.length}');
    print('Arguments: ${args.join(', ')}');
  }
}
```

## Functions as first-class objects

```dart
void printElement(int element) {
  print(element);
}

var list = [1, 2, 3];

// Pass printElement as a parameter.
list.forEach(printElement);
```

```dart
var shout = (msg) => "!!! ${msg.toUpperCase()} !!!";
shout("hello"); // returns "!!! HELLO !!!"
```
