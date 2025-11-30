// ? inside closure stackoverflow.com/a/55011757
use std::collections::BTreeMap;

fn parse_kv(data:&str) -> Option<BTreeMap<String,String>> {
  data.split('&')
    .map(|    kv|       kv.split('='))
    .map(|mut kv| Some((kv.next()?.into() // iterator of Option<(String,String)>
      ,                 kv.next()?.into())))
    .collect() // can automatically convert Iterator<Option<T>> into Option<Collection<T>>
    // allows short-circuiting: on 1st None yielded by the iterator, collect will immediately return with None, rather than continuing to process each element
}
