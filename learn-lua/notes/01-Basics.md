# Lua Basics

`print("string")` - prints a string to the console

To run a Lua script, use the command:

```bash
lua script.lua
```

- Chunk: A chunk is a piece of code that can be executed. It can be a single line or multiple lines.

Lua is case-sensitive

Lua needs no separator between statements,
but newlines or semicolons `;` can be used to separate them.

```lua
a = 10; b = 20; print(a + b)

c = 10 d = a * 2 -- valid
```

A semicolon makes a line invalid as an expression, but it is valid as a command.

```sh
lua -i
> io.flush()
true
> io.flush(); -- Does not print anything
```

There are reserved words in Lua that cannot be used as variable names.

- and
- break
- do
- else
- elseif
- end
- false
- for
- function
- goto
- if
- in
- local
- nil
- not
- or
- repeat
- return
- then
- true
- until
- while

## Comments

A comment starts with two consecutive hyphens `--` and continues to the end of the line.

```lua
-- This is a comment
```

```lua
print("Hello, World!") -- This prints "Hello, World!" to the console
```

A multi-line comment starts with `--[[` and ends with `--]]`.

```lua
--[[ A multi-line comment
   that spans multiple lines
   ]]
```

A common trick is to use a multi-line comment to temporarily disable a block of code:

```lua
--[[
  print("This line is commented out")
--]]
```

to uncomment it, simply add a `-` at the beginning of the line.

```lua
---[[ This multi-line comment would be commented out by --
  print("This line is uncommented")
--]]
```

## Variables

Variables are defined using `var = value`

Every variable in Lua is a global variable by default.

```lua
a = 10
```

To create a local variable, use the `local` keyword:

```lua
local b = 20
```

To access a global variable, you can simply use its name

To access a local variable, you must be within the same scope where it was defined:

```lua
print(a) -- prints 10
```

### Types

Lua is a dynamically typed language.
There are no type declarations in Lua.

There are 8 basic types in Lua:

1. `nil` - represents the absence of a value
2. `boolean` - represents true or false
3. `number` - represents a number (integer or float)
4. `string` - represents a sequence of characters
5. `function` - represents a function
6. `userdata` - represents a user-defined data type
7. `thread`- represents a thread of execution
8. `table` - represents a collection of key-value pairs

```lua
type(nil) -- nil
type(true) -- boolean
type(10) -- number
type("Hello") -- string
type(print) -- function
type({}) -- table
type(coroutine.create(function() end)) -- thread
type(io.stdin) -- userdata
type(io) -- userdata
type(type(true)) -- string
```

- The result of `type()` is a string that represents the type of the value.

Any variable can hold any type of value, and the type can change at runtime.

#### `nil`

If `nil` is assigned to a variable, Lua behaves like the variables is not initialized, and the memory is reclaimed.
Lua does not differentiate between uninitialized and explicitly set to `nil`.

```lua
b -- nil
b = 10
b -- 10
b = nil
b -- nil
```

#### `boolean`

Any value can represent a condition in Lua.

Conditional tests consider both `nil` and `false` as false, while all other values are considered true.

Conventional operators like `and`, `or`, and `not` can be used to combine boolean values.

- Both `and` and `or` use short-circuit evaluation.

##### `and` operator

The result of the `and` operator is the first operand if it is false, otherwise it is the second operand.

```lua
4 and 5 -- 5
nil and 5 -- nil
```

##### `or` operator

The result of the `or` operator is the first operand if it is not false (true), otherwise it is the second operand.

```lua
false or "hi" -- "hi"
0 or "hi" -- 0
```

##### `not` operator

The `not` operator negates a boolean value.
It always gives a boolean result.

```lua
not true -- false
not false -- true
not 0 -- false
not nil -- true
not not nil -- false
```

## Lua Interpreter

`#!/usr/bin/env lua` - This is a shebang line that allows the script to be run as an executable on Unix-like systems.

`lua -e` can be used to execute a Lua expression directly from the command line.

```sh
lua -e 'print("Hello, World!")'
```

`lua -i` starts an interactive Lua interpreter session,
after running other arguments.

`lua -e "sin=math.sin" script a b` will collect create the arguments table like:

- `arg[-3]` = `"lua"`
- `arg[-2]` = `"-e"`
- `arg[-1]` = `"sin=math.sin"`
- `arg[0]` = `"script"`
- `arg[1]` = `"a"`
- `arg[2]` = `"b"`
