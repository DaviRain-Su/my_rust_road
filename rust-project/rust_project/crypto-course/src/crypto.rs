pub trait Crypto {
    fn decode(&self, message: &str) -> String;
    fn encode(&self, message: &str) -> String;
    fn solution(&self, message: &str) -> Vec<String>;
}

const K: u8 = 3;
const N: u8 = 26;

pub struct ShiftCipher;

impl Crypto for ShiftCipher {
    fn decode(&self, message: &str) -> String {
        let message: Vec<u8> = message
            .chars()
            .map(|val| {
                let val = val as u8;
                let val = val - 65;
                val
            })
            .collect();

        let closure = |val: &u8| {
            let res = (val - K) % N; // k = 3
            res
        };

        let computer_after_message_vec: Vec<u8> = message.iter().map(closure).collect();

        let result: String = computer_after_message_vec
            .iter()
            .map(|val| {
                let val = val + 97;
                let val = val as char;
                val
            })
            .collect();
        result
    }

    fn encode(&self, message: &str) -> String {
        let message: Vec<u8> = message
            .chars()
            .map(|val| {
                let val = val as u8;
                let val = val - 97;
                val
            })
            .collect();

        let closure = |val: &u8| {
            let res = (val + K) % N; // k = 3
            res
        };

        let computer_after_message_vec: Vec<u8> = message.iter().map(closure).collect();

        let result: String = computer_after_message_vec
            .iter()
            .map(|val| {
                let val = val + 65;
                let val = val as char;
                val
            })
            .collect();
        result
    }

    fn solution(&self, message: &str) -> Vec<String> {
        let statement_vec: Vec<u8> = message
            .chars()
            .map(|val| {
                let val = val as u8;
                let val = val - 65;
                val
            })
            .collect();

        let mut ret: Vec<String> = Vec::new();
        let mut k = 0;
        loop {
            if k == 26 {
                break;
            }

            let computer_after_statement_vec: Vec<u8> = statement_vec
                .iter()
                .map(|val: &u8| {
                    let res = (val - k) % N; // k = 3
                    res
                })
                .collect();

            let result: String = computer_after_statement_vec
                .iter()
                .map(|val| {
                    let val = val + 97;
                    let val = val as char;
                    val
                })
                .collect();

            ret.push(result);
            k += 1;
        }
        ret
    }
}
