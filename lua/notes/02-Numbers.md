# Numbers

Numbers are represented by 64-bit integers called _integers_,
or double-precision floating-point numbers called _floats_ (since Lua 5.3).

## Numerals

- `4`
- `0.4`
- `3.12e-2`
- `0.3e2`
- `5E+12`

Numbers with decimal points or exponents are floats.

Both values have same type `"number"`.

Numbers and floats of same value are equal:

```lua
1 == 1.0 -- true
0.2e3 == 200 -- true
```

```lua
10 -- 10
100.0 -- 100.0
2e3 -- 2000.0
0xff -- 255
0x1A4 -- 420
0x0.2 -- 0.123
```

`math.type` function can be used to distinguish between integers and floats:

```lua
math.type(1) -- "integer"
math.type(1.0) -- "float"
```

`string.format` can be used to preserve numbers with full precision:

```lua
string.format("%a", 419) -- 0x1.a3p+8
string.format("%a", 0.1) -- 0x1.999999999999ap-4
```

## Arithmetic Operators

> "The programmer may choose to mostly ignore the difference between integers and floats or to assume complete comtrol over the representation of each number"

Operations on two integers return an integer, except for division.
**Division always returns a float**
Operations on two floats always return a float.
Operations on an integer and a float return a float.

```lua
13 + 15 -- 28
13.0 + 15.0 -- 28.0
13.0 + 25 -- 38.0
-(3 * 6.0) -- -18.0.
3.0 / 2 -- 1.5
3 / 2 -- 1.5
4 / 2 -- 2.0

3 // 2 -- 1
3.0 // 2 -- 1.0
6.0 // 2.0 -- 3.0

2 ^ 3 -- 8
16 ^ (1/ 2) -- 4.0
```

## Relational Operators

- `<` (less than)
- `>` (greater than)
- `<=` (less than or equal to)
- `>=` (greater than or equal to)
- `==` (equal to)
- `~=` (not equal to)

Comparison disregards the type of the number, so `1 == 1.0` is true.

## `math` library

The `math` library provides standard mathematical functions and constants:

### Constants

- `math.pi` - The value of π (3.14159...)
- `math.huge` - Positive infinity
- `math.maxinteger` - Maximum value for an integer
- `math.mininteger` - Minimum value for an integer

### Basic Functions

- `math.abs(x)` - Returns the absolute value of x
- `math.ceil(x)` - Returns the smallest integer greater than or equal to x
- `math.floor(x)` - Returns the largest integer less than or equal to x
- `math.max(x, ...)` - Returns the maximum value among its arguments
- `math.min(x, ...)` - Returns the minimum value among its arguments
- `math.fmod(x, y)` - Returns the remainder of the division of x by y
- `math.modf(x)` - Returns the integral part of x and the fractional part of x
- `math.type(x)` - Returns "integer" if x is an integer, "float" if it's a float, or nil if not a number

### Exponents and Logarithms

- `math.exp(x)` - Returns e^x
- `math.log(x [, base])` - Returns the logarithm of x in the given base (default is e)
- `math.log10(x)` - Returns the base-10 logarithm of x
- `math.pow(x, y)` - Returns x^y
- `math.sqrt(x)` - Returns the square root of x

### Trigonometric Functions

- `math.cos(x)` - Returns the cosine of x (x in radians)
- `math.sin(x)` - Returns the sine of x (x in radians)
- `math.tan(x)` - Returns the tangent of x (x in radians)
- `math.acos(x)` - Returns the arc cosine of x (in radians)
- `math.asin(x)` - Returns the arc sine of x (in radians)
- `math.atan(x)` - Returns the arc tangent of x (in radians)
- `math.atan2(y, x)` - Returns the arc tangent of y/x (in radians)

### Random Numbers

- `math.random([m [, n]])`:
  1. Returns a random number between m and n (inclusive) `[m,n]`.
  2. If m is provided but n is not, returns a float between 1 and m `[1,m]`.
  3. If m and n are not provided, returns a float between 0 and 1 `[0,1]`.
- `math.randomseed(x)` - Sets the seed for the random generator.
  Recommended to use `math.randomseed(os.time())`.

### Other Functions

- `math.deg(x)` - Converts angle x from radians to degrees
- `math.rad(x)` - Converts angle x from degrees to radians
- `math.tointeger(x)` - Converts x to an integer if possible, or returns nil
- `math.ult(m, n)` - Returns a boolean, true if integer m is below n when compared as unsigned integers

## Conversions

1. Force a number to be a float: add `0.0` to it: `-4 + 0.0 -- -4.0`
2. Force a number to be integer:
   1. OR it with `0`: `-4.0 | 0 -- -4`
   2. use `math.tointeger`: `math.tointeger(-4.0) -- -4`
      - Returns `nil` if the number is not an integer. `math.tointeger(4.5)` returns `nil`.
   3. use `math.floor`: `math.floor(-4.0) -- -4` (rounds down)

## Precedence

Lua operators are evaluated in the following order (from highest to lowest precedence):

1. `^` (exponentiation)
2. Unary operators (`-`, `#`, `~`, `not`)
3. `*`, `/`, `//`, `%` (multiplication, division, floor division, modulo)
4. `+`, `-` (addition, subtraction)
5. `..` (concatenation)
6. `<<`, `>>` (bitwise shifts)
7. `&` (bitwise AND)
8. `~` (bitwise exclusive OR)
9. `|` (bitwise OR)
10. `<`, `>`, `<=`, `>=`, `~=`, `==` (comparison)
11. `and` (logical AND)
12. `or` (logical OR)

Operators with the same precedence are evaluated from left to right, except for `^` and `..` which are right associative.

> Recommended to use parentheses to clarify the order of operations, especially in complex expressions.
