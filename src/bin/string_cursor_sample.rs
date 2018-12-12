use std::io::{BufRead, Cursor};

fn main() {
    let target_str = "\
        include \"./foo.conf\"
        foo {
            bar.bazz {
                value = \"ああああ\"

                arr = []
            }
        }



aaa = \"aaa\"

    "
    .to_string();
    let mut lines = Cursor::new(target_str).lines();
    while let Some(Ok(line)) = lines.next() {
        match line {
            _ if line.starts_with(INCLUDE) => {
                let line = line.replace(INCLUDE, "");
                let line = line.trim();
                println!("include: {}", &line[1..line.len() - 1]);
            }
            _ => {
                println!("normal line: {}", line);
            }
        }
    }
}

const INCLUDE: &'static str = "include";
