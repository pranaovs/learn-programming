# Go Functions

Functions are values too. They can be passed around like other values.

> Can be used as function arguments or return values

```go
func compute(fn func(float64, float64) float64) float64 {
  return fn(3, 4)
}
```

## Function closures

A closure is a function that captures the environment in which it was created.
It can access variables from its surrounding scope even after that scope has finished executing.

```go
func adder() func(int) int {
  sum := 0
  return func(x int) int {
    sum += x
    return sum
  }
}
```

Each closure has its own copy of the variables it captures.

## Methods

Go does not have classes, but it has methods. Methods are functions that are associated with a type.

```go
func (t Type) MethodName(args) returnType {
  // method body
}
```

```go
type Vertex struct {
  X, Y int
}

func (v Vertex) Add(w Vertex) Vertex {
  return Vertex{v.X + w.X, v.Y + w.Y}
}

func main() {
  v1 := Vertex{1, 2}
  v2 := Vertex{3, 4}
  result := v1.Add(v2)
  fmt.Println(result) // Output: {4 6}

  // Methods are also regular functions
  result2 := Add(v1, v2)
}
```

> Like rust structs, methods are associated with a type, not a class.

Methods can be declared on non-struct types too.

```go
type MyFloat float64

func (f MyFloat) IsZero() bool {
  return f == 0
}

func main() {
  f := MyFloat(0)
  fmt.Println(f.IsZero()) // true
}
```

### Pointer receivers

Methods with pointer receivers can modify the value they are called on.

```go
type Vetex struct {
  X, Y int
}

func (v *Vertex) Scale(factor int) {
  v.X *= factor
  v.Y *= factor
}

func main() {
  v := Vertex{3, 4} // &Vertex also works
  v.Scale(2)
  fmt.Println(v) // Output: &{6 8}
}
```

> Functions with a pointer argument must take a pointer.
> Methods with a pointer receiver can be called with either a value or a pointer.
>
> Functions that take a value argument must take a value of that type.
> Methods with a value receiver can be called with either a value or a pointer.

## Interfaces

An interface type is defined as a set of method signatures.

A value of interface type can hold any value that implements those methods.

```go
type Circle struct {
  Radius float64
}

type Rectangle struct {
  Width, Height float64
}

type Shape interface {
  Area() float64
}

// Shape interface is implemented by Circle (Area method)
func (c Circle) Area() float64 {
  return math.Pi * c.Radius * c.Radius
}

// Shape interface is implemented by Rectangle (Area method)
func (r Rectangle) Area() float64 {
  return r.Width * r.Height
}

// Accept any value that implements the Shape interface
func printArea(s Shape) {
  fmt.Println("Area:", s.Area())
}

func main() {
  c := Circle{Radius: 5}
  r := Rectangle{Width: 4, Height: 6}

  // printArea accepts both Circle and Rectangle
  printArea(c) // Area: 78.53981633974483
  printArea(r) // Area: 24
}
```

> Implementation is implicit. There is no "implements" keyword.

### Nil interface

A nil interface value holds neither value nor concrete type.

The method will be called with a nil receiver.

It is common in go to write methods that gracefully handle a nil receiver.

> A nil interface holds neither value nor concrete type.

### Empty interface

The interface type that specifies zero methods is known as the empty interface.

`interface{}`

Can hold any type.

> It can be used to handle values of unknown type.

## Type assertions

Provides access to an interface value's underlying concrete value.

`t := i.(T)`

The statement asserts that the interface value `i` holds the concrete type `T` and assigns the underlying value to `t`.

> If `i` does not hold a `T`, the statement will trigger a panic.

### Checking the type

A type assertion can return two values:
the underlying value and a boolean value that reports whether the assertion succeeded.

`t, ok := i.(T)`

* If `i` holds a `T`, then `t` will be the underlying value and `ok` will be true.
* If not, `t` will be the zero value of type `T` and `ok` will be false. No panic occurs.

## Type switches

A construct that permits several type assertions in series.

```go
switch v := i.(type) {
  case T:
    // v has type T
  case S:
    // v has type S
  case nil:
    // i is nil
  default:
    // no match; v has the same type as i
}
```

## Stringer

`Stringer` is an interface defined in the `fmt` package.

```go
type Stringer interface {
  String() string
}
```

It is a type that can describe itself as a string.
The `fmt` package (and many others) look for this interface to print values.

```go
type Person struct {
  Name string
  Age  int
}
func (p Person) String() string {
  return fmt.Sprintf("%v (%v years)", p.Name, p.Age)
}
func main() {
  a := Person{"Arthur Dent", 42}
  z := Person{"Zaphod Beeblebrox", 9001}
  fmt.Println(a, z)
}
```
