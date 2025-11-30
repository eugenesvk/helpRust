#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,non_snake_case)]
#[derive(Default,Debug)] struct User {
  pub is_active	: bool,
  pub age      	: u8,
}
enum SearchOpt {
  IncludeInactive	(bool),
  AgeWithin      	(u8,u8),
}
struct SearchOpts {opts:Vec<SearchOpt>}
impl SearchOpts { // The builder pattern 👇
  fn default()                                                       ->      Self {
    Self {opts:vec![SearchOpt::IncludeInactive(false)]}}
  // fn include_inactive(mut self, include_inactive:bool)               ->      Self {
    // self.opts.push(SearchOpt::IncludeInactive(include_inactive));            self}
  // fn age_within      (mut self, min_age:u8, max_age:u8)              ->      Self {
    // self.opts.push(SearchOpt::AgeWithin(min_age, max_age));                  self}
  fn add_option      (&mut self, opt:SearchOpt)                      -> &mut Self {
    self.opts.push(opt);                                                     self}
}
fn search_users(users:Vec<User>, opts:&SearchOpts) -> Vec<User> {
  users.into_iter().filter(|user| {
    opts.opts.iter().all(|opt| match opt {
      SearchOpt::IncludeInactive(include_inactive) => {
        *include_inactive || user.is_active}
      SearchOpt::AgeWithin(min_age, max_age) => {
        user.age >= *min_age &&
        user.age <= *max_age}
    })}).collect()
}

fn test3() {
  let user1          	:User     	= User{is_active:true, age:6};
  let user2          	:User     	= User{is_active:true, age:21};
  let user_vec       	:Vec<User>	= vec![user1,user2];
  let mut search_opts	          	= SearchOpts::default();
  search_opts
    // .add_option(SearchOpt::IncludeInactive(false))
    .add_option(SearchOpt::AgeWithin(8,99));
  let result = search_users(user_vec,&search_opts);
  // let result = search_users(user_vec,SearchOpts::default()
    // .add_option(SearchOpt::IncludeInactive(false))
    // .add_option(SearchOpt::AgeWithin(8, 99)),);
  p!("{:?}",result);
}
