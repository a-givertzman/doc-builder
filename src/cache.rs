use std::{sync::{Arc, LazyLock, RwLock}, time::Instant};

///
/// 
struct Cache {
    data: LazyLock<Vec<usize>>,
    data1: Arc<RwLock<Option<Vec<usize>>>>,
}
impl Cache {
    pub fn new() -> Self {
        Self {
            data: LazyLock::new(Self::init),
            data1: Arc::new(RwLock::new(None))
        }
    }
    pub fn get(&self, i: usize) -> usize {
        (self.data)[i]
    }
    pub fn get1(&self, i: usize) -> usize {
        if self.data1.read().unwrap().is_none() {
            self.data1.write().unwrap().replace(Self::init());
        }
        self.data1.read().unwrap().as_deref().unwrap()[i]
    }
    fn init() -> Vec<usize> {
        vec![1, 2, 3]
    }
}
///
/// 
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let time = Instant::now();
    let cache = Cache::new();
    for _ in 0..1_000_000 {
        for i in [0, 1, 2] {
            cache.get(i);
        }
    }
    println!("Elapsed: {:?}", time.elapsed());
    let time = Instant::now();
    let cache = Cache::new();
    for _ in 0..1_000_000 {
        for i in [0, 1, 2] {
            cache.get1(i);
        }
    }
    println!("Elapsed: {:?}", time.elapsed());
}
