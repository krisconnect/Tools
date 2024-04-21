# Keys to the playground
### This is a step-by-step guide that allows you to start playing with smart contracts locally using solidity and foundry.
1, Getting the target code:\
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    a, Clone the target code base:
    ``` git clone *target URL* ```\
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    b, Install codium(or your IDE of choice) and forge:\
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    ``` curl -L https://foundry.paradigm.xyz | bash ``` \
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    c, Build the contract(best case scenario) ``` forge build``` \
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    d, Spin up a local test chain by running ```anvil``` \
2, Deploy locally:\
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    a, ``` forge create [contractName] --rpc-url http://127.0.0.1:8545 --private-key [anvilPrivateKey]```\
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    b, Make sure that all constructors etc. are deployed as well\
3, Write test and run them with ``` forge test``` \
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    a, To interact with specific functions use ``` cast send/call [contractAddress] "[functionName]([inputType])" [value/arguements] --rpc-url --private-key```

# Additional Resources
### This section is my collection of links/tools/articles that I find useful during research and testing.
- [Example exploits](https://github.com/Cyfrin/sc-exploits-minimized.git)
- [Reports](https://github.com/softstackHQ/Smart-Contract-Security-Audits)
- [Solana hacking](https://github.com/0xsanny/solsec)
- [Slither Usage](https://github.com/crytic/slither/wiki/Usage#detector-selection) // To go through output one-by-one, removing resolved findings.