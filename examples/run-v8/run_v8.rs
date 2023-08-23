fn eval(script: &str, _path: &str) {
    // Create a new Isolate and make it the current one.
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    // Create a stack-allocated handle scope.
    let handle_scope = &mut v8::HandleScope::new(isolate);

    // Create a new context.
    let context = v8::Context::new(handle_scope);

    // Enter the context for compiling and running the hello world script.
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    // Create a string containing the JavaScript source code.
    let source = v8::String::new(scope, script).unwrap();

    // Compile the source code.
    let script = v8::Script::compile(scope, source, None).unwrap();

    // Run the script to get the result.
    let value = script.run(scope);
    if let Some(value) = value {
        println!("Result: {}", value.to_rust_string_lossy(scope));
    }
}

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Get script path from args
    let args: Vec<String> = std::env::args().collect();
    let default_path = "test/add.js".to_string();
    let path = args.get(1).unwrap_or_else(|| &default_path);
    println!("Path: {:?}", path);

    let script = &std::fs::read_to_string(path).unwrap();

    eval(script, path);
}

#[cfg(test)]
mod tests {
    use super::eval;
    use std::io::Write;
    use std::sync::Once;

    fn prepare() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });
    }

    fn bench(path: &str, repeat: usize) {
        let script = &std::fs::read_to_string(path).unwrap();

        let start = std::time::Instant::now();
        for _ in 0..repeat {
            eval(script, path);
        }
        let end = std::time::Instant::now();

        writeln!(
            std::io::stdout(),
            "V8 Time for {:?}: {:?}",
            path,
            end - start
        )
        .unwrap();
    }

    #[test]
    fn v8_eval() {
        prepare();

        let path = "test/add.js";
        let script = &std::fs::read_to_string(path).unwrap();

        eval(script, path);
    }

    #[test]
    fn bench_v8_fib() {
        prepare();

        bench("test/fib.js", 1000);
    }

    #[test]
    fn bench_v8_add() {
        prepare();

        bench("test/add.js", 1000);
    }

    #[test]
    fn bench_v8_ray() {
        prepare();

        bench("test/ray.js", 1);
    }
}
