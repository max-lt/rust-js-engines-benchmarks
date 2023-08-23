use rusty_jsc::JSContext;

fn eval(script: &str, _path: &str) {
    let mut context = JSContext::default();

    let value = context.evaluate_script(script, 1);
    if let Ok(value) = value {
        println!("Result: {}", value.to_string(&mut context).unwrap().to_string());
    }
}


fn main() {
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

    fn bench(path: &str, repeat: usize) {
        let script = &std::fs::read_to_string(path).unwrap();

        let start = std::time::Instant::now();
        for _ in 0..repeat {
            eval(script, path);
        }
        let end = std::time::Instant::now();

        writeln!(std::io::stdout(), "JSC Time for {:?}: {:?}", path, end - start).unwrap();
    }

    #[test]
    fn jsc_eval() {
        let path = "test/add.js";
        let script = &std::fs::read_to_string(path).unwrap();

        eval(script, path);
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
