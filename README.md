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

### V8 (0.75.0)

```
V8 Time for "test/add.js": 980.329277ms
V8 Time for "test/fib.js": 1.793192404s
V8 Time for "test/ray.js": 2.004268899s
```

### JavaScriptCore (0.1.0)

```
JSC Time for "test/add.js": 977.229438ms
JSC Time for "test/fib.js": 1.634471474s
JSC Time for "test/ray.js": 2.006349927s
```

### Boa (0.17.0)

```
boa Time for "test/add.js": 438.969138ms
boa Time for "test/fib.js": 47.390937638s
boa Time for "test/ray.js": 35.36545172s
```
