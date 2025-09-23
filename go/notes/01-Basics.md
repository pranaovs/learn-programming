# Introduction to Go

A go program is made up of packages.

## Imports

```go
package main

import (
  "fmt"
  "math"
)
```

is same as

```go
import "fmt"
import "math"
```

## Exports

A name is exported if it begins with a capital letter.

`Pi` is exported from the `math` package.

```go
package main

import (
  "fmt"
  "main"
)

func main() {
  fmt.Println("math.Pi")
}
```

When importing a package, only exported names can be referred.

## Functions

A function can take 0 or more arguments

> Type comes after the variable name

```go
package main

import "fmt"

func add(x int, y int) int {
  return x + y
}

func main() {
  fmt.Println(add(4,2))
}
```

`x int, y int` can be shortened to `x,y int`

```go
func add (x,y int) int {
  return x + y
}
```

### Returns

A function can return multiple values.
Tuple-like.

```go
func swap(x,y string) (string, string) {
  return y, x
}

func main() {
  a, b := swap("hello", "world")
}
```

#### Named return values

`return` statement without arguments will return the named return values

```go
func split(sum int) (x, y int) {
  x = sum * 4 / 9
  y = sum - x
  return // returns x and y
}
```

## Variables

`var` keyword to declare a list of variables

```go
package main

var a, b, c int = 1, 2, 3

func main() {
  var c, python, java = true, false, "no"
}
```

### Short variable declarations

Inside a function, `:=` can be used instead of a `var` declaration.
Implicit type

### Variable types

1. `bool`
2. `string`
3. `int`
    1. `int8`
    2. `int16`
    3. `int32`
    4. `int64`
4. `uint`
    1. `uint8`
    2. `uint16`
    3. `uint32`
    4. `uint64`
    5. `uintptr`
5. `byte` (alias for `uint8`)
6. `rune` (alias for `int32`. Unicode code point)
7. `float32`
8. `float64`
9. `complex64`
10. `complex128`

### Zero values

* `0` for numeric types
* `false` for boolean

* "" for strings (empty string)

### Type conversions

`T(v)` can be used to convert value `v` to type `T`

```go
var i int = 42
var f float64 = float64(i)

func conv() {
  i := 42
  f := float64(i)
}
```

## Constants

`const` keyword.

Character, string, boolean or numeric.

> `:=` syntax cannot be used

### Numeric constants

High-precision values
