// SPDX-License-Identifier: MIT
pragma solidity ^0.8;


/*
Not cheap :D
*/
contract Chat {
    event Message(address indexed _from, address indexed _to, string message);

    function sendMessage(address _to, string calldata message) external {
        emit Message(msg.sender, _to, message);
    }
}