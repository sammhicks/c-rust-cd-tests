use std::ffi::CStr;
use std::os::unix::io::AsRawFd;

#[repr(C)]
#[derive(Debug, Default)]
struct CdToc {
    cdth_trk0: u8, /* start track */
    cdth_trk1: u8, /* end track */
}

#[repr(u8)]
#[derive(Debug)]
enum LbaMsf {
    Lba = 0x01,
    Msf = 0x02,
}

impl Default for LbaMsf {
    fn default() -> Self {
        Self::Lba
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct cdrom_msf0 {
    minute: u8,
    second: u8,
    frame: u8,
}

#[repr(C)]
union cdrom_addr {
    msf: cdrom_msf0,
    lba: libc::c_int,
}

impl Default for cdrom_addr {
    fn default() -> Self {
        Self { lba: 0 }
    }
}

#[repr(C)]
#[derive(Default)]
struct AdrCtrl(u8);

impl AdrCtrl {
    fn adr(&self) -> u8 {
        #[cfg(target_endian = "big")]
        {
            self.0 >> 4
        }
        #[cfg(target_endian = "little")]
        {
            self.0 & 0b1111
        }
    }

    fn ctrl(&self) -> u8 {
        #[cfg(target_endian = "big")]
        {
            self.0 & 0b1111
        }
        #[cfg(target_endian = "little")]
        {
            self.0 >> 4
        }
    }

    fn debug_fields<'a, 'b: 'a>(&self, s: &mut std::fmt::DebugStruct<'a, 'b>) {
        let adr = self.adr();
        let ctrl = self.ctrl();
        s.field("cdte_adr", &adr).field("cdte_ctrl", &ctrl);
    }
}

impl std::fmt::Debug for AdrCtrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("AdrCtrl");
        self.debug_fields(&mut debug_struct);
        debug_struct.finish()
    }
}

#[repr(C)]
#[derive(Default)]
struct CdTocEntry {
    cdte_track: u8,
    cdte_adr_ctrl: AdrCtrl,
    cdte_format: LbaMsf,
    cdte_addr: cdrom_addr,
    cdte_datamode: u8,
}

impl std::fmt::Debug for CdTocEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("CdTocEntry");

        debug_struct.field("cdte_track", &self.cdte_track);
        self.cdte_adr_ctrl.debug_fields(&mut debug_struct);
        debug_struct.field("cdte_format", &self.cdte_format);

        match self.cdte_format {
            LbaMsf::Lba => debug_struct.field("cdte_addr", unsafe { &self.cdte_addr.lba }),
            LbaMsf::Msf => debug_struct.field("cdte_addr", unsafe { &self.cdte_addr.msf }),
        };

        debug_struct.field("cdte_datamode", &self.cdte_datamode);

        debug_struct.finish()
    }
}

#[repr(u32)]
enum IoCtlRequest {
    CDROMREADTOCHDR = 0x5305,
    CDROMREADTOCENTRY = 0x5306,
}

unsafe fn check_errno(result: libc::c_int) {
    if result < 0 {
        let message = CStr::from_ptr(libc::strerror(*libc::__errno_location()));
        panic!("Failed to read TOC: {:?}", message);
    }
}

fn main() {
    unsafe {
        let device = std::fs::File::open("/dev/cdrom").unwrap();

        let fd = device.as_raw_fd();

        let mut toc = CdToc::default();

        check_errno(libc::ioctl(
            fd,
            IoCtlRequest::CDROMREADTOCHDR as u32,
            (&mut toc) as *mut CdToc,
        ));

        println!("{:?}", toc);

        for i in toc.cdth_trk0..=toc.cdth_trk1 {
            let mut entry = CdTocEntry {
                cdte_track: i,
                cdte_format: LbaMsf::Lba,
                ..CdTocEntry::default()
            };

            check_errno(libc::ioctl(
                fd,
                IoCtlRequest::CDROMREADTOCENTRY as u32,
                (&mut entry) as *mut CdTocEntry,
            ));

            println!("{:?}", entry);
        }
    }
}
