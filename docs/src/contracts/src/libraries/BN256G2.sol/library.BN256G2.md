# BN256G2
[Git Source](https://github.com/EspressoSystems/espresso-sequencer/blob/1cf02cf71b6620b1e983b68f0627b9cca54765e8/contracts/src/libraries/BN256G2.sol)

**Author:**
Mustafa Al-Bassam (mus@musalbas.com)

*Homepage: https://github.com/musalbas/solidity-BN256G2*


## State Variables
### FIELD_MODULUS

```solidity
uint256 internal constant FIELD_MODULUS = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47;
```


### TWISTBX

```solidity
uint256 internal constant TWISTBX = 0x2b149d40ceb8aaae81be18991be06ac3b5b4c5e559dbefa33267e6dc24a138e5;
```


### TWISTBY

```solidity
uint256 internal constant TWISTBY = 0x9713b03af0fed4cd2cafadeed8fdf4a74fa084e52d1852e4a2bd0685c315d2;
```


### PTXX

```solidity
uint256 internal constant PTXX = 0;
```


### PTXY

```solidity
uint256 internal constant PTXY = 1;
```


### PTYX

```solidity
uint256 internal constant PTYX = 2;
```


### PTYY

```solidity
uint256 internal constant PTYY = 3;
```


### PTZX

```solidity
uint256 internal constant PTZX = 4;
```


### PTZY

```solidity
uint256 internal constant PTZY = 5;
```


## Functions
### ECTwistAdd

Add two twist points


```solidity
function ECTwistAdd(
    uint256 pt1xx,
    uint256 pt1xy,
    uint256 pt1yx,
    uint256 pt1yy,
    uint256 pt2xx,
    uint256 pt2xy,
    uint256 pt2yx,
    uint256 pt2yy
) internal view returns (uint256, uint256, uint256, uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`pt1xx`|`uint256`|Coefficient 1 of x on point 1|
|`pt1xy`|`uint256`|Coefficient 2 of x on point 1|
|`pt1yx`|`uint256`|Coefficient 1 of y on point 1|
|`pt1yy`|`uint256`|Coefficient 2 of y on point 1|
|`pt2xx`|`uint256`|Coefficient 1 of x on point 2|
|`pt2xy`|`uint256`|Coefficient 2 of x on point 2|
|`pt2yx`|`uint256`|Coefficient 1 of y on point 2|
|`pt2yy`|`uint256`|Coefficient 2 of y on point 2|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|(pt3xx, pt3xy, pt3yx, pt3yy)|
|`<none>`|`uint256`||
|`<none>`|`uint256`||
|`<none>`|`uint256`||


### ECTwistMul

Multiply a twist point by a scalar


```solidity
function ECTwistMul(uint256 s, uint256 pt1xx, uint256 pt1xy, uint256 pt1yx, uint256 pt1yy)
    internal
    view
    returns (uint256, uint256, uint256, uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`s`|`uint256`|    Scalar to multiply by|
|`pt1xx`|`uint256`|Coefficient 1 of x|
|`pt1xy`|`uint256`|Coefficient 2 of x|
|`pt1yx`|`uint256`|Coefficient 1 of y|
|`pt1yy`|`uint256`|Coefficient 2 of y|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|(pt2xx, pt2xy, pt2yx, pt2yy)|
|`<none>`|`uint256`||
|`<none>`|`uint256`||
|`<none>`|`uint256`||


### GetFieldModulus

Get the field modulus


```solidity
function GetFieldModulus() internal pure returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The field modulus|


### submod


```solidity
function submod(uint256 a, uint256 b, uint256 n) internal pure returns (uint256);
```

### _FQ2Mul


```solidity
function _FQ2Mul(uint256 xx, uint256 xy, uint256 yx, uint256 yy) internal pure returns (uint256, uint256);
```

### _FQ2Muc


```solidity
function _FQ2Muc(uint256 xx, uint256 xy, uint256 c) internal pure returns (uint256, uint256);
```

### _FQ2Add


```solidity
function _FQ2Add(uint256 xx, uint256 xy, uint256 yx, uint256 yy) internal pure returns (uint256, uint256);
```

### _FQ2Sub


```solidity
function _FQ2Sub(uint256 xx, uint256 xy, uint256 yx, uint256 yy) internal pure returns (uint256 rx, uint256 ry);
```

### _FQ2Div


```solidity
function _FQ2Div(uint256 xx, uint256 xy, uint256 yx, uint256 yy) internal view returns (uint256, uint256);
```

### _FQ2Inv


```solidity
function _FQ2Inv(uint256 x, uint256 y) internal view returns (uint256, uint256);
```

### _isOnCurve


```solidity
function _isOnCurve(uint256 xx, uint256 xy, uint256 yx, uint256 yy) internal pure returns (bool);
```

### _modInv


```solidity
function _modInv(uint256 a, uint256 n) internal view returns (uint256 result);
```

### _fromJacobian


```solidity
function _fromJacobian(uint256 pt1xx, uint256 pt1xy, uint256 pt1yx, uint256 pt1yy, uint256 pt1zx, uint256 pt1zy)
    internal
    view
    returns (uint256 pt2xx, uint256 pt2xy, uint256 pt2yx, uint256 pt2yy);
```

### _ECTwistAddJacobian


```solidity
function _ECTwistAddJacobian(
    uint256 pt1xx,
    uint256 pt1xy,
    uint256 pt1yx,
    uint256 pt1yy,
    uint256 pt1zx,
    uint256 pt1zy,
    uint256 pt2xx,
    uint256 pt2xy,
    uint256 pt2yx,
    uint256 pt2yy,
    uint256 pt2zx,
    uint256 pt2zy
) internal pure returns (uint256[6] memory pt3);
```

### _ECTwistDoubleJacobian


```solidity
function _ECTwistDoubleJacobian(
    uint256 pt1xx,
    uint256 pt1xy,
    uint256 pt1yx,
    uint256 pt1yy,
    uint256 pt1zx,
    uint256 pt1zy
) internal pure returns (uint256 pt2xx, uint256 pt2xy, uint256 pt2yx, uint256 pt2yy, uint256 pt2zx, uint256 pt2zy);
```

### _ECTwistMulJacobian


```solidity
function _ECTwistMulJacobian(
    uint256 d,
    uint256 pt1xx,
    uint256 pt1xy,
    uint256 pt1yx,
    uint256 pt1yy,
    uint256 pt1zx,
    uint256 pt1zy
) internal pure returns (uint256[6] memory pt2);
```

