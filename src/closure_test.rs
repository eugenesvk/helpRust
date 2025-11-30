#[test] fn parse_kv_test() {
  let res1 = parse_kv("test1=1&test2=2").unwrap();
  assert_eq!(    res1["test1"],"1");
  assert_eq!(    res1[        "test2"],"2");
  let res2 = parse_kv("test1=1&test2");
  assert_eq!(res2, None);
}
