/// These are the most common definitions for tty ioctl numbers.
/// Most of them do not use the recommended _IOC(), but there is
/// probably some source code out there hardcoding the number,
/// so we might as well use them for all new platforms.
///
/// The architectures that use different values here typically
/// try to be compatible with some Unix variants for the same
/// architecture.

/// 0x54 is just a magic number to make these relatively unique ('T')

pub const TCGETS: i32 = 0x5401;
pub const TCSETS: i32 = 0x5402;
pub const TCSETSW: i32 = 0x5403;
pub const TCSETSF: i32 = 0x5404;
pub const TCGETA: i32 = 0x5405;
pub const TCSETA: i32 = 0x5406;
pub const TCSETAW: i32 = 0x5407;
pub const TCSETAF: i32 = 0x5408;
pub const TCSBRK: i32 = 0x5409;
pub const TCXONC: i32 = 0x540A;
pub const TCFLSH: i32 = 0x540B;
pub const TIOCEXCL: i32 = 0x540C;
pub const TIOCNXCL: i32 = 0x540D;
pub const TIOCSCTTY: i32 = 0x540E;
pub const TIOCGPGRP: i32 = 0x540F;
pub const TIOCSPGRP: i32 = 0x5410;
pub const TIOCOUTQ: i32 = 0x5411;
pub const TIOCSTI: i32 = 0x5412;
pub const TIOCGWINSZ: i32 = 0x5413;
pub const TIOCSWINSZ: i32 = 0x5414;
pub const TIOCMGET: i32 = 0x5415;
pub const TIOCMBIS: i32 = 0x5416;
pub const TIOCMBIC: i32 = 0x5417;
pub const TIOCMSET: i32 = 0x5418;
pub const TIOCGSOFTCAR: i32 = 0x5419;
pub const TIOCSSOFTCAR: i32 = 0x541A;
pub const FIONREAD: i32 = 0x541B;
pub const TIOCINQ: i32 = FIONREAD;
pub const TIOCLINUX: i32 = 0x541C;
pub const TIOCCONS: i32 = 0x541D;
pub const TIOCGSERIAL: i32 = 0x541E;
pub const TIOCSSERIAL: i32 = 0x541F;
pub const TIOCPKT: i32 = 0x5420;
pub const FIONBIO: i32 = 0x5421;
pub const TIOCNOTTY: i32 = 0x5422;
pub const TIOCSETD: i32 = 0x5423;
pub const TIOCGETD: i32 = 0x5424;
/// Needed for POSIX tcsendbreak()
pub const TCSBRKP: i32 = 0x5425;
/// BSD compatibility
pub const TIOCSBRK: i32 = 0x5427;
/// BSD compatibility
pub const TIOCCBRK: i32 = 0x5428;
/// Return the session ID of FD
pub const TIOCGSID: i32 = 0x5429;
//#define TCGETS2		_IOR('T', 0x2A, struct termios2)
//#define TCSETS2		_IOW('T', 0x2B, struct termios2)
//#define TCSETSW2	_IOW('T', 0x2C, struct termios2)
//#define TCSETSF2	_IOW('T', 0x2D, struct termios2)
pub const TIOCGRS485: i32 = 0x542E;
pub const TIOCSRS485: i32 = 0x542F;
//#define TIOCGPTN	_IOR('T', 0x30, unsigned int) /* Get Pty Number (of pty-mux device) */
//#define TIOCSPTLCK	_IOW('T', 0x31, int)  /* Lock/unlock Pty */
//#define TIOCGDEV	_IOR('T', 0x32, unsigned int) /* Get primary device node of /dev/console */
/// SYS5 TCGETX compatibility
pub const TCGETX: i32 = 0x5432;
pub const TCSETX: i32 = 0x5433;
pub const TCSETXF: i32 = 0x5434;
pub const TCSETXW: i32 = 0x5435;
//#define TIOCSIG		_IOW('T', 0x36, int)  /* pty: generate signal */
pub const TIOCVHANGUP: i32 = 0x5437;
//#define TIOCGPKT	_IOR('T', 0x38, int) /* Get packet mode state */
//#define TIOCGPTLCK	_IOR('T', 0x39, int) /* Get Pty lock state */
//#define TIOCGEXCL	_IOR('T', 0x40, int) /* Get exclusive mode state */
//#define TIOCGPTPEER	_IO('T', 0x41) /* Safely open the slave */
//#define TIOCGISO7816	_IOR('T', 0x42, struct serial_iso7816)
//#define TIOCSISO7816	_IOWR('T', 0x43, struct serial_iso7816)

pub const FIONCLEX: i32 = 0x5450;
pub const FIOCLEX: i32 = 0x5451;
pub const FIOASYNC: i32 = 0x5452;
pub const TIOCSERCONFIG: i32 = 0x5453;
pub const TIOCSERGWILD: i32 = 0x5454;
pub const TIOCSERSWILD: i32 = 0x5455;
pub const TIOCGLCKTRMIOS: i32 = 0x5456;
pub const TIOCSLCKTRMIOS: i32 = 0x5457;
/// For debugging only
pub const TIOCSERGSTRUCT: i32 = 0x5458;
/// Get line status register
pub const TIOCSERGETLSR: i32 = 0x5459;
/// Get multiport config
pub const TIOCSERGETMULTI: i32 = 0x545A;
/// Set multiport config
pub const TIOCSERSETMULTI: i32 = 0x545B;

/// wait for a change on serial input line(s)
pub const TIOCMIWAIT: i32 = 0x545C;
/// read serial port inline interrupt counts
pub const TIOCGICOUNT: i32 = 0x545D;

/// Some arches already define FIOQSIZE due to a historical
/// conflict with a Hayes modem-specific ioctl value.
pub const FIOQSIZE: i32 = 0x5460;

/// Used for packet mode
pub const TIOCPKT_DATA: i32 = 0;
pub const TIOCPKT_FLUSHREAD: i32 = 1;
pub const TIOCPKT_FLUSHWRITE: i32 = 2;
pub const TIOCPKT_STOP: i32 = 4;
pub const TIOCPKT_START: i32 = 8;
pub const TIOCPKT_NOSTOP: i32 = 16;
pub const TIOCPKT_DOSTOP: i32 = 32;
pub const TIOCPKT_IOCTL: i32 = 64;

/// Transmitter physically empty
pub const TIOCSER_TEMT: i32 = 0x01;