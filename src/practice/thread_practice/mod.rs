#[cfg(test)]
mod test {
    use std::thread;
    use std::sync::mpsc::channel;

    #[test]
    fn test1() {
        let jh = thread::spawn(move || {
            println!("some thing");
        });
        jh.join();
        let jh = thread::Builder::new().name("child".to_string()).spawn(move || {
            println!("other thing");
        }).unwrap();
        jh.join();

        let (sender, receiver) = channel();
        thread::spawn(move || {
           sender.send(10).unwrap();
        });
        assert_eq!(receiver.recv().unwrap(), 10);

        let (sender, receiver) = channel();
        for i in 0..10 {
            let sender = sender.clone();
            thread::spawn(move || {
               sender.send(i).unwrap();
            });
        }
        for _ in 0..10 {
            let j = receiver.recv().unwrap();
            assert!(j < 10 && j >= 0);
        }
    }
}