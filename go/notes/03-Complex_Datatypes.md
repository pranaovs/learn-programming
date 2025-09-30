# Go Complex Datatypes

## Pointers

`vat p *int`

```go
i := 42
p = &i

fmt.Println(*p)
*p = 21 // Change the value at the address
```

## Structs

```go
type Vertex struct {
  X int
  Y int
}

func main() {
  fmt.Println(Vertex{1, 2})
  v := Vertex{1,3}
  v.X = 4
  fmt.Println(v.X) // 4
}

```

### Pointers to Structs

```go
type Vertex struct {
  X int
  Y int
}

func main() {
  v := Vertex{1,2}
  p := &v
  p.X = 1e9 // Same as (*p).X = 1e9
  fmt.Println(v) // {1000000000 2}
}
```

### Struct Literals

```go
type Vertex struct {
  X, Y int
}

var (
  v1 = Vertex{1, 2}
  c2 = Vertex{x: 1}
  v3 = Vertex{}
  p = &Vertex{1, 2} // Pointer to struct. Type is *Vertex
)

func main() {
  fmt.Println(v1, p, v2, v3) // {1 2} &{1 2} {1 0} {0 0}
}
```

## Arrays

`var a [10]int`

```go
func main() {
  var a [2]string
  a[0] = "Hello"
  a[1] = "World"
  fmt.Println(a) // [Hello World]

  primes :=  [6]int{2,3,5,7,11,13}
}
```

Arrays cannot be resized.

### Array Literals

`[3]bool{true, true, false}`

## Slices

A slice is a dynamically-sized flexible view into the elements of an array.

`[]T` is a slice with elements of type `T`.

`a[low : high]`. Includes first element, excludes last one.

`a[1:4]` creates a slice which includes elements 1, 2, 3 of array `a`.

A slice does not store any data, it only references an array.

```go
func main() {
  names := [4]string(
  "Adam"
  "Bella"
  "Castella"
  "Donny"
  )

  a := names[0:2]

  a[0] = "Eve"

  fmt.Println(a) // [Eve Bella]

}
```

### Slice Literal

An array literal without length

`[]bool{true, true, false}`

```go
func main() {
  q := []int{2,3,5,7,11,13}
  fmt.Println(q) // [2 3 5 7 11 13]

  s := []struct {
    i int
    b bool
  }{
    {2, true},
    {3, true},
    {4, false},
    {5, true}
  }
  fmt.Println(s) // [{2 true} {3 true} {4 false} {5 true}]
}
```

### Slice defaults

* Lower bound = 0
* Higher bound = length of array

```go
a[0:10]
a[:10]
a[0:]
a[:]
```

### Slice length and capacity

* Length of slice = number of elements it contains
  * `len(s)`
  * Can be extended by resizing it
* Capacity of slice = number of elements in the underlying array, counting from the first element in the slice
  * `cap(s)`

```go
  func main() {
  s := []int{2, 3, 5, 7, 11, 13} // len=6 cap=6

  // Slice the slice to give it zero length.
  s = s[:0] // len=0 cap=6

  // Extend its length.
  s = s[:4] // len=4 cap=6

  // Drop its first two values.
  s = s[2:] // len=2 cap=4
}
```

### Nil slices

Zero value of a slice is `nil`

Length is 0 and no underlying array

```go
func main() {
  var s[] int
  if s == nil {
    fmt.Println("nil") // nil
}
```

### Creating a slice with make

`make` function allocates a zeroed array and returns a referring slice.

```go
func main() {
  a := make([]int, 5) // len(a)=5
  b := make([]int, 0, 5) // len(b)=0, cap(b)=5
  b = b[:cap(b)]    // len(b)=5, cap(b)=5
  b = b[1:]   // len(b)=4, cap(b)=4
}
```

> Slices can contain any type, including other slices

### Appending to a slice

`append` function.

`func append(s []T, vs ...T) []T`

The resulting value of append is a slice containing all the elements of the original slice plus the provided values.

If the backing array of s is too small to fit all the given values a bigger array will be allocated. The returned slice will point to the newly allocated array.

```go
func main() {

  var s []int // len=0 cap=0 []

  // append works on nil slices.
  s = append(s, 0) // len=1 cap=1 [0]

  // The slice grows as needed.
  s = append(s, 1) // len=2 cap=2 [0 1]

  // We can add more than one element at a time.
  s = append(s, 2, 3, 4) // len=5 cap=6 [0 1 2 3 4]
}
```

## Range

The `range` form of `for` loop iterates over a [slice](#slices) or a [map](#maps).

```go
func main() {
  pow := []int{1, 2, 4, 8, 16, 32, 64, 128}

  for i, v := range pow {
    fmt.Printf("2**%d = %d\n", i , v)
  }
}
```

> Index or value can be slipped by assigning it to _.
>
> `for _, value := range pow`
>
> Second variable (value) can be omitted if only index is needed.

## Maps

Map keys to values

Zero value of a map is `nil`.

```go
type Vertex struct {
  Lat, Long float64
}
var m map[string]Vertex

func main() {
  m = make(map[string]Vertex)
  m["Bell Labs"] = Vertex{
    40.68433, -74.39967,
  }
  fmt.Println(m["Bell Labs"]) // {40.68433 -74.39967}
}
```

### Map literals

Like struct literals but with keys and values

```go
var m = map[string]Vertex{
  "Bell Labs": Vertex{
    40.68433, -74.39967,
  },
  "Google": Vertex{
    37.42202, -122.08408,
  },
}
```

```go
var m = map[string]Vertex{
  "Bell Labs": {40.68433, -74.39967},
  "Google":    {37.42202, -122.08408},
}
```

### Map operations

* Insert or update a value: `m[key] = value`
* Retrieve a value: `value = m[key]`
* Delete a key: `delete(m, key)`
* Test for existence: `value, ok = m[key]`
  * If key exists, `ok` is true and `value` is the value associated with the key.
  * If key does not exist, `ok` is false and `value` is the zero value for the map's value type.
  * `elem, ok := m[key]` idiomatic way.
