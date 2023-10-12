# hehe

```shell
nix-shell 

DA_PORT=4501 
CONSENSUS_PORT=4502 
PORT=6060

target/release/web-server -p $DA_PORT 

target/release/web-server -p $CONSENSUS_PORT

# option 1: start 1 node of espresso
target/release/sequencer \
    --orchestrator-url http://127.0.0.1:$PORT \
    --da-server-url http://127.0.0.1:$DA_PORT \
    --consensus-server-url http://127.0.0.1:$CONSENSUS_PORT \
    -- http --port 8083 -- query --storage-path storage -- submit

# option 2 : start multiple nodes of espresso
N=5
for i in `seq $N`; do
    target/release/sequencer \
        --orchestrator-url http://localhost:$PORT \
        --da-server-url http://localhost:$DA_PORT \
        --consensus-server-url http://localhost:$CONSENSUS_PORT
done 
```


```shell
# ethermintd start
RPC_URL=local # local is by default port 8545 in forge command
RPC_URL="http://127.0.0.1:8545"
forge script DeployHotShot --broadcast --rpc-url "https://rpc.ankr.com/polygon_mumbai"--sender "0x933B70baE0d6fB875734a4B1ed2b4a2C6D9E5483" --private-key "965bbd010a5f698045098d6ae3e5e8f378c783fdb5dbc3e424b988211b7110ea"

# ganache
forge script DeployHotShot --broadcast --rpc-url $RPC_URL --sender "0x28440895469DfAB5db912efAe3F50F50CEBC2B4B" --private-key "0xf089121cb4c473ca0820accef6f461fd47baa6e28bc9260c70befa4d58e0a17b" 

--block-gas-limit 15000000  --gas-price 5000000000 --gas-limit 2442499 --block-base-fee-per-gas
```


A code which work
```shell
 forge create --rpc-url https://rpc-sepolia.rockx.com --private-key "965bbd010a5f698045098d6ae3e5e8f378c783fdb5dbc3e424b988211b7110ea" src/Counter.sol:Counter
```
with additional command 
```shell
forge create --rpc-url https://rpc-sepolia.rockx.com --private-key "965bbd010a5f698045098d6ae3e5e8f378c783fdb5dbc3e424b988211b7110ea" src/Counter.sol:Counter --verify rc/Counter.sol:Counter 
```

```shell
forge script DeployHotShot --broadcast --rpc-url sepolia
```

```shell
forge create --rpc-url https://rpc-sepolia.rockx.com --private-key "965bbd010a5f698045098d6ae3e5e8f378c783fdb5dbc3e424b988211b7110ea" contracts/script/HotShot.s.sol:DeployHotShot

[⠒] Compiling...
No files changed, compilation skipped
Deployer: 0x933B70baE0d6fB875734a4B1ed2b4a2C6D9E5483
Deployed to: 0xF2E5d66053256194Fc03C64e5E4a623C3993CAbc
Transaction hash: 0x93f46202afe5bab30a7ba97f7a7671be0335da9bbbc58c554f94ddfb7c6ca8c5
```

#Noooblien


forge script DeployHotShot --broadcast --rpc-url "http://localhost:8545" --sender "0x933B70baE0d6fB875734a4B1ed2b4a2C6D9E5483" --private-key "965BBD010A5F698045098D6AE3E5E8F378C783FDB5DBC3E424B988211B7110EA" --gas-limit 9100000