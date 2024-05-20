// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract MultiSigWallet {
    event Deposit(address indexed sender, uint256 amount);
    event Submit(uint256 indexed txId);
    event Approve(address indexed owner, uint256 indexed txId);
    event Revoke(address indexed owner, uint256 indexed txId);
    event Execute(uint256 indexed txId);

    struct Transaction {
        address to;
        uint256 value;
        bytes data;
        bool executed;
    }

    address[] public owners;
    mapping(address => bool) public isOwner;
    uint256 public required;

    Transaction[] public transactions;
    // mapping from tx id => owner => bool
    mapping(uint256 => mapping(address => bool)) public approved;

    modifier onlyOwner() {
        require(isOwner[msg.sender], "not owner");
        _;
    }

    modifier txExists(uint256 txId) {
        require(txId < transactions.length, "tx does not exist");
        _;
    }

    modifier notApproved(uint256 txId) {
        require(!approved[txId][msg.sender], "tx already approved");
        _;
    }

    modifier notExecuted(uint256 txId) {
        require(!transactions[txId].executed, "tx already executed");
        _;
    }

    constructor(address[] memory _owners, uint256 _required) {
        require(_owners.length > 0, "owners required");
        require(_required > 0 && _required <= _owners.length, "invalid required number of owners");

        for (uint256 i; i < _owners.length; i++) {
            address owner = _owners[i];

            require(owner != address(0), "invalid owner");
            require(!isOwner[owner], "owner is not unique");

            isOwner[owner] = true;
            owners.push(owner);
        }

        required = _required;
    }
    
    receive() external payable {
        emit Deposit(msg.sender, msg.value);
    }
    
    function submit(address to, uint256 value, bytes calldata data)
        external
        onlyOwner
    {
        transactions.push(
            Transaction({to: to, value: value, data: data, executed: false})
        );
        emit Submit(transactions.length - 1);
    }

    
    function approve(uint256 txId) external onlyOwner txExists(txId) notApproved(txId) notExecuted(txId) {
        approved[txId][msg.sender] = true;
        emit Approve(msg.sender, txId);
    }
    
    function _getApprovalCount(uint256 txId) private view returns(uint256 count) {
        for (uint256 i; i < owners.length; i ++) {
            if (approved[txId][owners[i]]) {
                count += 1;
            }
        }
    }
    
    function execute(uint256 txId) external onlyOwner txExists(txId) notApproved(txId) notExecuted(txId) {
        require(_getApprovalCount(txId) >= required, "Not enough approvals!");
        Transaction storage transaction = transactions[txId];
        transaction.executed = true;
        
        (bool ok, ) = transaction.to.call{value: transaction.value}(transaction.data);
        require(ok, "Transaction failed");
        
        emit Execute(txId);
    }
    
    function revoke(uint256 txId) external onlyOwner txExists(txId) notExecuted(txId) {
        require(approved[txId][msg.sender], "Transaction not approved!");
        approved[txId][msg.sender] = false;
        emit Revoke(msg.sender, txId);
    }
}
