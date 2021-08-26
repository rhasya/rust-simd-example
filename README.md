# Rust WASM32 SIMD Example
## 동작 환경
* Rust 1.54.0 이상
* Chrome / Edge 91 이상

## 컴파일
```
$ wasm-pack build --release --target web
```

## 서버 기동
```
$ npm start
```

## 접속 및 확인
* 현재 주소는 localhost:65501 로 해 두었으며 package.json에서 변경 가능

## 성능 비교
* Ryzen5 3600 에서 수행했을 시 시간 차이
* 그림 크기 : 3840x2160

|대상|시간|
|--|--|
|SIMD 사용시|30 ~ 35 ms|
|SIMD 미사용시|100 ~ 105ms|