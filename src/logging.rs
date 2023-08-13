use std::io   	::Write;
use log       	::{Level,LevelFilter};
use env_logger	::{Env,Builder,WriteStyle,fmt::Color};
use chrono    	::Local;

// #[macro_use] extern crate log; // added at main.rs
const time_fmt_1:&str = "%Y-%m-%d %H:%M:%S%.3f";
const time_fmt_s:&str = "%_M:%_S"; //23: 4: 4
const time_fmt_f:&str = "%a %_d %b %_y %_H:%_M"; //Tue 10 Jan 23 23: 4
const time_fmt  :&str = time_fmt_s;

pub fn log_init() { // todo: convert to env var style to allow env vars overriding this?, at least levels
  // let env = Env::default()  // `Env` lets us tweak what env vars to read and their defaults
  //   .filter_or("MY_LOG_LEVEL", "trace")
  //   .write_style_or("MY_LOG_STYLE", "always");
  // env_logger::init_from_env(env);

  let mut builder = Builder::new();
  builder
    .filter_module("various"             ,LevelFilter::Warn ) // filter for a specific module
    .filter_module("various::logging"    ,LevelFilter::max()) // filter log at most the specified level provided
    .filter_module("various::parser::key",LevelFilter::Warn)
    .format(|buf, record| {
      let mut style      	= buf.style();
      let mut level_style	= buf.style();
      match record.level() {
        Level::Error	=> {},//level_style.set_color(Color::Red).set_bold(true);()},
        Level::Warn 	=> {},//level_style.set_color(Color::Yellow);()},
        Level::Info 	=> {level_style.set_color(Color::Green);()},
        Level::Debug	=> {level_style.set_color(Color::Blue);()},
        Level::Trace	=> {},//level_style.set_color(Color::Cyan);()},
      };
      let level_icon = match record.level() {
        Level::Error	=> "❗️",
        Level::Warn 	=> "⚠️",
        Level::Info 	=> "🄸", //ℹ️
        Level::Debug	=> "🄳",
        Level::Trace	=> "🅃",
      };
      // let ts = buf.timestamp();
      let target = record.target();
      let target_rmost = match target.rsplit_once("::") {
        Some((pre,pos))	=> "→".to_string() + pos,
        None           	=> target.to_string(),
      };
      let ts = Local::now().format(time_fmt);
      writeln!(buf,"{} {} {} {}", ts,target_rmost, level_style.value(level_icon), style.value(record.args())
      )})
    .format_target(true)
    .init()
}
pub fn log_prints() {
  trace!("trace some trace log");
  debug!("debug Mary has a little lamb");
  info!("info {:?}", "And every where that Mary went");
  warn!("warn {:#?}", "The lamb was sure to go");
  error!("error {}", "Its fleece was white as snow");

  if log_enabled!(Level::Info) {
    let x = 3 * 4; // expensive computation
    info!("IF Level::Info the answer was: {}", x);
    debug!("debug Mary has a little lamb");
    error!("error {}", "Its fleece was white as snow");
    info!("info {:?}", "And every where that Mary went");
    warn!("warn {:#?}", "The lamb was sure to go");
  }
}
