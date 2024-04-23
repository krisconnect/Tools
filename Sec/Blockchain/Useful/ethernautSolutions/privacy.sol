// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Privacy {
    // Storage slot 0 stores boolean value. It doesn't take up all 32 bytes available to each of the 256-1 slots but because the item on storage slot 1 does, the boolean value takes up the whole slot. 
    bool public locked = true;
    // Storage slot 1 is fully taken up by the uint256
    uint256 public ID = block.timestamp;
    // Storage slot 2 starts with uint8 which is 1 byte out of 32 allowing
    uint8 private flattening = 10;
    // Storage slot 2 to also store the next uint8 and
    uint8 private denomination = 255;
    // Storage slot 2 to also store the uint16 which is an additional 2 bytes
    uint16 private awkwardness = uint16(block.timestamp);
    // As each element of the next array can't be fit into the remaining space in slot 2, it will be moved to slot 3, 4 and 5.

    bytes32[3] private data;

    constructor(bytes32[3] memory _data) {
        data = _data;
    }

    function unlock(bytes16 _key) public {
        require(_key == bytes16(data[2])); // We need the data at storage slot 5 ```await web3.eth.getStorageAt(addr, 5)```, then the first 16 bytes of that.
        locked = false;
    }

    /*
    A bunch of super advanced solidity algorithms...

      ,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`
      .,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,
      *.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^         ,---/V\
      `*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.    ~|__(o.o)
      ^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'  UU  UU
    */
}