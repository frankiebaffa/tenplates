// The core library for TenPlates compilation.
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

mod context;
mod error;
mod input;
mod macros;
mod output;
mod parser;

pub use error::{ InternalResult, InternalError, };

use {
    crate::{
        input::TryIntoInput,
        parser::TemplateParser,
    },
    std::{
        fmt::Debug,
        io::{ Read, stdout, Write, },
        path::Path,
    },
};

/// The tenplates compiler.
pub struct Tenplates;
impl Tenplates {
    /// Compile an input template and to a given output.
    ///
    /// # Arguments
    ///
    /// * `input` - The [readable](Read) template.
    /// * `output` - The [writable](Write) output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tenplates_core::Tenplates;
    /// 
    /// let input = "<% set i %>0<% /set %>{{ i }}";
    /// let mut output = Vec::<u8>::new();
    /// Tenplates::compile(input, &mut output).unwrap();
    /// let output_str = String::from_utf8(output).unwrap();
    ///assert_eq!("0", output_str);
    /// ```
    ///
    pub fn compile<R, I, W>(input: I, output: W) -> InternalResult<()>
    where
        R: Read + Debug,
        I: TryIntoInput<R>,
        W: Write + Debug,
    {
        let input = input.try_into_input()?;
        let mut parser = TemplateParser::new(
            context::Context::default(),
            input,
            output,
        )?;

        parser.parse()?;

        Ok(())
    }

    /// Compile a template file to a given output.
    ///
    /// # Arguments
    ///
    /// * `path` - The [path](Path) to the file.
    /// * `output` - The [writable](Write) output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tenplates_core::Tenplates;
    ///
    /// let path = "./resources/parse_file_1/page.tenplate";
    /// let mut output = Vec::<u8>::new();
    /// Tenplates::compile_file(path, &mut output).unwrap();
    /// let output_str = String::from_utf8(output).unwrap();
    /// assert_eq!("The number: 4", output_str);
    /// ```
    ///
    pub fn compile_file<P, W>(path: P, output: W) -> InternalResult<()>
    where
        P: AsRef<Path>,
        W: Write + Debug,
    {
        Self::compile(path.as_ref(), output)
    }

    /// Compile a template file to [stdout](std::io::Stdout).
    ///
    /// # Arguments
    ///
    /// * `path` - The [path](Path) to the template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tenplates_core::Tenplates;
    ///
    /// Tenplates::compile_file_to_stdout("./resources/parse_file_1/page.tenplate")
    ///     .unwrap();
    /// ```
    ///
    pub fn compile_file_to_stdout<P>(path: P) -> InternalResult<()>
    where
        P: AsRef<Path>,
    {
        Self::compile_file(path, stdout())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_file_1() {
        let mut output = Vec::<u8>::new();
        crate::Tenplates::compile_file("./resources/parse_file_1/page.tenplate", &mut output).unwrap();
        let output = String::from_utf8(output).unwrap();
        assert_eq!("The number: 4", output);
    }
}
