# Go Readers

## Readers

`io` package specifies `io.Reader` interface,
which represents the read end of a stream of data.

```go
func (T) Read(b []byte) (n int, err error)
```

Populates the given byte slice with data
and returns the number of bytes populated and error.

`io.EOF` returned when the stream ends.

```go
func main() {
  r := strings.NewReader("Hello, Reader!")
  b := make([]byte, 8)

  for {
    n, err := r.Read(b)
    n, err := r.Read(b)
    fmt.Printf("n = %v err = %v b = %v\n", n, err, b)
    fmt.Printf("b[:n] = %q\n", b[:n])
    if err == io.EOF {
      break
    }
  }
}
```

