# PNA-JS (Portable Notation for Actions — JavaScript Interpreter)

**PNA**는 간단한 스크립트 문법을 추구하는 프로그래밍 언어입니다.
이 저장소는 **Node.js 런타임용 인터프리터**를 제공합니다.

## 설치 & 실행

```bash
# 프로젝트 폴더로 이동
npm install
npm run start     # examples/demo.pna 실행
# 또는
node ./bin/pna.js path/to/file.pna
```

# 문법 (기본)

## 1) 객체 블록
```pna
user: {
  name: "Alice",
  age: 20,
}
```

## 2) 속성 대입
```pna
user.age: 21
```

## 3) 스칼라 대입
```pna
i: 0
```

## 4) 출력
```pna
log user.name
log upper user.name
log "hi " + user.name
```

## 5) 조건
```pna
cond (user.age >= 18) -> {
  log "adult"
}
end
```

- Else 문
```pna
cond (user.age >= 18) -> {
  log "adult"
} else -> {
  log "minor"
}
end
```

## 6) 입력
```
input "Your name?" -> user.name
input "NO" -> cmd       # 프롬프트 없이 입력
```

## 7) 반복문
```pna
i: 0
loop (i < 3) -> {
  log i
  i: i + 1
}
end
```

## 8) 제어문
```pna
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
