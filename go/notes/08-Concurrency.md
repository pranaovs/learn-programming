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

## Range and Close

A sender can close a channel to indicate that no more values will be sent.

`v, ok := <-ch`

`ok` is false if no more values and channel is closed.

> Only the sender should close a channel, never the receiver.
> Sending on a closed channel causes a panic.
>
> Channels are not like files, you don't usually need to close them.
> Closing is necessary to tell the receiver that no more values will be sent.

```go
func fibonacci(n int, c chan int) {
  x, y := 0, 1
  for i := 0; i < n; i++ {
    c <- x
    x, y = y, x+y
  }
  close(c)
}
func main() {
  c := make(chan int, 10)
  go fibonacci(cap(c), c)
  for i := range c { // equivalent to for { v, ok := <-c; if !ok { break } ... }
    fmt.Println(i)
  }
}
```

## Select

`select` lets a goroutine wait on multiple communication operations.

It blocks until one of its cases can run, then it executes that case.

```go
func main() {
  tick := time.Tick(100 * time.Millisecond) // tick is a channel. Receives a value every 100ms
  boom := time.After(500 * time.Millisecond) // boom is a channel. Receives a value after 500ms and only once
  for {
    select {
    case <-tick: // run when a value is received from tick channel (every 100ms)
      fmt.Println("tick.")
    case <-boom: // run when a value is received from boom channel (500ms)
      fmt.Println("BOOM!")
      return
    default: // run if neither tick nor boom has a value ready
      fmt.Println("    .")
      time.Sleep(50 * time.Millisecond)
    }
  }
}
```

The `default` case runs if no other case is ready.
It blocks until one of its cases can run, then it executes that case.

## Mutex

Mutex (Mutual Exclusion) is a lock that can be used to protect shared data from being simultaneously accessed by multiple goroutines.

sync.Mutex has two methods: `Lock` and `Unlock`.

A block of code can be `Lock`ed so that only one goroutine can access it at a time.

`defer` can be used to ensure that the `Unlock` method is called even if the function returns early due to an error or panic.

```go
func main() {
  var mu sync.Mutex
  var balance int

  // deposit adds amount to the account balance.
  deposit := func(amount int) {
    mu.Lock()
    defer mu.Unlock()
    fmt.Printf("Adding %d to balance %d\n", amount, balance)
    balance += amount
  }

  // withdraw subtracts amount from the account balance.
  withdraw := func(amount int) {
    mu.Lock()
    defer mu.Unlock()
    fmt.Printf("Withdrawing %d from balance %d\n", amount, balance)
    balance -= amount
  }

  var wg sync.WaitGroup // use WaitGroup to wait for all goroutines to finish
  for i := 0; i < 10; i++ {
    wg.Add(1) // increments the WaitGroup counter
    go func(i int) {
      defer wg.Done() // decrements the counter when the goroutine completes
      deposit(100)
      withdraw(50)
    }(i)
  }
  wg.Wait() // ensures that main waits for all goroutines to finish
  fmt.Println("Final balance:", balance)
}
```
