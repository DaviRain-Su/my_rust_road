#[derive(PartialOrd, PartialEq)]
struct StackMin<T: PartialEq + Clone + PartialOrd> {
    stack_data: Vec<T>,
    stack_min: Vec<T>,
}

impl <T : PartialEq + Clone + PartialOrd> StackMin<T> {
    pub fn new() -> Self {
        Self {
            stack_data: Vec::new(),
            stack_min: Vec::new(),
        }
    }

    pub fn push(&mut self, elem : T)  {
        let tmp_elem = elem.clone();
        if self.stack_min.is_empty() {
            self.stack_min.push(elem);
        } else if &elem <= self.get_min().unwrap() {
            self.stack_min.push(elem);
        }
        // let tmp_elem = elem.clone();
        self.stack_data.push(tmp_elem);
    }
    pub fn pop(&mut self) -> Option<T> {
        let ret = self.stack_data.pop();

        let lhs = self.get_min();
        match lhs {
            Some(lhs_temp) => {
                match ret {
                    Some(ref rhs_temp) => {
                        if lhs_temp == rhs_temp {
                            self.stack_min.pop();
                        }
                    },
                    None => {
                        println!("stack data is empty!");
                    }
                }
            },
            None => println!("stack min is empty")
        }
        ret
    }
    pub fn get_min(&self) -> Option<&T> {
        self.stack_min.first()
    }
}

#[test]
fn test_stack_min() {
    let mut s = StackMin::new();
    s.push(2);
    s.push(4);
    s.push(1);
    assert_eq!(s.get_min(), Some(&1));
}