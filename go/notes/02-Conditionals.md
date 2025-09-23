# Go Conditionals

## For loop

3 components separated by semicolons.

1. init statement: executed before first iteration
2. condition expression: evaluated before every iteration
3. post statement: executed at the end of each statement

> There are no parenthesis surrounding the statements
> Braces `{ }` are always required

```go
func main() {
  sum := 0
  for i := 0; i < 10; i++ {
    sum += 1
  }
  fmt.Println(sum)
}
```

### While loop

* init and post statements are optional

```go
func main() {
  sum := 1
  for sum < 1000 {
    sum += sum
  }
  fmt.Println(sum)
}
```

### Infinite Loop

```go
fun main() {
  for {
    // Infinite loop
  }
}
```

## If loop

Parenthesis not required but braces are required

```go
func main() {
  if 1 < 2 {
    fmt.Println("Yea 2 is greater")
  }
}
```

`if` statement can start with a short statement.
Variables declared by this statement are valid till the scope of `if`

```go
func pow(x, n, lim float64) float64 {
  if v := math.Pow(x, n); v < lim {
    return v
  }
  return lim
}
```

### If-else

Variables declared inside `if` short statement are accessible inside the else blocks.

```go
func pow(x, n, lim float64) float64 {
  if v := math.Pow(x, n); v < lim {
    return v
  } else {
    fmt.Printf("%g >= %g\n", v, lim)
  }
  return lim
}
```

## Switch

Shorter way to run a sequence of if-else.

Go only runs the first case that matches, not all cases.
`break` is provided implicitly.

Switch cases need not be constants, and values involved need not be integers.

Evaluates cases from top to bottom, stopping when a case succeeds.

```go
func main() {
  fmt.Print("Go runs on ")
  switch os := runtime.GOOS; os {
  case "darwin":
    fmt.Println("macOS.")
  case "linux":
    fmt.Println("Linux.")
  default:
    // freebsd, openbsd,
    // plan9, windows...
    fmt.Printf("%s.\n", os)
  }
}
```

> `switch` can have no condition. Equivalent to `switch true`
> Cleaner way to write long if-then-else chains

## Defer

Defer statement runs the statement just before the surrounding function returns.

`defer func1()`

> The arguments are evaluated immediately, but the function call is not executed until the surrounding function returns.

### Stacking defers

Deferred function calls are pushed into a stack.
When a function returns, they are executed in a FILO order.
