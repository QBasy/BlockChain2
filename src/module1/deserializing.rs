use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct MyData {
    pub value: u64,
}

fn main() {
    let serialized_data: Vec<u8> = vec![/* Serialized data bytes here */];
    let deserialized_data = MyData::try_from_slice(&serialized_data).unwrap();
    println!("{:?}", deserialized_data);
}
