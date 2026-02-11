use crate::args::InputArg;
use eyre::Result;
use std::{
    fs,
    io::{self, Read},
    os,
};

const MAX_EAGER_LEN: u64 = 1 << 20;

pub(super) enum JsonSource<S> {
    File(fs::File),
    Stdin(io::Stdin),
    Inline(S),
}

pub(super) enum JsonSourceRead<'a> {
    File(&'a mut fs::File),
    Stdin(&'a mut io::Stdin),
}

pub(super) enum ResolvedInputKind {
    Mmap,
    Owned,
    Buffered,
}

impl<S> JsonSource<S> {
    #[cfg(unix)]
    pub(crate) fn try_as_raw_desc(&self) -> Option<os::fd::RawFd> {
        use std::os::fd::AsRawFd as _;

        match self {
            Self::File(f) => Some(f.as_raw_fd()),
            Self::Stdin(s) => Some(s.as_raw_fd()),
            Self::Inline(_) => None,
        }
    }

    #[cfg(windows)]
    pub(crate) fn try_as_raw_desc(&self) -> Option<os::windows::io::RawHandle> {
        use os::windows::io::AsRawHandle;

        match self {
            Self::File(f) => Some(f.as_raw_handle()),
            Self::Stdin(s) => Some(s.as_raw_handle()),
            Self::Inline(_) => None,
        }
    }

    pub(crate) fn try_as_read(&mut self) -> Option<JsonSourceRead<'_>> {
        match self {
            Self::File(f) => Some(JsonSourceRead::File(f)),
            Self::Stdin(s) => Some(JsonSourceRead::Stdin(s)),
            Self::Inline(_) => None,
        }
    }
}

impl Read for JsonSourceRead<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::File(f) => f.read(buf),
            Self::Stdin(s) => s.read(buf),
        }
    }
}

pub(super) fn decide_input_strategy<S>(
    source: &JsonSource<S>,
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
            JsonSource::File(file) => match file.metadata() {
                Ok(meta) if meta.len() <= MAX_EAGER_LEN => Ok((ResolvedInputKind::Owned, None)),
                _ if is_mmap_available() => Ok((ResolvedInputKind::Mmap, Some(ResolvedInputKind::Buffered))),
                _ => Ok((ResolvedInputKind::Buffered, None)),
            },
            JsonSource::Inline(_) => Ok((ResolvedInputKind::Owned, None)),
            // There is not much that can be done for stdin. A memory map over stdin does not work unless
            // the input is a file (e.g. when ran as `rq query < file_path` from bash), but the user should
            // pass files directly as an argument, at which point they are handled by the path above.
            // Since there is no way to determine the length of content in stdin without reading it,
            // and an expected use case of rq is to chain multiple queries together (which might produce
            // large piped outputs), we pessimistically assume stdin is large and buffer it.
            JsonSource::Stdin(_) => Ok((ResolvedInputKind::Buffered, None)),
        }
    }
}

fn is_mmap_available() -> bool {
    // The memmap2 crate itself only supports unix and windows.
    cfg!(unix) || cfg!(windows)
}
