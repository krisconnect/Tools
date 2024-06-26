# Ethernaut Lessons Learned
1. Fallback - Logic errors. Make sure you understand how functions work with one another
2. Fallout - Remember to check version specific logs to see version specific vulnerabilites, do not trust dev comments :D
3. CoinFLip - Hardcoded randomization, use oracles
4. Telephone - Make sure to check for msg.sender and tx.origin correctly!
5. Token - Safe math in old solidity version. Underflow from a balance with 0 amount...
6. Delegation - Understand which address delegates
7. Force - ```selfdestruct``` force into other contracts! Noice.
8. Vault - Private on-chain data accessed by requesting storage slots at the right index
9. King - Sometimes you win by just not allowing anyone else to win. Know your functions! ```transfer``` can fail, we can just simply not implement receive functions in the attack contract!
10. Re-entrency - If CEI is not followed, the call function will trigger the recieve function in the attacker contract which can initiate a loop to drain funds before states are updated. With an elegant selfdestruct at the end? A work of art.
11. Elevator - This one I didn't get at first tbh, included as a reminder to focus on logic rather than words!
12. Privacy - Refresher on how (storage slots works)[https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html]
13. Gatekeeper I - Remember ```tx.origin``` and ```tx.sender```, write test for calculating gas and typecasting
14. Gatekeeper II - Data passed in the constructor will lead to a code size of zero, C background with bitwise operations comming in hot, XOR a with a is 0.
15. Naught Coin - What the rules don't specifically deny, it is allowed. Make sure to understand everything that is imported, or better yet only import what you need. Built everything locally and used cast to chew through it. Fantastic exercise!
16. Preservation - Delegate call overwriting buffer again and typecasting addresses correctly!
17. Recovery - Calculate the address of an ethereum contract (https://ethereum.stackexchange.com/questions/760/how-is-the-address-of-an-ethereum-contract-computed)
18. Magic Number - [Deploying contract written in bytecode](https://solidity-by-example.org/app/simple-bytecode-contract/) LOVE assembly.
19. Alien Codex - Older version magic, calculating underflow, checking contract layout. Need to revisit this one for sure...
20. Denial - Make sure that there is a require when working with ```call``` functions, that makes sure the code execution is uninterrupted!
21. Shop - Best I can describe it now is an interface hijack attack :/ If the interface that the contract is using points to ```msg.sender```, then the execution is going to happen in the attack contract's context. So the ```buy``` function is checking for values in the attacker's contract.
22. Dex - Rounding error, do the math!
23. DexTwo - Verify the token addresses used in swaps!
24. PuzzleWallet - Understand how functions interact! tbc
25.  

# Dev stuff
1. Removing elements from array: Copy the last element of the array to the index you want removed and just pop.
2. Order of inheritance: most base-like to derived.