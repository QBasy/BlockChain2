use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct MyData {
    pub value: u64,
}

fn main() {
    let data = MyData { value: 42 };
    let serialized_data = data.try_to_vec().unwrap();
    println!("{:?}", serialized_data);
}