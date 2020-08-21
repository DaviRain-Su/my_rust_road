pub mod pat_1002 {
    use std::collections::HashMap;

    pub fn  write_the_number(n: String) -> Vec<String> {

        let mut  temp_hashmap = HashMap::new();
        temp_hashmap.insert(0u8, "ling");
        temp_hashmap.insert(1,"yi");
        temp_hashmap.insert(2, "er");
        temp_hashmap.insert(3, "san");
        temp_hashmap.insert(4, "si");
        temp_hashmap.insert(5, "wu");
        temp_hashmap.insert(6, "liu");
        temp_hashmap.insert(7, "qi");
        temp_hashmap.insert(8, "ba");
        temp_hashmap.insert(9, "jiu");


        let sum = n
            .chars()
            .map(|temp| {
            temp.to_string()
                .as_str()
                .parse::<u32>()
                .unwrap()
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();

        // println!("sum = {}", sum);

        let ret = sum
            .to_string()
            .chars()
            .map(|i| {
                let index = i.to_string().parse::<u8>().unwrap();
                temp_hashmap.get(&index)
                    .unwrap()
                    .to_string()
            }).collect::<Vec<String>>();

        // println!("ret = {:?}", ret);

        ret
    }
}