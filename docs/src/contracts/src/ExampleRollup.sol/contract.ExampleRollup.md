# ExampleRollup
[Git Source](https://github.com/EspressoSystems/espresso-sequencer/blob/1cf02cf71b6620b1e983b68f0627b9cca54765e8/contracts/src/ExampleRollup.sol)


## State Variables
### hotshot

```solidity
HotShot public hotshot;
```


### stateCommitment

```solidity
uint256 public stateCommitment;
```


### numVerifiedBlocks

```solidity
uint256 public numVerifiedBlocks;
```


## Functions
### constructor


```solidity
constructor(address hotshotAddress, uint256 initialState);
```

### _verifyProof


```solidity
function _verifyProof(uint256, uint256, uint256 oldState, uint256 newState, BatchProof calldata proof)
    private
    pure
    returns (bool);
```

### verifyBlocks


```solidity
function verifyBlocks(uint64 count, uint256 nextStateCommitment, BatchProof calldata proof) external;
```

## Events
### StateUpdate

```solidity
event StateUpdate(uint256 blockHeight, uint256 stateCommitment);
```

## Errors
### NotYetSequenced

```solidity
error NotYetSequenced(uint256 numVerifiedBlocks, uint64 count, uint256 blockHeight);
```

### InvalidProof

```solidity
error InvalidProof(uint256 firstBlock, uint256 lastBlock, uint256 oldState, uint256 newState, BatchProof proof);
```

### NoBlocks

```solidity
error NoBlocks();
```

## Structs
### BatchProof

```solidity
struct BatchProof {
    uint256 firstBlock;
    uint256 lastBlock;
    uint256 oldState;
    uint256 newState;
}
```

