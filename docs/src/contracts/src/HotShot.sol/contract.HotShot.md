# HotShot
[Git Source](https://github.com/EspressoSystems/espresso-sequencer/blob/1cf02cf71b6620b1e983b68f0627b9cca54765e8/contracts/src/HotShot.sol)


## State Variables
### MAX_BLOCKS

```solidity
uint256 public constant MAX_BLOCKS = 500;
```


### commitments

```solidity
mapping(uint256 blockHeight => uint256 commitment) public commitments;
```


### blockHeight

```solidity
uint256 public blockHeight;
```


### _stakeAmounts

```solidity
mapping(uint256 index => uint256 amount) private _stakeAmounts;
```


### _stakingKeys

```solidity
BN254.G2Point[] private _stakingKeys;
```


## Functions
### _verifyQC


```solidity
function _verifyQC(QC calldata) private pure returns (bool);
```

### newBlocks


```solidity
function newBlocks(QC[] calldata qcs) external;
```

### addNewStakingKey

This function is for testing purposes only. The real sequencer
contract will perform several checks before adding a new key (e.g.
validate deposits).

*Stake table related functions*


```solidity
function addNewStakingKey(BN254.G2Point memory stakingKey, uint256 amount) public;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`stakingKey`|`BN254.G2Point`|public key for the BLS scheme|
|`amount`|`uint256`|stake corresponding to the staking key|


### getStakingKey


```solidity
function getStakingKey(uint256 index) public view returns (BN254.G2Point memory, uint256);
```

### verifyAggSig

*Verify an aggregated signature against a bitmap (use to reconstruct
the aggregated public key) and some stake threshold. If the stake
involved by the signers is bigger than the threshold and the signature is
valid then the validation passes, otherwise the transaction
reverts.*


```solidity
function verifyAggSig(bytes memory message, BN254.G1Point memory sig, bool[] memory bitmap, uint256 minStakeThreshold)
    public
    view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`message`|`bytes`|message to check the signature against|
|`sig`|`BN254.G1Point`|aggregated signature|
|`bitmap`|`bool[]`|bit vector that corresponds to the public keys of the stake table to take into account to build the aggregated public key|
|`minStakeThreshold`|`uint256`|total stake that must me matched by the signers in order for the signature to be valid|


## Events
### NewStakingKey

```solidity
event NewStakingKey(BN254.G2Point stakingKey, uint256 amount, uint256 index);
```

### NewBlocks

```solidity
event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);
```

## Errors
### TooManyBlocks

```solidity
error TooManyBlocks(uint256 numBlocks);
```

### InvalidQC

```solidity
error InvalidQC(uint256 blockNumber);
```

### IncorrectBlockNumber

```solidity
error IncorrectBlockNumber(uint256 blockNumber, uint256 expectedBlockNumber);
```

### NoKeySelected

```solidity
error NoKeySelected();
```

### NotEnoughStake

```solidity
error NotEnoughStake();
```

## Structs
### QC

```solidity
struct QC {
    uint256 height;
    uint256 blockCommitment;
    uint256 pad1;
    uint256 pad2;
}
```

