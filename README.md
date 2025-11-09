# PNA-CC

## Status
- Minimal compiler focused on core syntax.
- Targets C++ only. (WASM backend removed.)

## Requirements
- Rust (stable)
- A C++20 compiler (GCC/Clang/MSVC; MinGW GCC recommended on Windows)

## Quick Start
```
cargo run -- <input.pna> -o out.cpp
g++ -std=c++20 out.cpp -o out && ./out
```

## Language (currently supported)

Object block:
```
user: {
  name: "Alice",
  age: 20,
}
```

Property assign:
```
user.age: 21
```

Scalar assign:
```
i: 0
```

Print:
```
log user.name
log upper user.name
log "hi " + user.name
```

Condition:
```
cond (user.age >= 18) -> {
  log "adult"
} else -> {
  log "minor"
}
end
```

Input:
```
input "Your name?" -> user.name
input "NO" -> cmd
```

Loop:
```
i: 0
loop (i < 3) -> {
  log i
  i: i + 1
}
end
```

While with ended, and control flow:
```
while (cond) -> {
  ...
} ended {
  ...
}
break
continue
```

## Functions (new)

### Overview
- Declare with `function` … `end`.
- Typed parameters and a typed return.
- Supported types (for now): `int`, `double`, `string`, `void`
- Call by value (arguments are evaluated left-to-right).
- Each call runs in a fresh local environment; outer variables can be **read** but not mutated (writes create/override locals).
- Return value must match the declared return type; for `void`, you may use `return` with no expression, or just fall through the end.

### Syntax
```pna
function add(a:int, b:int) -> int {
  return a + b
} end

x: add(2, 3) # 5
```

### Type notes
- `int` / `double` follow standard arithmetic promotions (`+ - * / %`, comparisons).
- `string + string` concatenates.
- `string * int` repeats (`int < 0` behaves as 0).
- `void` functions produce no value; using the result in an expression is invalid.

---

## Expressions
- Literals: numbers, strings (`"..."`), booleans (`true`/`false`)
- Operators: `+ - * / %`, `== != < <= > >=`, logical `&& || !`
- String concatenation: `+`
- Variable/member: `user.name`, `i`

## I/O Semantics
- `input "<prompt>" -> target`
- Reads **one token** (whitespace-delimited) from `stdin`.
- If the token parses as a number, it’s stored as number; otherwise as string.
- `log expr` prints with minimal formatting:
- Numbers that are “effectively integers” print without a decimal point.
- Other numbers trim trailing zeros.
