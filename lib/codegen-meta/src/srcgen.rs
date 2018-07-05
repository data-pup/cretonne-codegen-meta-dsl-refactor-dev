//! Source code generator.
//!
//! The `srcgen` module contains generic helper routines and classes for
//! generating source code.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path;

use error;

static SHIFTWIDTH: usize = 4;

struct IndentedScope {
    fmt: Formatter,
    after: Option<String>,
}

impl IndentedScope {
    fn _enter(&mut self) {
        self.fmt.indent_push();
    }

    fn _exit(&mut self) {
        self.fmt.indent_pop();
        if let Some(ref s) = self.after {
            self.fmt.line(&s);
        }
    }
}

pub struct Formatter {
    indent: usize,
    lines: Vec<String>
}

impl Formatter {
    /// Source code formatter class.
    ///
    /// - Collect source code to be written to a file.
    /// - Keep track of indentation.
    ///
    /// Indentation example: FIXUP: Convert example to Rust.
    ///
    ///    >>> f = Formatter()
    ///    >>> f.line('Hello line 1')
    ///    >>> f.writelines()
    ///    Hello line 1
    ///    >>> f.indent_push()
    ///    >>> f.comment('Nested comment')
    ///    >>> f.indent_pop()
    ///    >>> f.format('Back {} again', 'home')
    ///    >>> f.writelines()
    ///    Hello line 1
    ///        // Nested comment
    ///    Back home again
    pub fn new() -> Formatter {
        Formatter {
            indent: 0,
            lines: Vec::new(),
        }
    }

    /// Increase current indentation level by one.
    pub fn indent_push(&mut self) {
        // self.indent = format!("{}{:-2$}", self.indent, " ", SHIFTWIDTH);
        self.indent += 1;
    }

    /// Decrease indentation by one level.
    pub fn indent_pop(&mut self) {
        assert!(self.indent > 0, "Already at top level indentation");
        self.indent -= 1;
    }

    /// Get the current whitespace indentation in the form of a String.
    fn get_indent(&self) -> String {
        format!("{}{:-1$}", " ", self.indent * SHIFTWIDTH)
    }

    /// Get a string containing whitespace outdented one level. Used for
    /// lines of code that are inside a single indented block.
    fn get_outdent(&self) -> String {
        let outdent_level = self.indent + 1;
        format!("{}{:-1$}", " ", outdent_level * SHIFTWIDTH)
    }

    /// Add an indented line.
    pub fn line(&mut self, contents: &str) {
        let indented_line = format!("{}{}\n", self.get_indent(), contents);
        self.lines.push(indented_line);
    }

    /// Emit a line outdented one level.
    pub fn outdented_line(&mut self, s: String) {
        let new_line = format!("{}{}", self.get_outdent(), s);
        self.lines.push(new_line);
    }

    /// Write all lines to `out`, or stdout if `out` is `None`.
    pub fn writelines(&self, out: Option<&io::Write>) -> Result<(), io::Error> {
        match out {
            Some(_w) => {
                unimplemented!();
                // FIXUP:
                // self.lines.iter().map(|line| w.write(line))
            },
            None => {
                self.lines.iter().for_each(|line| println!("{}", line));
                Ok(())
            },
        }
    }

    /// Write `self.lines` to a file.
    pub fn update_file(&self, filename: &str, directory: Option<&str>) -> Result<(), error::Error> {
        #[cfg(target_family = "windows")]
        let sep = "\\";
        #[cfg(not(target_family = "windows"))]
        let sep = "/";

        let path_str = if let Some(dir) = directory {
            format!("{}{}{}", dir, sep, filename)
        } else {
            filename.to_string()
        };

        let path = path::Path::new(&path_str);
        let mut f = fs::File::create(path)?;

        for l in self.lines.iter().map(|l| l.as_bytes()) {
            f.write_all(l)?;
        }

        Ok(())
    }

    /// FIXUP: Convert example code into Rust.
    ///
    /// Return a scope object for use with a `with` statement:
    ///
    ///    >>> f = Formatter()
    ///    >>> with f.indented('prefix {', '} suffix'):
    ///    ...     f.line('hello')
    ///    >>> f.writelines()
    ///    prefix {
    ///        hello
    ///    } suffix
    ///
    /// The optional `before` and `after` parameters are surrounding lines
    /// which are *not* indented.
    fn indented(&self, _before: Option<&str>, _after: Option<&str>) -> IndentedScope {
        unimplemented!();
    }

    // TODO: Should this class implement a format trait?

    /// Add one or more lines after stripping common indentation.
    pub fn multi_line(&mut self, s: &str) {
        parse_multiline(s)
            .into_iter()
            .for_each(|l| self.line(&l));
    }

    /// Add a comment line.
    pub fn comment(&mut self, s: &str) {
        let commented_line = format!("// {}", s);
        self.line(&commented_line);
    }

    /// Add a (multi-line) documentation comment.
    pub fn doc_comment(&mut self, contents: &str) {
        parse_multiline(contents)
            .iter()
            .map(|l| format!("/// {}", l))
            .for_each(|s| self.line(s.as_str()));
    }

    /// FIXUP: Convert example code into Rust.
    ///
    /// Add a match expression.
    ///
    /// Example:
    ///
    ///    >>> f = Formatter()
    ///    >>> m = Match('x')
    ///    >>> m.arm('Orange', ['a', 'b'], 'some body')
    ///    >>> m.arm('Yellow', ['a', 'b'], 'some body')
    ///    >>> m.arm('Green', ['a', 'b'], 'different body')
    ///    >>> m.arm('Blue', ['x', 'y'], 'some body')
    ///    >>> f.match(m)
    ///    >>> f.writelines()
    ///    match x {
    ///        Orange { a, b } |
    ///        Yellow { a, b } => {
    ///            some body
    ///        }
    ///        Green { a, b } => {
    ///            different body
    ///        }
    ///        Blue { x, y } => {
    ///            some body
    ///        }
    ///    }
    fn add_match(&mut self, _m: Match) {
        unimplemented!();
    }
}

/// Compute the indentation of s, or None of an empty line.
fn indent(s: &str) -> Option<usize> {
    if s.is_empty() {
        None
    } else {
        let t = s.trim_left();
        Some(s.len() - t.len())
    }
}


/// Given a multi-line string, split it into a sequence of lines after
/// stripping a common indentation, as described in the "trim" function
/// from PEP 257. This is useful for strings defined with doc strings:
///    >>> parse_multiline('\\n    hello\\n    world\\n')
///    ['hello', 'world']
fn parse_multiline(s: &str) -> Vec<String> {
    // Convert tabs into spaces.
    let expanded_tab = format!("{:-1$}", " ", SHIFTWIDTH);
    let lines: Vec<String> = s.lines().map(|l| l.replace("\t", &expanded_tab)).collect();

    // Determine minimum indentation, ignoring the first line.
    let indent = lines.iter()
        .skip(1)
        .map(|l| l.len() - l.trim_left().len())
        .filter(|&i| i > 0)
        .min();

    // Strip off leading blank lines.
    let mut lines_iter = lines.iter().skip_while(|l| l.is_empty());
    let mut trimmed = Vec::with_capacity(lines.len());

    // Remove indentation (first line is special)
    lines_iter
        .next()
        .map(|l| l.trim())
        .map(|l| l.to_string())
        .map(|s| trimmed.push(s));

    // Remove trailing whitespace from other lines.
    if let Some(indent) = indent {
        let mut other_lines = lines_iter
            .map(|l| &l[indent..])
            .map(|l| l.trim_right())
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        trimmed.append(&mut other_lines);
    }

    // Strip off trailing blank lines.
    while let Some(s) = trimmed.pop() {
        if s.is_empty() {
            continue;
        } else {
            trimmed.push(s);
            break;
        }
    }

    trimmed
}

/// Match formatting class.
///
/// Match objects collect all the information needed to emit a Rust `match`
/// expression, automatically deduplicating overlapping identical arms.
///
/// TODO: Convert the example to Rust
///
/// Example:
///
///    >>> m = Match('x')
///    >>> m.arm('Orange', ['a', 'b'], 'some body')
///    >>> m.arm('Yellow', ['a', 'b'], 'some body')
///    >>> m.arm('Green', ['a', 'b'], 'different body')
///    >>> m.arm('Blue', ['x', 'y'], 'some body')
///    >>> assert(len(m.arms) == 3)
///
/// Note that this class is ignorant of Rust types, and considers two fields
/// with the same name to be equivalent.
struct Match<'a> {
    expr: &'a str,
    arms: BTreeMap<(Vec<&'a str>, &'a str), HashSet<&'a str>>,
}

impl<'a> Match<'a> {
    /// Create a new match statemeht on `expr`.
    fn _new(expr: &'a str) -> Match {
        Match {
            expr,
            arms: BTreeMap::new(),
        }
    }

    /// Add an arm to the Match statement.
    fn _arm(&mut self, name: &'a str, fields: Vec<&'a str>, body: &'a str) {
        // let key = (fields, body);
        let match_arm = self.arms.entry((fields, body)).or_insert(HashSet::new());
        match_arm.insert(name);
    }
}

#[cfg(test)]
mod match_tests {
    use super::Match;

    #[test]
    fn adding_arms_works() {
        let mut m = Match::_new("x");
        m._arm("Orange", vec!["a", "b"], "some body");
        m._arm("Yellow", vec!["a", "b"], "some body");
        m._arm("Green", vec!["a", "b"], "different body");
        m._arm("Blue", vec!["x", "y"], "some body");
        assert_eq!(m.arms.len(), 3);
    }
}

#[cfg(test)]
mod parse_multiline_tests {
    use super::parse_multiline;

    #[test]
    fn parse_multiline_works() {
        let input = "\n    hello\n    world\n";
        let expected = vec!["hello", "world"];
        let output = parse_multiline(input);
        assert_eq!(output, expected);
    }
}