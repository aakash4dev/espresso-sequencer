# BLSTest
[Git Source](https://github.com/EspressoSystems/espresso-sequencer/blob/1cf02cf71b6620b1e983b68f0627b9cca54765e8/contracts/src/BLSTest.sol)


## Functions
### hashToField

This contract is for testing purposes only


```solidity
function hashToField(bytes memory message) public pure returns (uint256);
```

### hashToCurve


```solidity
function hashToCurve(bytes memory input) public view returns (uint256, uint256);
```

### verifyBlsSig


```solidity
function verifyBlsSig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk) public view;
```

