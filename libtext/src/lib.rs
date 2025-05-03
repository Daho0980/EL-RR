use pyo3::prelude::*;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};


#[pyfunction]
fn actual_len(a:&str) -> PyResult<usize> {
    Ok(UnicodeWidthStr::width(a))
}

fn ensure_reset_code(active_codes: &String, current_line: &mut String) {
    if !active_codes.is_empty() && !current_line.ends_with("\x1b[0m") {
        current_line.push_str("\x1b[0m");
    }
}

#[pyfunction]
fn cutter(input: &str, width: usize, maintain: bool) -> Vec<String> {
    let mut result = Vec::new();

    'line_loop: for line in input.split('\n') {
        let mut visible = 0;

        let mut current_line = String::with_capacity(width+16);
        let mut active_codes = String::with_capacity(16);

        let mut chars = line.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch == '\u{1b}' {
                let mut seq = String::new();
                let mut iter = chars.clone();

                seq.push(iter.next().unwrap()); // '\x1b'

                if iter.peek() == Some(&'[') {
                    seq.push(iter.next().unwrap()); // '['

                    while let Some(c) = iter.next() {
                        seq.push(c);
                        if c.is_ascii_alphabetic() { break; }
                    }

                    if seq.ends_with('m') {
                        if  seq == "\x1b[0m" { active_codes.clear(); }
                        else                 { active_codes = seq.clone(); }

                        current_line.push_str(&seq);

                        for _ in 0..seq.chars().count() { chars.next(); }

                        continue;
                    }
                }
            }

            let ch = chars.next().unwrap();
            let w = UnicodeWidthChar::width(ch).unwrap_or(0);

            if (visible+w) > width {
                ensure_reset_code(&active_codes, &mut current_line);

                result.push(std::mem::take(&mut current_line));
                
                if !maintain { continue 'line_loop }

                visible = 0;

                if !active_codes.is_empty() { current_line.push_str(&active_codes); }
            }

            current_line.push(ch);
            visible += w;
        } // while end

        if !current_line.is_empty() {
            ensure_reset_code(&active_codes, &mut current_line);
            result.push(current_line);
        }

        if line.is_empty() { result.push(String::new()); }
    } // for end

    result
}


#[pymodule]
fn libtext(m:&Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(actual_len, m)?)?;
    m.add_function(wrap_pyfunction!(cutter, m)?)?;
    Ok(())
}