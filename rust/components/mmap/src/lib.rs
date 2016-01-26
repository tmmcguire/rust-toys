extern crate libc;

macro_rules! unwrap {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => panic!(concat!(stringify!($e), ": unwrap! produced None"))
        }
    }
}

// A file descriptor, open for reading.
struct FileDescriptor(libc::c_int);

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        let FileDescriptor(fd) = *self;
        unsafe {
            libc::close(fd);
        }
    }
}

impl FileDescriptor {
    unsafe fn open(filename: &str) -> Result<FileDescriptor,String> {
        if let Ok(file) = std::ffi::CString::new(filename) {
            let fd = libc::open(file.as_ptr(), libc::O_RDONLY, 0);
            if fd >= 0 {
                Ok( FileDescriptor(fd) )
            } else {
                Err( format!("failure in open({}): {}", filename, std::io::Error::last_os_error()) )
            }
        } else {
            Err( format!("cannot convert filename to CString: {}", filename) )
        }
    }
}

#[test]
fn test_open_success() {
    unsafe {
        match FileDescriptor::open("Cargo.toml") {
            Ok(_) => { },
            Err(e) => { panic!(e); }
        }
    }
}

#[test]
fn test_open_failure() {
    unsafe {
        match FileDescriptor::open("nonexistent") {
            Ok(FileDescriptor(f)) => { panic!("open nonexistent file succeded: {}", f); },
            Err(_) =>  { }
        }
    }
}

// -------------------------------------

impl FileDescriptor {
    unsafe fn get_size(&self) -> Result<libc::size_t,String> {
        let FileDescriptor(fd) = *self;
        let mut stat: libc::stat = std::mem::zeroed();
        if libc::fstat(fd, &mut stat) < 0 {
            Err( format!("failure in fstat(): {}", std::io::Error::last_os_error()) )
        } else {
            Ok( stat.st_size as libc::size_t )
        }
    }

    unsafe fn get_fd(&self) -> &libc::c_int { &self.0 }
}

#[test]
fn test_get_size() {
    use std::os::unix::fs::MetadataExt;
    unsafe {
        if let Ok(m) = std::fs::metadata("Cargo.toml") {
            let res = FileDescriptor::open("Cargo.toml")
                .and_then(|fd| { fd.get_size() })
                .and_then(|sz| { if sz == m.size() as usize { Ok(true) } else { Err(format!("{} != 149", sz)) }});
            if res.is_err() {
                panic!(res.unwrap_err());
            }
        } else {
            panic!("cannot get metadata for Cargo.toml");
        }
    }
}
// =====================================

pub struct MappedRegion {
    _fd: FileDescriptor,        // unused after construction; needed to keep fd open
    ptr: *mut u8,
    sz: libc::size_t,
}

impl Drop for MappedRegion {
    fn drop(&mut self) {
        unsafe {
            if libc::munmap(self.ptr as *mut libc::c_void, self.sz) < 0 {
                panic!("cannot munmap: {}", std::io::Error::last_os_error());
            }
        }
    }
}
// -------------------------------------

impl MappedRegion {

    pub fn mmap(filename: &str) -> Result<MappedRegion,String> {
        unsafe {
            match FileDescriptor::open(filename) {
                Ok(fd) => map(fd),
                Err(e) => Err(e)
            }
        }
    }

    pub fn get_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.sz as usize)
        }
    }

    pub fn get_str<'s>(&'s self) -> Result<&'s str,String> {
        std::str::from_utf8(self.get_slice()).map_err(|e| { format!("{}", e) })
    }
}

// -------------------------------------

unsafe fn map(fd: FileDescriptor) -> Result<MappedRegion,String> {
    match fd.get_size() {
        Ok(sz) => {
            let address = libc::mmap(0 as *mut libc::c_void, sz, libc::PROT_READ, libc::MAP_PRIVATE, *fd.get_fd(), 0);
            if address < 0 as *mut libc::c_void {
                Err( format!("failure in mmap(): {}", std::io::Error::last_os_error()) )
            } else {
                Ok( MappedRegion {
                    _fd: fd,
                    ptr: address as *mut u8,
                    sz: sz,
                })
            }
        }
        Err(e) => { Err(e) }
    }
}
#[test]
fn test_mmap() {
    use std::os::unix::fs::MetadataExt;
    if let Ok(m) = std::fs::metadata("Cargo.toml") {
        match MappedRegion::mmap("Cargo.toml") {
            Ok(mr) => {
                assert_eq!(mr.get_slice().len(), m.size() as usize);
                match mr.get_str() {
                    Ok(s) => {
                        assert_eq!( unwrap!( s.lines().nth(0) ), "[package]");
                    }
                    Err(e) => { panic!(e); }
                }
            }
            Err(e) => { panic!(e); }
        }
    } else {
        panic!("cannot get metadata for Cargo.toml");
    }
}
