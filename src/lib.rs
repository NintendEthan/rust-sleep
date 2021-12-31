pub fn sleep(x: u64) {
    std::thread::sleep(std::time::Duration::from_secs(x));
}