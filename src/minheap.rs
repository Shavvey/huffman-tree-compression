struct MinHeap {
    arr: Vec<u32>,
    capacity: u32,
    size: u32,
}

impl MinHeap {
    fn new(capacity: u32) -> MinHeap {
        MinHeap {
            arr: Vec::new(),
            capacity,
            size: 0,
        }
    }
}
