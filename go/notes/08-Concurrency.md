# Go Concurrency

## Goroutine

A _goroutine_ is a thread managed by Go runtime.

`go f(x,y,z)` starts a new goroutine running `f(x,y,z)`

`f`,`x`,`y`,`z` happens in the current goroutine and the evaluation of `f` happens in a new goroutine.

The address space is shared, so access to shared memory must be synchronized.
`sync` package provides useful primitives (but they won't be needed much).


## Channels

Channels are a typed conduit through which values can be sent and received with the channel operator `<-`.

```go
ch <- v // Send v to channel ch
v := <-ch // Receive from ch, and assign to v
```

Channels must be created before use:
`ch := make(chan int)`

Sends and receives are blocked until the other side is ready.
This allows sync without locks.

```go
func sum(s []int, c chan int) {
  sum := 0
  for _, v := range s {
    sum += v
  }
  c <- sum // send sum to c
}

func main() {
  s := []int{7, 2, 8, -9, 4, 0}

  c := make(chan int)
  go sum(s[:len(s)/2], c)
  go sum(s[len(s)/2:], c)
  x, y := <-c, <-c // receive from c

  fmt.Println(x, y, x+y)
}
```

### Buffered channels

`ch := make(chan int, 100)`

Sends to a buffer channel block only when the buffer is full.
Receives block when the buffer is empty.

