use anyhow::{bail, ensure, Context, Result};

use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use clap::{App, Arg, ArgMatches};

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

fn main() -> Result<()> {
    let opts = get_matches();

    if let Some(path) = opts.value_of("formula_file") {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.is_present("verbose"))
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.is_present("verbose"))
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;

        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(())
}

fn get_matches() -> ArgMatches {
    App::new("逆ポーランド記法コマンドラインツール")
        .version("1.0.0")
        .author("tochiji")
        .about("逆ポーランド記法で計算ができます")
        .arg(
            Arg::new("formula_file")
                .about("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("1 1 ^").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}
