mod thread {

    #[test]
    fn test_thread_example() {
        use std::thread;
        use std::time::Duration;

        let handle = thread::spawn(||{
            for i in 1..10 {
                println!("hi number {} from the spawned thread！", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        handle.join().unwrap();
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        
    }

    #[test]
    fn move_thread_example() {
        use std::thread;
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move ||{
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }

    #[test]
    fn example_channel() {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // println!("val is {}", val);
        });

        // let received = rx.try_recv().unwrap();
        // while let (ret) = rx.try_recv() {}
        loop {
            if let Ok(ret) = rx.try_recv() {
                println!("{}", ret);
                break;
            }else{
                println!("error");
                continue;   
            }
        }
        // println!("Got : {}", received);
    }

    #[test]
    fn send_multi_val() {
        use std::thread;
        use std::time::Duration;
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move ||{
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move ||{
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Go: {}!", received);
        }
    }

    #[test]
    fn mutex_example() {
        use std::sync::{Mutex, Arc};
        use std::thread;
        // use std::rc::Rc;

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move ||{
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result : {}", *counter.lock().unwrap());
    }
}