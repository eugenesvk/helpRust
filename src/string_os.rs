use std::ffi::{OsStr,OsString};
pub fn join_os_str2(a:&OsStr, b:&OsStr) -> Result<OsString,Box<dyn std::error::Error>> {
  //! Alias for concat_os_str2
  concat_os_str2(a,b)}
pub fn concat_os_str2(a:&OsStr, b:&OsStr) -> Result<OsString,Box<dyn std::error::Error>> {
  //! Concatenate two `OsStr` into an owned `OsString` with 1 allocation and a check that max len is not exceeded
  let a_len = a.len();
  let b_len = b.len();
  let cap 	= usize::MAX - a_len; //println!("{:?}",&cap);
  if b_len < cap {
    let mut ret = OsString::with_capacity(a_len + b_len);	// allocate once
    ret.push(a); ret.push(b);                            	// doesn't allocate
    return Ok(ret);
  } else {return Err(format!("∑ of string lengths > usize ‘{}’",usize::MAX).into())}
}
pub fn join_oss(ss:&[&OsStr]) -> Result<OsString,Box<dyn std::error::Error>> {concat_oss(ss)}
  //! Alias for concat_oss
pub fn concat_oss(ss:&[&OsStr]) -> Result<OsString,Box<dyn std::error::Error>> {
  //! Join multiple `OsStr` into an owned `OsString` with 1 allocation and a check that max len is not exceeded
  let mut len:usize = 0;
  for s in ss {
    let slen	= s.len();
    let cap 	= usize::MAX - len;
    if slen < cap {
      len += slen;
    } else {return Err(format!("∑ of passed string lengths exceeds usize ‘{}’",usize::MAX).into())}  }
  let mut ret = OsString::with_capacity(len);	// allocate once
  for s in ss {ret.push(s);}                 	// doesn't allocate
  Ok(ret)
}
