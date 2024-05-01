# Final project for the 6 week Solana Bootcamp of 2024

### Hashvault
This is a proof of concept project for providing a solution for proof of ownership utilizing blockchain. 

### Usage
1. The user/content creator before uploading their product run a sha512 hash function on the desired product.
2. Then with a user defined seed, the hash is then stored on chain in a pda.
3. Whenever the user needs to prove that a digital product belongs to them they can present the sha512 hash of the product and cross-refrence it with their seeded hash on chain, which is immutable.