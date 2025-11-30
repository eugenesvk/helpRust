
fn main1() {
  if let Err(err) = run() {p!("{}", err); process::exit(1);}
}
fn run() -> Result<(), Box<dyn Error>> {
  let     file_p 	= get_first_arg()?;
  // 1 direct
  // let     file	= File::open(file_p)?;
  // let mut rdr 	= csv::Reader::from_reader(file);
  // 2 shorter
  // let mut rdr	= csv::Reader::from_path(file_p)?;
  // 3 builder
  let mut rdr	= csv::ReaderBuilder::new().has_headers(true).delimiter(b'\t').comment(Some(b'#'))
    .from_path(file_p)?;
  let header = rdr.headers()?.clone();
  if   let Some(col_value_i    )	= header.iter().position(|x| x.to_ascii_lowercase() == col_value_nm    ) {
      p!("headers={:?} val_i={}", header,col_value_i);
    if let Some(col_namespace_i)	= header.iter().position(|x| x.to_ascii_lowercase() == col_namespace_nm) {
      p!("headers={:?} val_i={} ns_i={}", header,col_value_i,col_namespace_i);
    } else {
      p!("headers={:?} but no index :(",header);
    }
  } else {
    // use column index from arguments
  }
  for result in rdr.records() {
    let record = result?;
    // p!("{:?}", record);
  }
  Ok(())
}
fn get_first_arg() -> Result<OsString, Box<dyn Error>> { //! Returns the first positional argument sent to this process. If there are no positional arguments, then this returns an error
  match env::args_os().nth(1) {
    None           	=> Err(From::from("expected 1 argument, but got none")),
    Some(file_path)	=> Ok(file_path),
  }
}
fn tutor_run_stdin() -> Result<(), Box<dyn Error>> {
  let mut rdr = csv::Reader::from_reader(io::stdin());
  for result in rdr.records() { // iterator of CSV retuls
    // match result	{                              	// Examine our Result
    //   Ok(record)	=> {p!("{:?}", record);},      	// no problem, return the record
    //   Err(err)  	=> return Err(From::from(err)),	// convert err to Box<dyn Error>
    // } // or use ↓
    let record = result?;
    p!("{:?}", record);
  }
  Ok(())
}

