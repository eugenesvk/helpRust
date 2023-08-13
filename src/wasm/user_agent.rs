// global var without unsafe
use core::sync::atomic::{AtomicUsize,AtomicBool, Ordering};
static IS_MOBILE:AtomicBool = AtomicBool::new(false);
pub fn set_value(val:bool)        	{IS_MOBILE.store(val, Ordering::Relaxed)}
pub fn get_value(        ) -> bool	{IS_MOBILE.load (     Ordering::Relaxed)}


fn isMobile() {
  let win       	= web::window() .expect("should have a window in this context");
  let navigator 	= win.navigator();
  let user_agent	= navigator.user_agent();

  let os_mobile = ["iPhone","iPad","iPod","Android","webOS","BlackBerry","Windows Phone","IEMobile",];

  match navigator.user_agent() {
    Ok(agent_s) => { for os in os_mobile.iter() {
      let is_match = agent_s.to_lowercase()
        .matches(os.to_lowercase().as_str())
        .next().is_some();
      if is_match {set_value(true);break;}
      }}
    Err(_) => panic!("Failed to determine browser OS!"),
  }
}