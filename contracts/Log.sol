// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

contract Log {
    event Hello();
    event World();

    function emitHello() public {
        emit Hello();
    }

    function emitWorld() public {
        emit World();
    }
}
