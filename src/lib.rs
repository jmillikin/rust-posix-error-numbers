#![no_std]
#![feature(doc_cfg)]
#![doc(cfg(unix))]

use core::{fmt, num, ops};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ErrorCode(num::NonZeroU16);

impl ErrorCode {
	pub fn new(n: num::NonZeroU16) -> ErrorCode { ErrorCode(n) }
}

impl From<ErrorCode> for u16 {
	fn from(err: ErrorCode) -> u16 { err.0.get() }
}

impl From<ErrorCode> for num::NonZeroU16 {
	fn from(err: ErrorCode) -> num::NonZeroU16 { err.0 }
}

impl From<ErrorCode> for u32 {
	fn from(err: ErrorCode) -> u32 { err.0.get().into() }
}

impl From<ErrorCode> for num::NonZeroU32 {
	fn from(err: ErrorCode) -> num::NonZeroU32 { err.0.into() }
}

impl From<ErrorCode> for i32 {
	fn from(err: ErrorCode) -> i32 { err.0.get().into() }
}

impl From<ErrorCode> for num::NonZeroI32 {
	fn from(err: ErrorCode) -> num::NonZeroI32 { err.0.into() }
}

impl From<ErrorCode> for u64 {
	fn from(err: ErrorCode) -> u64 { err.0.get().into() }
}

impl From<ErrorCode> for num::NonZeroU64 {
	fn from(err: ErrorCode) -> num::NonZeroU64 { err.0.into() }
}

impl From<ErrorCode> for i64 {
	fn from(err: ErrorCode) -> i64 { err.0.get().into() }
}

impl From<ErrorCode> for num::NonZeroI64 {
	fn from(err: ErrorCode) -> num::NonZeroI64 { err.0.into() }
}

impl fmt::Debug for ErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(fmt)
	}
}

impl fmt::Display for ErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(fmt)
	}
}

impl fmt::Binary for ErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
			self.0.fmt(fmt)
	}
}

impl fmt::LowerHex for ErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
			self.0.fmt(fmt)
	}
}

impl fmt::UpperHex for ErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
			self.0.fmt(fmt)
	}
}

macro_rules! impl_partial_eq {
	($t:ty) => {
		impl PartialEq<$t> for ErrorCode {
			fn eq(&self, x: &$t) -> bool {
				<$t>::from(self.0.get()) == *x
			}
		}

		impl PartialEq<ErrorCode> for $t {
			fn eq(&self, x: &ErrorCode) -> bool {
				<$t>::from(x.0.get()) == *self
			}
		}
	};
}

impl_partial_eq!(i32);
impl_partial_eq!(i64);
impl_partial_eq!(u16);
impl_partial_eq!(u32);
impl_partial_eq!(u64);
impl_partial_eq!(usize);

impl PartialEq<i16> for ErrorCode {
	fn eq(&self, x: &i16) -> bool {
		if *x <= 0 {
			return false;
		}
		self.0.get() == *x as u16
	}
}

impl PartialEq<ErrorCode> for i16 {
	fn eq(&self, x: &ErrorCode) -> bool {
		if *self <= 0 {
			return false;
		}
		x.0.get() == *self as u16
	}
}

impl PartialEq<isize> for ErrorCode {
	fn eq(&self, x: &isize) -> bool {
		if *x <= 0 {
			return false;
		}
		usize::from(self.0.get()) == *x as usize
	}
}

impl PartialEq<ErrorCode> for isize {
	fn eq(&self, x: &ErrorCode) -> bool {
		if *self <= 0 {
			return false;
		}
		usize::from(x.0.get()) == *self as usize
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NegErrorCode(num::NonZeroU16);

impl ops::Neg for ErrorCode {
	type Output = NegErrorCode;
	fn neg(self) -> NegErrorCode { NegErrorCode(self.0) }
}

impl ops::Neg for NegErrorCode {
	type Output = ErrorCode;
	fn neg(self) -> ErrorCode { ErrorCode(self.0) }
}

impl fmt::Debug for NegErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		(-i32::from(self.0.get())).fmt(fmt)
	}
}

impl fmt::Display for NegErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		(-i32::from(self.0.get())).fmt(fmt)
	}
}

impl fmt::Binary for NegErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		(-i32::from(self.0.get())).fmt(fmt)
	}
}

impl fmt::LowerHex for NegErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		(-i32::from(self.0.get())).fmt(fmt)
	}
}

impl fmt::UpperHex for NegErrorCode {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		(-i32::from(self.0.get())).fmt(fmt)
	}
}

impl PartialEq<u16> for NegErrorCode {
	fn eq(&self, _: &u16) -> bool { false }
}

impl PartialEq<NegErrorCode> for u16 {
	fn eq(&self, _: &NegErrorCode) -> bool { false }
}

impl PartialEq<u32> for NegErrorCode {
	fn eq(&self, _: &u32) -> bool { false }
}

impl PartialEq<NegErrorCode> for u32 {
	fn eq(&self, _: &NegErrorCode) -> bool { false }
}

impl PartialEq<u64> for NegErrorCode {
	fn eq(&self, _: &u64) -> bool { false }
}

impl PartialEq<NegErrorCode> for u64 {
	fn eq(&self, _: &NegErrorCode) -> bool { false }
}

impl PartialEq<i16> for NegErrorCode {
	fn eq(&self, x: &i16) -> bool {
		if *x >= 0 {
			return false;
		}
		i32::from(self.0.get()) == -i32::from(*x)
	}
}

impl PartialEq<NegErrorCode> for i16 {
	fn eq(&self, x: &NegErrorCode) -> bool {
		if *self >= 0 {
			return false;
		}
		i32::from(x.0.get()) == -i32::from(*self)
	}
}

impl PartialEq<i32> for NegErrorCode {
	fn eq(&self, x: &i32) -> bool {
		if *x >= 0 {
			return false;
		}
		i32::from(self.0.get()) == -*x
	}
}

impl PartialEq<NegErrorCode> for i32 {
	fn eq(&self, x: &NegErrorCode) -> bool {
		if *self >= 0 {
			return false;
		}
		i32::from(x.0.get()) == -*self
	}
}

impl PartialEq<i64> for NegErrorCode {
	fn eq(&self, x: &i64) -> bool {
		if *x >= 0 {
			return false;
		}
		i64::from(self.0.get()) == -*x
	}
}

impl PartialEq<NegErrorCode> for i64 {
	fn eq(&self, x: &NegErrorCode) -> bool {
		if *self >= 0 {
			return false;
		}
		i64::from(x.0.get()) == -*self
	}
}

impl ErrorCode {
	pub const E2BIG: ErrorCode = target::E2BIG;
	pub const EACCES: ErrorCode = target::EACCES;
	pub const EADDRINUSE: ErrorCode = target::EADDRINUSE;
	pub const EADDRNOTAVAIL: ErrorCode = target::EADDRNOTAVAIL;
	pub const EAFNOSUPPORT: ErrorCode = target::EAFNOSUPPORT;
	pub const EAGAIN: ErrorCode = target::EAGAIN;
	pub const EALREADY: ErrorCode = target::EALREADY;
	pub const EBADF: ErrorCode = target::EBADF;
	pub const EBADMSG: ErrorCode = target::EBADMSG;
	pub const EBUSY: ErrorCode = target::EBUSY;
	pub const ECANCELED: ErrorCode = target::ECANCELED;
	pub const ECHILD: ErrorCode = target::ECHILD;
	pub const ECONNABORTED: ErrorCode = target::ECONNABORTED;
	pub const ECONNREFUSED: ErrorCode = target::ECONNREFUSED;
	pub const ECONNRESET: ErrorCode = target::ECONNRESET;
	pub const EDEADLK: ErrorCode = target::EDEADLK;
	pub const EDESTADDRREQ: ErrorCode = target::EDESTADDRREQ;
	pub const EDOM: ErrorCode = target::EDOM;
	pub const EDQUOT: ErrorCode = target::EDQUOT;
	pub const EEXIST: ErrorCode = target::EEXIST;
	pub const EFAULT: ErrorCode = target::EFAULT;
	pub const EFBIG: ErrorCode = target::EFBIG;
	pub const EHOSTUNREACH: ErrorCode = target::EHOSTUNREACH;
	pub const EIDRM: ErrorCode = target::EIDRM;
	pub const EILSEQ: ErrorCode = target::EILSEQ;
	pub const EINPROGRESS: ErrorCode = target::EINPROGRESS;
	pub const EINTR: ErrorCode = target::EINTR;
	pub const EINVAL: ErrorCode = target::EINVAL;
	pub const EIO: ErrorCode = target::EIO;
	pub const EISCONN: ErrorCode = target::EISCONN;
	pub const EISDIR: ErrorCode = target::EISDIR;
	pub const ELOOP: ErrorCode = target::ELOOP;
	pub const EMFILE: ErrorCode = target::EMFILE;
	pub const EMLINK: ErrorCode = target::EMLINK;
	pub const EMSGSIZE: ErrorCode = target::EMSGSIZE;
	pub const EMULTIHOP: ErrorCode = target::EMULTIHOP;
	pub const ENAMETOOLONG: ErrorCode = target::ENAMETOOLONG;
	pub const ENETDOWN: ErrorCode = target::ENETDOWN;
	pub const ENETRESET: ErrorCode = target::ENETRESET;
	pub const ENETUNREACH: ErrorCode = target::ENETUNREACH;
	pub const ENFILE: ErrorCode = target::ENFILE;
	pub const ENOBUFS: ErrorCode = target::ENOBUFS;
	pub const ENODEV: ErrorCode = target::ENODEV;
	pub const ENOENT: ErrorCode = target::ENOENT;
	pub const ENOEXEC: ErrorCode = target::ENOEXEC;
	pub const ENOLCK: ErrorCode = target::ENOLCK;
	pub const ENOLINK: ErrorCode = target::ENOLINK;
	pub const ENOMEM: ErrorCode = target::ENOMEM;
	pub const ENOMSG: ErrorCode = target::ENOMSG;
	pub const ENOPROTOOPT: ErrorCode = target::ENOPROTOOPT;
	pub const ENOSPC: ErrorCode = target::ENOSPC;
	pub const ENOSYS: ErrorCode = target::ENOSYS;
	pub const ENOTCONN: ErrorCode = target::ENOTCONN;
	pub const ENOTDIR: ErrorCode = target::ENOTDIR;
	pub const ENOTEMPTY: ErrorCode = target::ENOTEMPTY;
	pub const ENOTRECOVERABLE: ErrorCode = target::ENOTRECOVERABLE;
	pub const ENOTSOCK: ErrorCode = target::ENOTSOCK;
	pub const ENOTSUP: ErrorCode = target::ENOTSUP;
	pub const ENOTTY: ErrorCode = target::ENOTTY;
	pub const ENXIO: ErrorCode = target::ENXIO;
	pub const EOPNOTSUPP: ErrorCode = target::EOPNOTSUPP;
	pub const EOVERFLOW: ErrorCode = target::EOVERFLOW;
	pub const EOWNERDEAD: ErrorCode = target::EOWNERDEAD;
	pub const EPERM: ErrorCode = target::EPERM;
	pub const EPIPE: ErrorCode = target::EPIPE;
	pub const EPROTO: ErrorCode = target::EPROTO;
	pub const EPROTONOSUPPORT: ErrorCode = target::EPROTONOSUPPORT;
	pub const EPROTOTYPE: ErrorCode = target::EPROTOTYPE;
	pub const ERANGE: ErrorCode = target::ERANGE;
	pub const EROFS: ErrorCode = target::EROFS;
	pub const ESPIPE: ErrorCode = target::ESPIPE;
	pub const ESRCH: ErrorCode = target::ESRCH;
	pub const ESTALE: ErrorCode = target::ESTALE;
	pub const ETIMEDOUT: ErrorCode = target::ETIMEDOUT;
	pub const ETXTBSY: ErrorCode = target::ETXTBSY;
	pub const EWOULDBLOCK: ErrorCode = target::EWOULDBLOCK;
	pub const EXDEV: ErrorCode = target::EXDEV;
}

#[doc(cfg(any(
	target_os = "linux",
	target_os = "macos",
)))]
#[cfg(any(
	doc,
	target_os = "linux",
	target_os = "macos",
))]
impl ErrorCode {
	pub const ENODATA: ErrorCode = target::ENODATA;
	pub const ENOSR: ErrorCode = target::ENOSR;
	pub const ENOSTR: ErrorCode = target::ENOSTR;
	pub const ETIME: ErrorCode = target::ETIME;
}

macro_rules! target_error_codes {
	($( $name:ident : $value:literal , )*) => {
		mod target {
			$(
				pub(super) const $name: super::ErrorCode = super::ErrorCode(unsafe{
					core::num::NonZeroU16::new_unchecked($value)
				});
			)*
		}
	}
}

#[cfg(target_os = "freebsd")]
target_error_codes! {
	E2BIG: 7,
	EACCES: 13,
	EADDRINUSE: 48,
	EADDRNOTAVAIL: 49,
	EAFNOSUPPORT: 47,
	EAGAIN: 35,
	EALREADY: 37,
	EBADF: 9,
	EBADMSG: 89,
	EBUSY: 16,
	ECANCELED: 85,
	ECHILD: 10,
	ECONNABORTED: 53,
	ECONNREFUSED: 61,
	ECONNRESET: 54,
	EDEADLK: 11,
	EDESTADDRREQ: 39,
	EDOM: 33,
	EDQUOT: 69,
	EEXIST: 17,
	EFAULT: 14,
	EFBIG: 27,
	EHOSTUNREACH: 65,
	EIDRM: 82,
	EILSEQ: 86,
	EINPROGRESS: 36,
	EINTR: 4,
	EINVAL: 22,
	EIO: 5,
	EISCONN: 56,
	EISDIR: 21,
	ELOOP: 62,
	EMFILE: 24,
	EMLINK: 31,
	EMSGSIZE: 40,
	EMULTIHOP: 90,
	ENAMETOOLONG: 63,
	ENETDOWN: 50,
	ENETRESET: 52,
	ENETUNREACH: 51,
	ENFILE: 23,
	ENOBUFS: 55,
	ENODEV: 19,
	ENOENT: 2,
	ENOEXEC: 8,
	ENOLCK: 77,
	ENOLINK: 91,
	ENOMEM: 12,
	ENOMSG: 83,
	ENOPROTOOPT: 42,
	ENOSPC: 28,
	ENOSYS: 78,
	ENOTCONN: 57,
	ENOTDIR: 20,
	ENOTEMPTY: 66,
	ENOTRECOVERABLE: 95,
	ENOTSOCK: 38,
	ENOTSUP: 45,
	ENOTTY: 25,
	ENXIO: 6,
	EOPNOTSUPP: 45,
	EOVERFLOW: 84,
	EOWNERDEAD: 96,
	EPERM: 1,
	EPIPE: 32,
	EPROTO: 92,
	EPROTONOSUPPORT: 43,
	EPROTOTYPE: 41,
	ERANGE: 34,
	EROFS: 30,
	ESPIPE: 29,
	ESRCH: 3,
	ESTALE: 70,
	ETIMEDOUT: 60,
	ETXTBSY: 26,
	EWOULDBLOCK: 35,
	EXDEV: 18,
}

#[cfg(all(
	target_os = "linux",
	any(
		target_arch = "x86",
		target_arch = "x86_64",
	),
))]
target_error_codes! {
	E2BIG: 7,
	EACCES: 13,
	EADDRINUSE: 98,
	EADDRNOTAVAIL: 99,
	EAFNOSUPPORT: 97,
	EAGAIN: 11,
	EALREADY: 114,
	EBADF: 9,
	EBADMSG: 74,
	EBUSY: 16,
	ECANCELED: 125,
	ECHILD: 10,
	ECONNABORTED: 103,
	ECONNREFUSED: 111,
	ECONNRESET: 104,
	EDEADLK: 35,
	EDESTADDRREQ: 89,
	EDOM: 33,
	EDQUOT: 122,
	EEXIST: 17,
	EFAULT: 14,
	EFBIG: 27,
	EHOSTUNREACH: 113,
	EIDRM: 43,
	EILSEQ: 84,
	EINPROGRESS: 115,
	EINTR: 4,
	EINVAL: 22,
	EIO: 5,
	EISCONN: 106,
	EISDIR: 21,
	ELOOP: 40,
	EMFILE: 24,
	EMLINK: 31,
	EMSGSIZE: 90,
	EMULTIHOP: 72,
	ENAMETOOLONG: 36,
	ENETDOWN: 100,
	ENETRESET: 102,
	ENETUNREACH: 101,
	ENFILE: 23,
	ENOBUFS: 105,
	ENODATA: 61,
	ENODEV: 19,
	ENOENT: 2,
	ENOEXEC: 8,
	ENOLCK: 37,
	ENOLINK: 67,
	ENOMEM: 12,
	ENOMSG: 42,
	ENOPROTOOPT: 92,
	ENOSPC: 28,
	ENOSR: 63,
	ENOSTR: 60,
	ENOSYS: 38,
	ENOTCONN: 107,
	ENOTDIR: 20,
	ENOTEMPTY: 39,
	ENOTRECOVERABLE: 131,
	ENOTSOCK: 88,
	ENOTSUP: 95,
	ENOTTY: 25,
	ENXIO: 6,
	EOPNOTSUPP: 95,
	EOVERFLOW: 75,
	EOWNERDEAD: 130,
	EPERM: 1,
	EPIPE: 32,
	EPROTO: 71,
	EPROTONOSUPPORT: 93,
	EPROTOTYPE: 91,
	ERANGE: 34,
	EROFS: 30,
	ESPIPE: 29,
	ESRCH: 3,
	ESTALE: 116,
	ETIME: 62,
	ETIMEDOUT: 110,
	ETXTBSY: 26,
	EWOULDBLOCK: 11,
	EXDEV: 18,
}

#[cfg(all(
	target_os = "linux",
	target_arch = "mips",
))]
target_error_codes! {
	E2BIG: 7,
	EACCES: 13,
	EADDRINUSE: 125,
	EADDRNOTAVAIL: 126,
	EAFNOSUPPORT: 124,
	EAGAIN: 11,
	EALREADY: 149,
	EBADF: 9,
	EBADMSG: 77,
	EBUSY: 16,
	ECANCELED: 158,
	ECHILD: 10,
	ECONNABORTED: 130,
	ECONNREFUSED: 146,
	ECONNRESET: 131,
	EDEADLK: 45,
	EDESTADDRREQ: 96,
	EDOM: 33,
	EDQUOT: 1133,
	EEXIST: 17,
	EFAULT: 14,
	EFBIG: 27,
	EHOSTUNREACH: 148,
	EIDRM: 36,
	EILSEQ: 88,
	EINPROGRESS: 150,
	EINTR: 4,
	EINVAL: 22,
	EIO: 5,
	EISCONN: 133,
	EISDIR: 21,
	ELOOP: 90,
	EMFILE: 24,
	EMLINK: 31,
	EMSGSIZE: 97,
	EMULTIHOP: 74,
	ENAMETOOLONG: 78,
	ENETDOWN: 127,
	ENETRESET: 129,
	ENETUNREACH: 128,
	ENFILE: 23,
	ENOBUFS: 132,
	ENODATA: 61,
	ENODEV: 19,
	ENOENT: 2,
	ENOEXEC: 8,
	ENOLCK: 46,
	ENOLINK: 67,
	ENOMEM: 12,
	ENOMSG: 35,
	ENOPROTOOPT: 99,
	ENOSPC: 28,
	ENOSR: 63,
	ENOSTR: 60,
	ENOSYS: 89,
	ENOTCONN: 134,
	ENOTDIR: 20,
	ENOTEMPTY: 93,
	ENOTRECOVERABLE: 166,
	ENOTSOCK: 95,
	ENOTSUP: 122,
	ENOTTY: 25,
	ENXIO: 6,
	EOPNOTSUPP: 122,
	EOVERFLOW: 79,
	EOWNERDEAD: 165,
	EPERM: 1,
	EPIPE: 32,
	EPROTO: 71,
	EPROTONOSUPPORT: 120,
	EPROTOTYPE: 98,
	ERANGE: 34,
	EROFS: 30,
	ESPIPE: 29,
	ESRCH: 3,
	ESTALE: 151,
	ETIME: 62,
	ETIMEDOUT: 145,
	ETXTBSY: 26,
	EWOULDBLOCK: 11,
	EXDEV: 18,
}

#[cfg(target_os = "macos")]
target_error_codes! {
	E2BIG: 7,
	EACCES: 13,
	EADDRINUSE: 48,
	EADDRNOTAVAIL: 49,
	EAFNOSUPPORT: 47,
	EAGAIN: 35,
	EALREADY: 37,
	EBADF: 9,
	EBADMSG: 94,
	EBUSY: 16,
	ECANCELED: 89,
	ECHILD: 10,
	ECONNABORTED: 53,
	ECONNREFUSED: 61,
	ECONNRESET: 54,
	EDEADLK: 11,
	EDESTADDRREQ: 39,
	EDOM: 33,
	EDQUOT: 69,
	EEXIST: 17,
	EFAULT: 14,
	EFBIG: 27,
	EHOSTUNREACH: 65,
	EIDRM: 90,
	EILSEQ: 92,
	EINPROGRESS: 36,
	EINTR: 4,
	EINVAL: 22,
	EIO: 5,
	EISCONN: 56,
	EISDIR: 21,
	ELOOP: 62,
	EMFILE: 24,
	EMLINK: 31,
	EMSGSIZE: 40,
	EMULTIHOP: 95,
	ENAMETOOLONG: 63,
	ENETDOWN: 50,
	ENETRESET: 52,
	ENETUNREACH: 51,
	ENFILE: 23,
	ENOBUFS: 55,
	ENODATA: 96,
	ENODEV: 19,
	ENOENT: 2,
	ENOEXEC: 8,
	ENOLCK: 77,
	ENOLINK: 97,
	ENOMEM: 12,
	ENOMSG: 91,
	ENOPROTOOPT: 42,
	ENOSPC: 28,
	ENOSR: 98,
	ENOSTR: 99,
	ENOSYS: 78,
	ENOTCONN: 57,
	ENOTDIR: 20,
	ENOTEMPTY: 66,
	ENOTRECOVERABLE: 104,
	ENOTSOCK: 38,
	ENOTSUP: 45,
	ENOTTY: 25,
	ENXIO: 6,
	EOPNOTSUPP: 102,
	EOVERFLOW: 84,
	EOWNERDEAD: 105,
	EPERM: 1,
	EPIPE: 32,
	EPROTO: 100,
	EPROTONOSUPPORT: 43,
	EPROTOTYPE: 41,
	ERANGE: 34,
	EROFS: 30,
	ESPIPE: 29,
	ESRCH: 3,
	ESTALE: 70,
	ETIME: 101,
	ETIMEDOUT: 60,
	ETXTBSY: 26,
	EWOULDBLOCK: 35,
	EXDEV: 18,
}
