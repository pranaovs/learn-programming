# Go Errors

## Error type

The `error` type is a built-in interface.

```go
type error interface {
  Error() string
}
```

The `fmt` package looks for the `error` interface when printing values.

```go
i, err := strconv.Atoi("42")
if err != nil {
  fmt.Printf("Couldn't convert number: %v\n", err)
  return
}
fmt.Println("Converted integer:", i)
```

> `nil` error denotes success
