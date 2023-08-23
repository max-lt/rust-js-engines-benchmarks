# Benchmark of different JS engines available in Rust


## Engines

- [V8](https://v8.dev/)
- [JavaScriptCore](https://developer.apple.com/documentation/javascriptcore)
- [Boa](https://boajs.dev/)



## Binary size

```
jsc 4.2M
v8  44M
boa 8.6M
```


## Results

### V8

```
V8 Time for "test/add.js": 980.329277ms
V8 Time for "test/fib.js": 1.793192404s
V8 Time for "test/ray.js": 2.004268899s
```

### JavaScriptCore

```
JSC Time for "test/add.js": 977.229438ms
JSC Time for "test/fib.js": 1.634471474s
JSC Time for "test/ray.js": 2.006349927s
```

### Boa

```
boa Time for "test/add.js": 263.384854ms
boa Time for "test/fib.js": 34.251084838s
boa Time for "test/ray.js": 3.974487ms <-- seems to fail
```
