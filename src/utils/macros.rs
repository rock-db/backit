






#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! failed {
    ($title: expr, $($msg: expr), *) => {
        use ansi_term::{self, Colour};
        let format = format!($($msg), *);
        let enabled = ansi_term::enable_ansi_support();
        eprintln!("{}: {}", Colour::Red.paint("Error"), $title);
        eprintln("{}", format);
        std::process::exit(1);
    };
}


#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! failed {
    ($title: expr, $($msg: expr), *) => {
        use ansi_term::{self, Colour};
        let format = format!($($msg), *);
        eprintln!("{}: {}", Colour::Red.paint("Error"), $title);
        eprintln!("{}", format);
        std::process::exit(1);
    };
}




