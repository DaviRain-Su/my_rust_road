use std::collections::BTreeSet;

#[test]
fn test_create_vec() {
    let number: Vec<i32> = vec![];
    assert_eq!(true, number.is_empty());
}

#[test]
fn test_create_vec_with_string() {
    let words = vec!["step", "on", "no", "pets"];
    assert_eq!(false, words.is_empty());
}

#[test]
fn test_create_vec_with_u8_array() {
    let buffer = vec![0u8; 1024];
    assert_eq!(false, buffer.is_empty());
}

#[test]
fn test_transfer_hashset_to_vec() {
    // create BtreeSet
    let mut temp_hash_set = BTreeSet::new();
    for val in 0..10 {
        temp_hash_set.insert(val);
    }
    assert_eq!(10, temp_hash_set.len());

    // transfer BtreeSet to Vec
    let my_vec = temp_hash_set.into_iter().collect::<Vec<i32>>();
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], my_vec);
}

#[test]
fn test_use_vec_inex() {
    let line = vec![1, 2, 3, 4, 5];
    assert_eq!(vec![1, 2, 3, 4, 5], line);
    let first_line = &line[0]; //取得一个元素的引用
    assert_eq!(1, *first_line);

    let fifth_line = line[4]; // Copy
    assert_eq!(5, fifth_line);
    let second_line = line[1].clone(); // Clone
    assert_eq!(2, second_line);

    let my_ref = &line[..2];
    assert_eq!(my_ref, vec![1, 2]);
    let my_copy = line[..2].to_vec(); // Clone
    assert_eq!(my_copy, vec![1, 2]);
}

#[test]
fn test_slice_first() {
    let v = vec![1, 2, 3, 4];
    let ret = v.first();
    assert_eq!(Some(&1), ret);

    let v: Vec<i32> = vec![];
    let ret = v.first();
    assert_eq!(None, ret);
}

#[test]
fn test_slice_last() {
    let v = vec![1, 2, 3, 4];
    let ret = v.last();
    assert_eq!(Some(&4), ret);

    let v: Vec<i32> = vec![];
    let ret = v.last();
    assert_eq!(None, ret);
}

#[test]
fn test_slice_get() {
    let v = vec![1, 2, 3, 4];
    let ret = v.get(1);
    assert_eq!(Some(&2), ret);

    let ret = v.get(4);
    assert_eq!(None, ret);
}

#[test]
fn test_slice_first_mut() {
    let mut v = vec![1, 2, 3, 4];
    {
        let first = v.first_mut().unwrap();
        assert_eq!(1, *first);
        *first = 100;

    }
    assert_eq!(vec![100, 2, 3, 4], v);
}

#[test]
fn test_slice_last_mut() {
    let mut v = vec![1, 2, 3, 4];
    {
        let last = v.last_mut().unwrap();
        assert_eq!(4, *last);
        *last = 100;
    }
    assert_eq!(vec![1, 2, 3, 100], v);
}

#[test]
fn test_slice_get_mut() {
    let mut v = vec![1, 2, 3, 4];
    {
        let tmp = v.get_mut(3).unwrap();
        assert_eq!(4, *tmp);
        *tmp = 10;
    }
    assert_eq!(vec![1, 2, 3, 10], v);
    {
        let tmp = v.get_mut(4);
        assert_eq!(None, tmp);
    }
    assert_eq!(vec![1, 2, 3, 10], v);
}

#[test]
fn test_slice_to_vec() {
    // slice.to_vec() 克隆整个切片，返回一个向量
    let v = vec![1, 2, 3, 4];
    assert_eq!(v.to_vec(), v);
    assert_eq!(v[0..2].to_vec(),vec![1, 2]);
}

#[test]
fn test_iterator() {
    env_logger::init();

    let v = vec![1, 2, 3, 4, 5];
    for val in v {
        log::info!("{}", val);
    }
    let mut v = vec![1, 2, 3, 4, 5];

    {
        for val in &v {
            log::info!("{}", val);
        }

        for val in v.iter() {
            log::info!("{}", val);
        }
    }
    {
        for val in &mut v {
            log::info!("{}", val);
            *val += 1;
        }
    }

    assert_eq!(vec![2, 3, 4, 5, 6], v);

    {
        for val in v.iter_mut() {
            log::info!("{}", val);
            *val += 1;
        }
    }
    assert_eq!(vec![3, 4, 5, 6, 7], v);
}

#[test]
fn test_vec_with_capacity() {
    let v : Vec<i32> = Vec::with_capacity(10);
    assert_eq!(10, v.capacity());
}

#[test]
fn test_vec_reserve() {
    let mut v: Vec<i32> = Vec::with_capacity(2);
    v.push(1);
    v.push(2);
    assert_eq!(2, v.capacity());
    v.reserve(2);
    assert_eq!(2, v.len());
    assert_eq!(4, v.capacity());

    v.shrink_to_fit();
    assert_eq!(2, v.len());
    assert_eq!(2, v.capacity());
}

#[test]
fn test_vec_push_and_pop() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    assert_eq!(vec![1, 2, 3, 4, 5], v);
    assert_eq!(Some(5), v.pop());
    assert_eq!(vec![1, 2, 3, 4], v);
}

#[test]
fn test_vec_insert() {
    let mut v = vec![1, 2, 3, 4];
    v.insert(0, 0);
    assert_eq!(vec![0, 1, 2, 3, 4], v);
    v.insert(5, 5);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], v);
}

#[test]
fn test_vec_remove() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.remove(0);
    assert_eq!(vec![2, 3, 4, 5], v);
}