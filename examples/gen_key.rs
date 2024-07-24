use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
	let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes
	rand::thread_rng().fill_bytes(&mut fx_key);
	println!("\n Generated key for HMAC: \n {fx_key:?}");

	let b64u = base64_url::encode(&fx_key);
	println!("\nKey b64u encoded:\n{b64u}");

	Ok(())
}
