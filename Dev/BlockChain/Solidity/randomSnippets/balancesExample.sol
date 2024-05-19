// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract MappingBasic {
    // Mapping from address to uint256 used to store balance of addresses
    mapping(address => uint256) public balances;

    // Nested mapping from address to address to bool
    // used to store if first address is a friend of second address
    mapping(address => mapping(address => bool)) public isFriend;

    function examples() external {
        // Insert
        balances[msg.sender] = 123;
        // Read
        uint256 bal = balances[msg.sender];
        // Update
        balances[msg.sender] += 456;
        // Delete
        delete balances[msg.sender];

        // msg.sender is a friend of this contract
        isFriend[msg.sender][address(this)] = true;
    }

    function get(address addr) external view returns (uint256) {
        return balances[addr];
    }

    function set(address addr, uint256 val) external {
        balances[addr] = val;
    }

    function remove(address addr) external {
        delete balances[addr];
    }
}
