# BLSSig
[Git Source](https://github.com/EspressoSystems/espresso-sequencer/blob/1cf02cf71b6620b1e983b68f0627b9cca54765e8/contracts/src/libraries/BLSSig.sol)

*test top
This library implements the verification of the BLS signature scheme over the BN254 curve following
the rust implementation at
https://github.com/EspressoSystems/jellyfish/blob/e1e683c287f20160738e6e737295dd8f9e70577a/primitives/src/signatures/bls_over_bn254.rs*


## Functions
### _uint256FromBytesLittleEndian


```solidity
function _uint256FromBytesLittleEndian(uint8[] memory input) private pure returns (uint256);
```

### expand

*Takes a sequence of bytes and turn in into another sequence of bytes with fixed size. Equivalent of
https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/expander/mod.rs#L37*


```solidity
function expand(bytes memory message) internal pure returns (bytes memory);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`message`|`bytes`|message to be "expanded"|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|fixed size array of bytes|


### hashToField

*Hash a sequence of bytes to a field element in Fq. Equivalent of
https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/mod.rs#L65*


```solidity
function hashToField(bytes memory message) internal pure returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`message`|`bytes`|input message to be hashed|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|field element in Fq|


### hashToCurve

*Hash a sequence of bytes to a group element in BN254.G_1. We use the hash-and-pray algorithm for now.
Rust implementation can be found at
https://github.com/EspressoSystems/jellyfish/blob/e1e683c287f20160738e6e737295dd8f9e70577a/primitives/src/signatures/bls_over_bn254.rs#L318*


```solidity
function hashToCurve(bytes memory input) internal view returns (uint256, uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`input`|`bytes`|message to be hashed|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|group element in G_1|
|`<none>`|`uint256`||


### verifyBlsSig

*Verify a bls signature. Reverts if the signature is invalid*


```solidity
function verifyBlsSig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk) internal view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`message`|`bytes`|message to check the signature against|
|`sig`|`BN254.G1Point`|signature represented as a point in BN254.G_1|
|`pk`|`BN254.G2Point`|public key represented as a point in BN254.G_2|


## Errors
### BLSSigVerificationFailed

```solidity
error BLSSigVerificationFailed();
```

