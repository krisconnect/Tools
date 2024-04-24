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
15. Naught Coin - 