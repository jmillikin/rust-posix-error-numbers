Demo crate for https://github.com/rust-lang/rfcs/pull/2973

Notes:
- names the type `ErrorCode`
- uses the `Neg` + unsigned alternative representation
- defines error codes for macOS, FreeBSD, and some Linux
  architectures (x86 + MIPS)
- doesn't have `PartialEq<isize>` for `Neg` because it's midnight
  and I don't want to stare at bit arithmetic any more right now.
