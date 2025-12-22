# Libraries and Imports

Identifies that start with `_` are only visible inside the library.
`private`, `protected`, and `public` keywords are not supported.

Every dart file is a library, even without `library` declarative.
Libraries can be distributed using packages.

## Using libraries

`import` to specify a library to be used in another library.

```dart
import 'dart:js_interop'; // Importing a built-in library
```

For built-in libraries, URI is `dart:`.

For other libraries, URI can be a relative or absolute path, or `package:` URI.

```dart
import 'package:my_package/my_library.dart'; // Importing a library from a package
import 'src/utils.dart'; // Importing a library using a relative path

import 'src2/utils.dart' as utils2; // Importing with duplicate names using prefix

Element element1 = Element() // from utils.dart
utils2.Element element2 = utils2.Element(); // from utils2.dart

import 'package:lib1/lib1.dart' show Lib1Class; // Selective import
import 'package:lib2/lib2.dart' hide Lib2Class; // Hiding specific names
```

## Lazy loading libraries

```dart
import 'src/heavy_library.dart' deferred as heavyLib;

Future<void> loadHeavyLibrary() async {
  await heavyLib.loadLibrary(); // Load the library when needed
  heavyLib.heavyFunction(); // Use the library after loading
}
```

* During deferred import, constants don't exist until the library is loaded.
* Types from deferred libraries can't be used
* `loadLibrary()` is implicitly inserted into the namespace when accessing members. It returns a `Future`.

## `library` directive

To specify library-level doc comments or metadata annotations, use the `library` directive at the top of the file.

```dart
@TestOn('browser')
library;
```
