use std::io::Read;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 1 {
        eprintln!("no arguments are expected");
        return ExitCode::FAILURE;
    }

    let input = {
        let mut buf = String::new();
        if let Err(err) = std::io::stdin().read_to_string(&mut buf) {
            eprintln!("error reading stdin: {err}");
            return ExitCode::FAILURE;
        }
        buf
    };

    let res = rsonpath_syntax::parse(&input);

    match res {
        Ok(x) => println!("OK: {x:?}\nDISPLAY:{x}\nINPUT: {input}"),

        Err(err) => {
            println!("DBGERR: {err:?}");
            #[cfg(feature = "color")]
            println!("ERR: {}\nINPUT: {input}", err.colored());
            #[cfg(not(feature = "color"))]
            println!("ERR: {err}\nINPUT: {input}");
        }
    }

    ExitCode::SUCCESS
}
