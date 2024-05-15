// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract SendEther {
    receive() external payable {}

    fallback() external payable{}

    function sendViaTransfer(address payable to) external payable {
        // This function is no longer recommended for sending Ether.
        to.transfer(msg.value);
    }

    function sendViaSend(address payable to) external payable {
        // Send returns a boolean value indicating success or failure.
        // This function is not recommended for sending Ether.
        bool sent = to.send(msg.value);
        require(sent, "Failed to send Ether");
    }

    function sendEth(address payable to, uint256 amount) external {
        (bool sent,) = to.call{value: amount}("");
        require(sent, "Failed to send Ether");
    }
}

contract EtherWallet {
    address payable public owner;

    constructor() {
        owner = payable(msg.sender);
    }

    receive() external payable {}

    fallback() external payable {}

    function withdraw(uint256 amount) external {
        require(msg.sender == owner, "not owner");
        (bool sent,) = owner.call{value: amount}("");
        require(sent, "Failed to send Ether");
    }

    function getBalance() external view returns(uint) {
        return address(this).balance;
    }
}
