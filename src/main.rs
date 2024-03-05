
use std::fmt;


struct ABCDEF<T> {
    wert: T,
    breite: usize,
}


trait ZuStringFormatierer {
    fn zu_string(&self) -> String;
}

impl<T: fmt::Display> ZuStringFormatierer for ABCDEF<T> {
    fn zu_string(&self) -> String {
        let x = self.wert.to_string();
        if x.len() > self.breite {
            format!("*{}", &x[x.len() - self.breite + 1..])
        } else {
            format!("{:width$}", self.wert, width = self.breite)
        }
    }
}

// --------------------------------------------

impl<T: fmt::Display> fmt::Display for ABCDEF<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.zu_string())
    }
}

fn main() {
    let n = 120;
    for j in 73..=n {
        print!("{} ", ABCDEF { wert: j, breite: 2 });
        if j % 6 == 0 {
            println!();
        }
    }
}

