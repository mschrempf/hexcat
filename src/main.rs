use std::fmt::Write as FmtWrite;
use std::io::Read;
use std::io::Write;

const GROUP_SIZE: usize = 4;
const CHUNK_SIZE: usize = GROUP_SIZE * 4;

struct Chunk<'a> {
    bytes: &'a [u8],
    chunk_size: usize,
}

impl<'a> Chunk<'a> {
    fn new(bytes: &'a [u8], chunk_size: usize) -> Self {
        Self { bytes, chunk_size }
    }
}

impl std::fmt::Display for Chunk<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.chunk_size {
            if i < self.bytes.len() {
                let byte = self.bytes[i];
                f.write_fmt(format_args!("{byte:02x}"))?;
            } else {
                f.write_str("  ")?;
            }
            if i > 1 && (i + 1) % GROUP_SIZE == 0 {
                f.write_char('|')?;
            } else {
                f.write_char(' ')?;
            }
        }
        f.write_char(' ')?;

        for &byte in self.bytes {
            if byte.is_ascii_graphic() || byte == b' ' {
                f.write_char(byte.into())?;
            } else {
                f.write_char('.')?;
            }
        }

        Ok(())
    }
}

fn write_hex(mut reader: impl Read, mut writer: impl Write) -> std::io::Result<()> {
    let mut buf = [0u8; CHUNK_SIZE];
    let mut offset = 0;
    loop {
        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let chunk = Chunk::new(&buf[..bytes_read], CHUNK_SIZE);
        write!(writer, "{offset:08x}: ")?;
        writeln!(writer, "{chunk}")?;
        offset += bytes_read;
    }
}

fn main() {
    let mut args = std::env::args_os();

    if let Some(path) = args.nth(1) {
        match std::fs::File::open(&path) {
            Ok(file) => {
                // hexcat the contents of the given file
                let _ = write_hex(file, std::io::stdout());
            }
            Err(error) => eprintln!("{path:?}: {error}"),
        }
    } else {
        // if no path was provided we read from stdin instead
        let _ = write_hex(std::io::stdin(), std::io::stdout());
    }
}
