# Classes

All classes descend from `Object`

_Mixin_-based inheritance means every class has exactly one superclass (except top-class `Object?`),
a class body can be reused in multiple class hierarchies.

Objects have _members_ consisting of functions and data.

```dart
class Point {
  double? x;
  double? y;
  double z = 0;
}
```

## Instance variables

```dart
var p = Point(2, 2);

print(p.y);

double distance = p.distanceTo(Point(4, 4));

// To avoid exceptions when p is null
var a = p?.y;
```

## Constructors

An object can be created using a constructor.
It can be either `ClassName` or `ClassName.identifier`

```dart
var p1 = Point(2, 2);
// optional new keyword
var p2 = new Point.fromJson({'x': 1, 'y': 2});
```

### Const

```dart
var p = const ImmutablePoint(2, 2) // Compile time immutable instance

var a = const ImmutablePoint(1, 1);
var b = const ImmutablePoint(1, 1);

assert(identical(a, b)); // They are the same instance!
```

```dart
// Lots of const keywords here.
const pointAndLine = const {
  'point': const [const ImmutablePoint(0, 0)],
  'line': const [const ImmutablePoint(1, 10), const ImmutablePoint(-2, 11)],
};
```

```dart
// Only one const, which establishes the constant context.
const pointAndLine = {
  'point': [ImmutablePoint(0, 0)],
  'line': [ImmutablePoint(1, 10), ImmutablePoint(-2, 11)],
};
```

> If a constant constructor is outside of a constant context and is invoked without `const`, it creates a non-constant object:

```dart
var a = const ImmutablePoint(1, 1); // Creates a constant
var b = ImmutablePoint(1, 1); // Does NOT create a constant

assert(!identical(a, b)); // NOT the same instance!
```

> `runtimeType` is used on an object to return the `Type` object

```dart
print('The type of a is ${a.runtimeType}');
```

## Instance Variables

An uninitialized instance variable has an initial value of `null`.
Non-nullable instance variables must be initialized before the constructor body ends.

```dart
class Point {
  double? x;
  double? y;
}

void main() {
  var point = Point();
  point.x = 1.0;
}
```

All instances variable generate implicit getters and setters (non-final and `late final`).

> Getters and setters are special methods that provide read and write access to an object's properties.
> It can be explicitly defined using the `get` and `set` keywords.

Initializing a non-`late` instance variable sets the value where the instance is created, before the constructor.

> Instance variables can be `final`

### Implicit interface

`_variable` is a __private__ instance variable.

### Implements

Use `implements` implement an interface.

```dart
class Person {
  final String _name;

  Personal(this._name);

  String greet(String who) => 'Hello, $who. I am $_name.';
}

class Immigrant implements Person {
  String get _name => '';

  String greet(String who) => 'Hello, $who. Do you know who I am?"
}

String greetBob(Person person) => person.greet('Bob');

void main() {
  print(greetBob(Person('Kathy')));
  print(greetBob(Impostor()));
}
```

> A class can implement multiple interfaces.


