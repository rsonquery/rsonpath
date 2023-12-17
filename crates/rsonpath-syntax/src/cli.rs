use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("provide exactly one argument, the query string");
        return ExitCode::FAILURE;
    }

    let input: &str = &args[1];

    let res = rsonpath_syntax::parse(input);

    match res {
        Ok(x) => println!("OK: {}\nINPUT: {input}", x),
        Err(err) => println!("ERR: {}\nINPUT: {input}", err.colored()),
    }

    ExitCode::SUCCESS
}
