# Go Types

## Type parameters

Go functions can work on multiple types using type parameters.

```go
func Index[T comparable](s []T, x T) int
```

* `s` is a slice of any type `T` that fulfills the
built-in constraint `comparable`.
* `x` is also a value of the same type.

`comparable` is a constraint that makes it possible to use `==` and `!=` operators.

`Index` function works for any type that supports comparison.

## Generic Types

A type can be parameterized with a type parameter,
which can be useful for implementing generic data structures.

```go
type List[T any] struct {
  next *List[T]
  val  T
}

```
