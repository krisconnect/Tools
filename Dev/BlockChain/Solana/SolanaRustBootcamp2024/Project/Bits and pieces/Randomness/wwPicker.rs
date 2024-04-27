let random_number = loader::pubkey().to_bytes()[..4].try_into()?;
let mut rng = ChaChaRng::from_seed(random_number);
let rand_integer: u32 = rng.gen_range(0, 1000);
//Pseudo random nonsense

