use boa::Context;
use boa::JsResult;
use boa::JsValue;
use boa::NativeFunction;
use boa_engine as boa;

fn write(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
    static mut C: i32 = 0;

    println!(
        "output #{}: Array<{}> {:?}",
        unsafe { C },
        args.len(),
        args.iter()
            .map(|v| v.to_string(context).unwrap())
            .map(|s| s.to_std_string_escaped())
            .collect::<Vec<_>>()
            .join(" ")
    );

    unsafe { C = C + 1 };

    Ok(JsValue::undefined())
}

fn eval(script: &str) {
    let mut context = Context::builder().build().unwrap();

    // // Set the context's runtime limits, awesome !!! 
    // context.runtime_limits_mut().set_loop_iteration_limit(10);
    // context.runtime_limits_mut().set_stack_size_limit(1000);
    // context.runtime_limits_mut().set_recursion_limit(10);

    let source = boa::Source::from_bytes(script);

    // Add write function
    let write: NativeFunction = NativeFunction::from_fn_ptr(write);
    context
        .register_global_builtin_callable("write", 99, write)
        .unwrap();

    let value = context.eval(source);
    match value {
        Ok(value) => {
            println!(
                "Result: {:?}",
                value
                    .to_string(&mut context)
                    .unwrap()
                    .to_std_string()
                    .unwrap()
            );
        }
        Err(error) => {
            println!("Error: {:?}", error.to_string());
        }
    }
}

fn main() {
    // Get script path from args
    let args: Vec<String> = std::env::args().collect();
    let default_path = "test/add.js".to_string();
    let path = args.get(1).unwrap_or_else(|| &default_path);
    println!("Path: {:?}", path);

    let script = &std::fs::read_to_string(path).unwrap();

    eval(script);
}

#[cfg(test)]
mod tests {
    use super::eval;
    use std::io::Write;

    fn bench(path: &str, repeat: usize) {
        let script = &std::fs::read_to_string(path).unwrap();

        let start = std::time::Instant::now();
        for _ in 0..repeat {
            eval(script);
        }
        let end = std::time::Instant::now();

        writeln!(
            std::io::stdout(),
            "boa Time for {:?}: {:?}",
            path,
            end - start
        )
        .unwrap();
    }

    #[test]
    fn boa_eval() {
        let path = "test/add.js";
        let script = &std::fs::read_to_string(path).unwrap();

        eval(script);
    }

    #[test]
    fn bench_boa_fib() {
        bench("test/fib.js", 1000);
    }

    #[test]
    fn bench_boa_add() {
        bench("test/add.js", 1000);
    }

    #[test]
    fn bench_boa_ray() {
        bench("test/ray.js", 1);
    }
}
