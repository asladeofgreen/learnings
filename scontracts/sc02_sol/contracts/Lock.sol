// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Lock {
    // Field: lock period.
    uint public unlockTime;

    // Field: contract owner.
    address payable public owner;

    // Event: fired when a withdrawal is executed.
    event Withdrawal(uint amount, uint when);

    // Ctor.
    constructor(uint _unlockTime) payable {
        // Guards.
        require(
            block.timestamp < _unlockTime,
            "Unlock time should be in the future"
        );

        // Set initial state.
        unlockTime = _unlockTime;
        owner = payable(msg.sender);
    }

    // Method: If time lock has elapsed then funds can be withdrawn.
    function withdraw() public {
        // Uncomment this line, and the import of "hardhat/console.sol", to print a log in your terminal
        // console.log("Unlock time is %o and block timestamp is %o", unlockTime, block.timestamp);

        // Guards.
        require(block.timestamp >= unlockTime, "You can't withdraw yet");
        require(msg.sender == owner, "You aren't the owner");

        // Withdraw & signal.
        emit Withdrawal(address(this).balance, block.timestamp);
        owner.transfer(address(this).balance);
    }
}
