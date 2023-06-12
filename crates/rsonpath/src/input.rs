use crate::args::InputArg;
use eyre::Result;
use std::{
    fs,
    io::{self, Read},
    os,
};

const MAX_EAGER_LEN: u64 = 1 << 20;

pub enum FileOrStdin {
    File(fs::File),
    Stdin(io::Stdin),
}

pub enum ResolvedInputKind {
    Mmap,
    Owned,
    Buffered,
}

impl os::fd::AsRawFd for FileOrStdin {
    fn as_raw_fd(&self) -> os::fd::RawFd {
        match self {
            FileOrStdin::File(f) => f.as_raw_fd(),
            FileOrStdin::Stdin(s) => s.as_raw_fd(),
        }
    }
}

impl Read for FileOrStdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            FileOrStdin::File(f) => f.read(buf),
            FileOrStdin::Stdin(s) => s.read(buf),
        }
    }
}

pub fn decide_input_strategy(
    source: &FileOrStdin,
    force_input: Option<&InputArg>,
) -> Result<(ResolvedInputKind, Option<ResolvedInputKind>)> {
    if let Some(force) = force_input {
        // If input is forced we make no choices, use exactly what the user asked for.
        let forced = match force {
            InputArg::Mmap => ResolvedInputKind::Mmap,
            InputArg::Eager => ResolvedInputKind::Owned,
            InputArg::Buffered => ResolvedInputKind::Buffered,
        };
        Ok((forced, None))
    } else {
        match source {
            FileOrStdin::File(file) => match file.metadata() {
                Ok(meta) if meta.len() <= MAX_EAGER_LEN => Ok((ResolvedInputKind::Owned, None)),
                _ if is_mmap_available() => Ok((ResolvedInputKind::Mmap, Some(ResolvedInputKind::Buffered))),
                _ => Ok((ResolvedInputKind::Buffered, None)),
            },
            // There is not much that can be done for stdin. A memory map over stdin does not work unless
            // the input is a file (e.g. when ran as `rq query < file_path` from bash), but the user should
            // pass files directly as an argument, at which point they are handled by the path above.
            // Since there is no way to determine the length of content in stdin without reading it,
            // and an expected use case of rq is to chain multiple queries together (which might produce
            // large piped outputs), we pessimistically assume stdin is large and buffer it.
            FileOrStdin::Stdin(_) => Ok((ResolvedInputKind::Buffered, None)),
        }
    }
}

fn is_mmap_available() -> bool {
    // The memmap2 crate itself only supports unix and windows.
    cfg!(unix) || cfg!(windows)
}
