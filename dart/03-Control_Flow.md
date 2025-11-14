# Control Flow Statements

## Loops

### For loop

```dart
var message = StringBuffer('Dart is fun');
for (var i = 0; i < 5; i++) {
  message.write('!');
}
```

```dart
var collection = [1, 2, 3];
collection.forEach(print); // 1 2 3
```

### While and do-while

```dart
while (!isDone()) {
  doSomething();
}
```

```dart
do {
  printLine();
} while (!atEndOfPage());
```

### Labels

A label is an identifier followed by a colon `labelName:`

```dart
outerLoop:
for (var i = 1; i <= 3; i++) {
  for (var j = 1; j <= 3; j++) {
    print('i = $i, j = $j');
    if (i == 2 && j == 2) {
      break outerLoop;
    }
  }
}
print('outerLoop exited');
```

> `break` is used to exit the loop

```dart
outerLoop:
for (var i = 1; i <= 3; i++) {
  for (var j = 1; j <= 3; j++) {
    if (i == 2 && j == 2) {
      continue outerLoop;
    }
    print('i = $i, j = $j');
  }
}
```

> `continue` skips the current iteration of the loop

```dart
for (int i = 0; i < candidates.length; i++) {
  var candidate = candidates[i];
  if (candidate.yearsExperience < 5) {
    continue;
  }
  candidate.interview();
}
```

## Branches

### If

`if` with optional `else` clauses

```dart
if (boolean_value) {
  // true
} else if (boolean_value2) {
  // true2
} else {
  // false
}
```

```dart
if (pair case [int x, int y]) return Point(x, y);
```

### Switch

```dart
var command = 'OPEN';
switch (command) {
  case 'CLOSED':
    print("Closed");
  case 'PENDING':
    print("Pending");
  case 'APPROVED':
    print("Approved");
  case 'DENIED':
    print("Denied");
  case 'OPEN':
    print("Open");
  default:
    print("Unknown command");
}
```

Empty cases fall through next case.
`break` can be used to exit the `switch` block.
`continue` with a label can be used for a non-sequential fall-through.

#### Switch expression

`switch` can be used as an expression to return a value based on conditions.

```dart
var command = 'OPEN';
var status = switch (command) {
  'CLOSED' => 'The request is closed',
  'PENDING' => 'The request is pending review',
  'APPROVED' => 'The request has been approved',
  'DENIED' => 'The request has been denied',
  'OPEN' => 'The request is open for review',
  _ => 'Unknown command status'
};
print(status); // The request is open for review
```

```dart
// Switch expression with multiple values
var result = switch (command) {
  'CLOSED' || 'DENIED' => 'Inactive',
  'PENDING' || 'OPEN' => 'Active',
  'APPROVED' => 'Complete',
  _ => 'Unknown'
};
```

> Exhaustiveness checking: compiler gives an error if not all possible values are covered.

### Guard clauses

Guard conditions evaluate a boolean expression **after** matching the pattern.
Adds additional constraints to the pattern.

`when` keyword is used to specify guard conditions.
It can follow `if-case` statements, `switch` statements, and `switch` expressions.

```dart
// Switch statement:
switch (something) {
  case somePattern when some || boolean || expression:
  // body
  case someOtherPattern when some || boolean || expression:
  // body
  case _:
  // default body
}
```

```dart
// Switch expression:
var value = switch (something) {
  somePattern when some || boolean || expression => body,
  someOtherPattern when some || boolean || expression => body2,
  _ => defaultBody
}
```

```dart
// If-case statement:
if (something case somePattern when some || boolean || expression) {
  // body;
}
```
