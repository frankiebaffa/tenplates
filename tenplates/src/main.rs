// Compiles TenPlates templates.
// Copyright (C) 2025  Frankie Baffa (frankiebaffa@gmail.com)
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use {
    tenplates_core::Tenplates,
    std::path::PathBuf,
};

fn help() -> ! {
	println!("{}", include_str!("../resources/help.txt"));
	std::process::exit(0)
}

fn version() -> ! {
    println!("tenplates: v{}", env!("CARGO_PKG_VERSION"));
	std::process::exit(0)
}

fn main() {
    let mut path: Option<PathBuf> = None;

    let mut args = std::env::args();
    args.next(); // burn program name

    for full_arg in args {
        if let Some(long_arg) = full_arg.strip_prefix("--") {
            match long_arg {
                "help" => help(),
                "version" => version(),
                long_arg => {
                    eprintln!("tenplates: unknown argument '--{long_arg}'");
                    std::process::exit(1);
                },
            }
        }
        else if full_arg.starts_with('-') && full_arg.len() > 1 {
            let mut short_args = full_arg[1..].chars();
            match short_args.next() {
                Some('h') => help(),
                Some('v') => version(),
                Some(short_arg) => {
                    eprintln!("tenplates: unknown arguemnt '-{short_arg}'");
                    std::process::exit(1);
                },
                _ => panic!("HOW WAS THE ARG NONE!?"),
            }
        }
        else {
            if path.is_some() {
                eprintln!("tenplates: cannot include more than one path");
                std::process::exit(1);
            }

            path = Some(PathBuf::from(full_arg));
        }
    }

    if path.is_none() {
        eprintln!("tenplates: path must be defined");
        std::process::exit(1);
    }

    if let Err(e) = Tenplates::compile_file_to_stdout(path.unwrap()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
