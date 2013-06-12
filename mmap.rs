#[ link(name = "mmap", vers="1.0") ];
#[ crate_type = "lib" ];

pub mod raw {
    pub extern {
        unsafe fn mmap(addr : *libc::c_char, length : libc::size_t, 
                       prot : libc::c_int,   flags  : libc::c_int, 
                       fd   : libc::c_int,   offset : libc::off_t) -> *u8;
        unsafe fn munmap(addr : *u8, length : libc::size_t) -> libc::c_int;
    }

    /* From /usr/include/asm-generic/mman-common.h on Linux */

    /* prot values */
    pub static PROT_NONE   : libc::c_int = 0x0;
    pub static PROT_READ   : libc::c_int = 0x1;
    pub static PROT_WRITE  : libc::c_int = 0x2;
    pub static PROT_EXEC   : libc::c_int = 0x4;
    // ...

    /* flags */
    pub static MAP_SHARED  : libc::c_int = 0x1;
    pub static MAP_PRIVATE : libc::c_int = 0x2;
    // ...
}

struct FileDescriptor(libc::c_int);

impl Drop for FileDescriptor {
    fn finalize(&self) { unsafe { libc::close(**self); } }
}

unsafe fn open(filename : &str) -> FileDescriptor {
    let fd = do str::as_c_str(filename) |cs| { libc::open(cs, libc::O_RDONLY as libc::c_int, 0) };
    if fd < 0 { fail!(fmt!("failure in open(%s): %s", filename, os::last_os_error())); }
    return FileDescriptor(fd);
}

unsafe fn fstat(fd : libc::c_int) -> libc::stat {
    /* target_arch = "x86_64", target_os = "linux" or target_os = "android" */
    let mut s = libc::stat {
        st_dev        : 0,
        st_ino        : 0,
        st_mode       : 0,
        st_nlink      : 0,
        st_uid        : 0,
        st_gid        : 0,
        st_rdev       : 0,
        st_size       : 0,
        st_blksize    : 0,
        st_blocks     : 0,
        st_atime      : 0,
        st_atime_nsec : 0,
        st_mtime      : 0,
        st_mtime_nsec : 0,
        st_ctime      : 0,
        st_ctime_nsec : 0,
        __pad0        : 0,
        __unused      : [0,0,0]
    };
    if libc::fstat(fd, &mut s) < 0 { fail!(fmt!("failure in fstat(): %s", os::last_os_error())); }
    return s;
}

struct MappedRegion {
    reg : *u8,
    siz : libc::size_t
}

impl Drop for MappedRegion {
    fn finalize(&self) {
        unsafe {
            if raw::munmap(self.reg, self.siz) < 0 {
                fail!(fmt!("munmap(): %s", os::last_os_error()));
            }
        }
    }
}

unsafe fn mmap(fd : libc::c_int, size : libc::size_t) -> MappedRegion {
    let buf = raw::mmap(0 as *libc::c_char, size, raw::PROT_READ, raw::MAP_SHARED, fd, 0);
    if buf == -1 as *u8 { fail!(fmt!("mmap(): %s", os::last_os_error())); }
    MappedRegion { reg : buf, siz : size }
 }

pub fn with_mmap_file_contents<U>(filename : &str, f : &fn(v : &[u8]) -> U) -> U {
    unsafe {
        let fd = open(filename);
        let st = fstat(*fd);
        let buf = mmap(*fd, st.st_size as libc::size_t);
        return vec::raw::buf_as_slice(buf.reg, buf.siz as uint, f);
    }
}
