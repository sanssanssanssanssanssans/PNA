# Contribute to PNA-JS

감사합니다! PNA-JS는 간단한 DSL(PNA)의 **Node.js 인터프리터 + 유틸리티(트랜스파일러)** 프로젝트입니다.  
이 문서는 로컬 개발, 코드 규칙, 테스트, 문법/내장함수 추가 방법을 안내합니다.

## 0. 철학
- **조심스러운 확장**: PNA는 "작고 읽기 쉬운" 언어입니다. 문법 확장은 단순성/일관성을 해치지 않도록 PR에서 동기·대안을 설명해주세요.
- **명시적 동작**: 암묵적 타입 변환은 최소화하고, README/문법서에 명확히 기록합니다.
- **양방향 호환**: 가능한 한 Python 인터프리터와 동작을 맞춥니다(아래 호환 섹션 참고).

## 1. 개발 세팅
```bash
git clone <repo>
cd pna-js
npm i
npm run start 
node bin/pna.js <file.pna>
```

## 2. 코딩 규칙
- ESM 사용
- 테스트는 최소한의 **exmaples/**의 스크립트를 추가하고 README.md에 사용예시를 보강합니다.
- 커밋 메세지 규칙 (권장사항)
    - feat(parse) : add ternary operator
    - fix(builtins) : clamp randint inclusive bounds
    - docs(readme): expand examples

## 3. 스크립트

- npm run demo : 예제 실행 (examples/demo.pna)
- npm run lint / npm run lint:fix
- npm test : 간단 실행 확인(데모 성공 여부)