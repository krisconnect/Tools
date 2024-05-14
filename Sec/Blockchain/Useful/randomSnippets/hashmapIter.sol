// SPDX-License-Identifier: MIT
pragma solidity ^0.8;



/*
In order to iterate through hashmaps we can add an additional hashmap to check weather or not it was already added to and array and
iterate through the array.
*/
contract IterableMapping {
    mapping(address => uint256) public balances;
    mapping(address => bool) public inserted;
    address[] public keys;

    function set(address addr, uint256 bal) external {
        balances[addr] = bal;

        if (!inserted[addr]) {
            inserted[addr] = true;
            keys.push(addr);
        }
    }

    function get(uint256 index) external view returns (uint256) {
        address key = keys[index];
        return balances[key];
    }

    function first() external view returns (uint256) {
        // Write your code here
        return balances[keys[0]];
    }

    function last() external view returns (uint256) {
        // Write your code here
        return balances[keys[keys.length-1]];
    }
}
