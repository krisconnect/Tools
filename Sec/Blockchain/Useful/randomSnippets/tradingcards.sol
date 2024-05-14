// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract StructExamples {
    struct Deck {
        string card;
        string rarity;
        uint256 year;
        address owner;
    }

    Deck[] public collection;

    function addCard(string memory card, string memory rarity, uint256 year) external {
        collection.push(Deck({card: card, rarity: rarity, year: year, owner: msg.sender}));
    }

    function getCard(uint256 index) external view returns (string memory card, string memory rarity, uint256 year, address owner) {
        Deck storage cardI = collection[index];
        return (cardI.card, cardI.rarity, cardI.year, cardI.owner);
    }

    function transferCard(uint256 index, address owner) external {
        collection[index].owner = owner;
    }
}
