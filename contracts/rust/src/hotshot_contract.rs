#[cfg(test)]

mod test {
    use crate::helpers::{MyG1Point, MyG2Point};
    use ark_ec::CurveGroup;
    use contract_bindings::bls_test::G2Point;
    use contract_bindings::hot_shot::{NewBlocksCall, Qc};
    use contract_bindings::{HotShot, TestL1System};
    use ethers::middleware::SignerMiddleware;
    use ethers::signers::Wallet;
    use ethers::types::Bytes;
    use ethers::{abi::AbiDecode, providers::Middleware, types::U256};
    use ethers_providers::Http;
    use jf_primitives::signatures::bls_over_bn254::{
        BLSOverBN254CurveSignatureScheme, SignKey, Signature, VerKey,
    };
    use jf_primitives::signatures::{AggregateableSignatureSchemes, SignatureScheme};
    use jf_utils::test_rng;
    use sequencer_utils::AnvilOptions;

    enum ResultExpected {
        Ok,
        Error,
    }

    #[async_std::test]
    async fn test_hotshot_block_commitment() {
        let anvil = AnvilOptions::default().spawn().await;
        let provider = anvil.provider();
        let TestL1System { hotshot, .. } = TestL1System::deploy(provider.clone()).await.unwrap();

        let block_num = U256::from(0);
        let commitment = U256::from(1234);
        let qcs = vec![Qc {
            height: block_num,
            block_commitment: commitment,
            ..Default::default()
        }];

        hotshot
            .new_blocks(qcs.clone())
            .send()
            .await
            .unwrap()
            .await
            .unwrap();

        assert_eq!(
            hotshot.commitments(block_num).call().await.unwrap(),
            commitment,
        );

        let (event, meta) = &hotshot
            .new_blocks_filter()
            .from_block(0)
            // Ethers does not set the contract address on filters created via contract bindings.
            // This seems like a bug and I have reported it:
            // https://github.com/gakonst/ethers-rs/issues/2528. In the mean time we can work around
            // by setting the address manually.
            .address(hotshot.address().into())
            .query_with_meta()
            .await
            .unwrap()[0];

        assert_eq!(event.first_block_number, block_num);
        assert_eq!(event.num_blocks, 1.into());

        // Parse the commitments from calldata.
        let tx = provider
            .get_transaction(meta.transaction_hash)
            .await
            .unwrap()
            .unwrap();
        let call = NewBlocksCall::decode(&tx.input).unwrap();
        assert_eq!(call.qcs, qcs);
    }

    #[async_std::test]
    async fn test_hotshot_stake_table() {
        let anvil = AnvilOptions::default().spawn().await;
        let provider = anvil.provider();
        let TestL1System { hotshot, .. } = TestL1System::deploy(provider.clone()).await.unwrap();

        let rng = &mut test_rng();
        for i in 0..5 {
            let (_, pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
            let pk = pk.to_affine();
            let pk_value: MyG2Point = pk.into();
            let amount = U256::from(10 * i);
            hotshot
                .add_new_staking_key(pk_value.clone().into(), amount)
                .send()
                .await
                .unwrap()
                .await
                .unwrap();

            let expected_stake_and_amount = (G2Point::from(pk_value), amount);
            assert_eq!(
                hotshot.get_staking_key(U256::from(i)).call().await.unwrap(),
                expected_stake_and_amount,
            );
        }
    }

    fn get_slice_from_bitmap<T: Clone>(input: Vec<T>, bitmap: &[bool]) -> Vec<T> {
        let mut res = vec![];
        for i in 0..input.len() {
            if bitmap[i] {
                res.push(input[i].clone());
            }
        }
        res
    }

    async fn check_agg_sig(
        bitmap: &Vec<bool>,
        staking_keys: &[(SignKey, VerKey)],
        messages: &[Bytes],
        stake_threshold: U256,
        hotshot: &HotShot<
            SignerMiddleware<
                ethers::providers::Provider<Http>,
                Wallet<ethers::core::k256::ecdsa::SigningKey>,
            >,
        >,
        res_expected: ResultExpected,
    ) {
        let n_sigs = bitmap.len();
        let rng = &mut test_rng();
        let mut signatures: Vec<Signature> = vec![];

        for (key_pair, message) in staking_keys.iter().take(n_sigs).zip(messages) {
            let sk = key_pair.0.clone();
            let sig = BLSOverBN254CurveSignatureScheme::sign(&(), &sk, message, rng).unwrap();
            signatures.push(sig.clone());
        }

        let vks = get_slice_from_bitmap(
            staking_keys.iter().map(|k| k.1).collect::<Vec<VerKey>>(),
            bitmap,
        );
        let sigs = get_slice_from_bitmap(signatures, bitmap);
        let agg_sig = BLSOverBN254CurveSignatureScheme::aggregate(&(), &vks, &sigs).unwrap();
        let agg_sig_value: MyG1Point = agg_sig.clone().sigma.into_affine().into();

        // Call the contract

        let message = messages[0].clone();

        let res = hotshot
            .verify_agg_sig(
                message.clone(),
                agg_sig_value.into(),
                bitmap.to_vec(),
                stake_threshold,
            )
            .call()
            .await;

        match res_expected {
            ResultExpected::Ok => assert!(res.is_ok()),
            ResultExpected::Error => assert!(res.is_err()),
        }
    }

    #[async_std::test]
    async fn test_validate_qc() {
        let anvil = AnvilOptions::default().spawn().await;
        let provider = anvil.provider();
        let TestL1System { hotshot, .. } = TestL1System::deploy(provider.clone()).await.unwrap();

        // Initialize the staking table with 5 keys
        let mut staking_keys = vec![];
        let rng = &mut test_rng();
        let n_sigs = 5;
        for i in 0..n_sigs {
            let (sk, pk) = BLSOverBN254CurveSignatureScheme::key_gen(&(), rng).unwrap();
            staking_keys.push((sk.clone(), pk));
            let pk = pk.to_affine();
            let pk_value: MyG2Point = pk.into();
            let amount = U256::from(10 * (i + 1));
            hotshot
                .add_new_staking_key(pk_value.clone().into(), amount)
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
        }

        // Happy path

        // Each signer is expected to sign the same message
        let messages = vec![Bytes::from(b"unique message"); n_sigs];
        let stake_threshold = U256::from(10);
        check_agg_sig(
            &vec![true, true, true, false, false],
            &staking_keys,
            &messages,
            stake_threshold,
            &hotshot,
            ResultExpected::Ok,
        )
        .await;

        check_agg_sig(
            &vec![false, true, true, false, true],
            &staking_keys,
            &messages,
            stake_threshold,
            &hotshot,
            ResultExpected::Ok,
        )
        .await;

        check_agg_sig(
            &vec![true, false, false, false, false],
            &staking_keys,
            &messages,
            stake_threshold,
            &hotshot,
            ResultExpected::Ok,
        )
        .await;

        // Error cases

        // Threshold is too low
        check_agg_sig(
            &vec![true, false, false, false, false],
            &staking_keys,
            &messages,
            U256::from(1000),
            &hotshot,
            ResultExpected::Error,
        )
        .await;

        // Signature is invalid because one message differs
        let unique_message = Bytes::from(b"unique message");
        let mut messages = vec![unique_message; n_sigs];
        messages[0] = Bytes::from(b"different message");
        check_agg_sig(
            &vec![true, true, true, false, true],
            &staking_keys,
            &messages,
            stake_threshold,
            &hotshot,
            ResultExpected::Error,
        )
        .await;

        // Bitmap length is too high
        check_agg_sig(
            &vec![true; 32],
            &staking_keys,
            &messages,
            stake_threshold,
            &hotshot,
            ResultExpected::Error,
        )
        .await;
    }
}
