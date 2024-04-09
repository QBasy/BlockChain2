use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    let keypair = Keypair::new();
    let recent_blockhash = "RECENT_BLOCKHASH_HERE".parse().unwrap();
    let instructions = vec![solana_sdk::instruction::instruction::Instruction::new_with_bytes(
        &keypair.pubkey(),
        &[],
        vec![],
    )];
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );
    println!("{:?}", transaction);
}