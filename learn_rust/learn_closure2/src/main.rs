// 实现一个缓存，只处理第一次传入的值，并保存

use std::collections::HashMap;
struct Cacher<T, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy,
    T: Fn(K) -> V,
{
    calcuation: T,
    value: HashMap<K, V>,
}
impl<T, K, V> Cacher<T, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy,
    T: Fn(K) -> V,
{
    pub fn new(calcuation: T) -> Self {
        Self {
            calcuation,
            value: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: K) -> V {
        if self.value.contains_key(&arg) {
            *self.value.get(&arg).unwrap()
        } else {
            let v = (self.calcuation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}
fn main() {
    let mut c = Cacher::new(|x| x + 1);
    let v = c.value(3);
    println!("v = {}", v);
    let v = c.value(4);
    println!("v = {}", v);

    let mut c = Cacher::new(|x: &str| x.len());

    let v = c.value("hello");
    println!("v = {}", v);
    let v = c.value("hello, world");
    println!("v = {}", v);
}
