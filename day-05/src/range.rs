use std::collections::BTreeMap;

#[derive(Debug)]
struct RangeWrapper<T>(RangeInclusive<T>);

impl<T> RangeWrapper<T> {
    fn new(value: RangeInclusive<T>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
struct RangeMap<K, V>(BTreeMap<RangeWrapper<K>, V>);
impl<K, V> RangeMap<K, V> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }
}
