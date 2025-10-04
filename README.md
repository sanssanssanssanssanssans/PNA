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

## 표현식
- 숫자, 문자열("..."), 불리언(true/false)
- 연산자: + - * / %, 비교 == != < <= > >=, 논리 && || !
- 문자열 + 연결 지원
- 변수 참조:
    - 객체 속성: user.name
    - 스칼라 변수: i
