use std::io::{self, Write};

pub fn console_write(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(s.as_bytes())?;
    handle.flush()?;
    Ok(())
}
