# Strings

Strings in Lua are immutable.
To change a value in string, we create a new string with the desired changes.

```lua
a = "one string"
print(a) -- one string
a = string.gsub(a, "one", "another")
print(a) -- another string
```

Length of a string can be obtained using the `#` operator.

```lua
a = "hello"
print(#a) -- 5
print(#"lmao") -- 4
```

> This operator counts the number of bytes in the string, not the number of characters.

## String Literals

Single quotes and double quotes can be used interchangeably to define strings.

```lua
a = 'single quotes'
b = "double quotes"
```

C-like escape sequences can be used in strings.

## Long Strings

Strings can be defined using double square brackets `[[ ]]` to create long strings that span multiple lines.

```lua
a = [[This is a long string
  which can span multiple lines
  and contain "quotes" without escaping.]]
```

To embed `[[` inside a long string,
we can use `[==[` or with more `=` signs to increase the level of nesting.

```lua
long = [===[ this is a long string
  with [[nested long strings]]
  and it can be as long as needed ]===]
```

- It can be used for comments also.

## Coercions

Lua will automatically convert strings to numbers when needed, such as in arithmetic operations,
and convert numbers to strings when concatenating with strings.

```lua
print(10.."20") -- 1020
print("10" + 20) -- 30
print("10" + "20") -- 30
print(tostring(10) == "10") -- true
```

`tostring` can be used to explicitly convert values to strings.

`tonumber` can be used to convert strings to numbers.
It accepts an optional second argument to specify the base (default is base 10).

## String Library
