use rusty_jsc::JSContext;
use rusty_jsc::JSValue;

// #[macro_use]
use rusty_jsc_macros::callback;

#[callback]
fn write(
    ctx: JSContext,
    _function: JSObject,
    _this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    static mut C: i32 = 0;

    println!(
        "output #{}: Array<{}> {:?}",
        C,
        args.len(),
        args.iter()
            .map(|v| v.to_string(&ctx).unwrap())
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    C = C + 1;

    Ok(JSValue::undefined(&ctx))
}

fn eval(script: &str) {
    let mut context: JSContext = JSContext::default();

    // Add write function
    let global = context.get_global_object();
    let print = JSValue::callback(&context, Some(write));
    global.set_property(&context, "write", print).unwrap();

    let value = context.evaluate_script(script, 1);

    match value {
        Ok(value) => {
            let ret = regex::Regex::new(r"[\n\r]").unwrap();
            let msp = regex::Regex::new(r" {2,}").unwrap();

            let value = value.to_string(&mut context).unwrap().to_string();
            let value = ret.replace_all(&value, " ").to_string();
            let value = msp.replace_all(&value, " ");

            println!("Result: {}", value);
        }
        Err(error) => {
            println!("Error: {:?}", error.to_string(&mut context).unwrap().to_string());
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
            "JSC Time for {:?}: {:?}",
            path,
            end - start
        )
        .unwrap();
    }

    #[test]
    fn jsc_eval() {
        let path = "test/add.js";
        let script = &std::fs::read_to_string(path).unwrap();

        eval(script);
    }

    #[test]
    fn bench_jsc_fib() {
        bench("test/fib.js", 1000);
    }

    #[test]
    fn bench_jsc_add() {
        bench("test/add.js", 1000);
    }

    #[test]
    fn bench_jsc_ray() {
        bench("test/ray.js", 1);
    }
}
