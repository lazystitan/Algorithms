#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Barrier, Condvar, Mutex};
    use std::thread;

    #[test]
    fn test1() {
        let jh = thread::spawn(move || {
            println!("some thing");
        });
        jh.join();
        let jh = thread::Builder::new()
            .name("child".to_string())
            .spawn(move || {
                println!("other thing");
            })
            .unwrap();
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

    #[test]
    fn test2() {
        let mut handles = Vec::with_capacity(10);
        let barrier = Arc::new(Barrier::new(10));

        for _ in 0..10 {
            let c = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before");
                c.wait();
                println!("after");
            }));
        }

        for handle in handles {
            handle.join();
        }
    }

    #[test]
    fn test3() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();
        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut start = lock.lock().unwrap();
            *start = true;
            cvar.notify_one();
        });

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
}
