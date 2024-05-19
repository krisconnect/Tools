// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract Account {
    address public bank;
    address public owner;
    uint256 public withdrawLimit;

    constructor(address _owner, uint256 _withdrawLimit) payable {
        bank = msg.sender;
        owner = _owner;
        withdrawLimit = _withdrawLimit;
    }
}

contract Bank {
    Account[] public accounts;

    function createAccount(address owner) external {
        Account account = new Account(owner, 0);
        accounts.push(account);
    }

    function createAccountAndSendEther(address owner) external payable {
        Account account = (new Account){value: msg.value}(owner, 0);
        accounts.push(account);
    }

    function createSavingsAccount(address owner) external {
        Account account = new Account(owner, 1000);
        accounts.push(account);
    }
}
