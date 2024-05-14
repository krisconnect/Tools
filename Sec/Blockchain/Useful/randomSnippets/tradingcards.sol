// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract StructExamples {
    struct Deck {
        string card;
        string rarity;
        uint8 year;
        address owner;
    }

    Deck[] public collection;

    function addCard(string card, sring rarity, uint8 year) external {
        collection.push(Deck({card: card, rarity: rarity, year: year, owner: msg.sender}));
    }

    function getCard(uint256 index)
        external
        view
        returns (string card, sring rarity, uint8 year, address owner)
    {
        Deck storage collection = Deck[index];
        return (collection.card, collection.rarity, collection.year, collection.owner);
    }

    function transferCard(uint256 index, address owner) external {
        collection[index].owner = owner;
    }
}
