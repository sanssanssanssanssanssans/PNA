# PNA (한국어 안내)

PNA 언어를 C++ 단일 파일(out.cpp)로 변환하는 간단한 컴파일러입니다. 생성된 파일은 작은 런타임을 포함하며 C++20 컴파일러로 바로 빌드/실행할 수 있습니다.

## 상태
- 핵심 문법 위주의 최소 구현
- C++ 타깃만 지원 (WASM 백엔드 제거)

## 요구사항
- Rust (stable)
- C++20 컴파일러 (Windows는 MinGW GCC 권장)

## 빠른 시작
```
cargo run -- <input.pna> -o out.cpp
g++ -std=c++20 out.cpp -o out && ./out
```

## 지원 문법

객체 블록:
```
user: {
  name: "Alice",
  age: 20,
}
```

속성 대입:
```
user.age: 21
```

스칼라 대입:
```
i: 0
```

출력:
```
log user.name
log upper user.name
log "hi " + user.name
```

조건:
```
cond (user.age >= 18) -> {
  log "adult"
} else -> {
  log "minor"
}
end
```

입력:
```
input "Your name?" -> user.name
input "NO" -> cmd
```

반복문:
```
i: 0
loop (i < 3) -> {
  log i
  i: i + 1
}
end
```

while/ended 및 제어문:
```
while (cond) -> {
  ...
} ended {
  ...
}
break
continue
```

표현식:
- 리터럴: 숫자, 문자열("..."), 불리언(true/false)
- 연산자: + - * / %, 비교 == != < <= > >=, 논리 && || !
- 문자열 + 연결
- 변수 참조: user.name, i

## 예제
```
cargo run -- examples/02_cond.pna -o out.cpp
g++ -std=c++20 out.cpp -o out && ./out
```

## 프로젝트 구조
- src/ — lexer, parser, AST, C++ 코드 생성
- examples/ — 샘플 PNA 프로그램
- scripts/ — e2e 테스트 스크립트
- .github/workflows/ci.yml — 포맷/클리피/예제 실행 CI

## 기여
CONTRIBUTE.md 참고.

## 라이선스
MIT
