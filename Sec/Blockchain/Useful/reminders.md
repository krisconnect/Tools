# Ethernaut Lessons Learned
1. Fallback - Logic errors. Make sure you understand how functions work with one another
2. Fallout - Remember to check version specific logs to see version specific vulnerabilites, do not trust dev comments :D
3. CoinFLip - Hardcoded randomization, use oracles
4. Telephone - Make sure to check for msg.sender and tx.origin correctly!
5. Token - Safe math in old solidity version. Underflow from a balance with 0 amount...
6. Delegation - Understand which address delegates
7. 