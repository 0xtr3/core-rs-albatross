use nimiq_keys::{Address, Signature, WebauthnPublicKey};
use nimiq_primitives::{account::AccountType, networks::NetworkId, transaction::TransactionError};
use nimiq_transaction::{
    account::AccountTransactionVerification, Transaction, WebauthnSignatureProof,
};
use nimiq_utils::merkle::Blake2bMerklePath;

#[test]
fn it_does_not_allow_creation() {
    let owner = Address::from([0u8; 20]);

    let transaction = Transaction::new_contract_creation(
        owner,
        AccountType::Basic,
        vec![],
        AccountType::Basic,
        vec![],
        100.try_into().unwrap(),
        0.try_into().unwrap(),
        0,
        NetworkId::Dummy,
    );

    assert_eq!(
        AccountType::verify_incoming_transaction(&transaction),
        Err(TransactionError::InvalidForRecipient)
    );
}

#[test]
fn it_does_not_allow_signalling() {
    let owner = Address::from([0u8; 20]);

    let transaction = Transaction::new_signaling(
        owner,
        AccountType::Basic,
        Address::from([1u8; 20]),
        AccountType::Basic,
        0.try_into().unwrap(),
        vec![],
        0,
        NetworkId::Dummy,
    );

    assert_eq!(
        AccountType::verify_incoming_transaction(&transaction),
        Err(TransactionError::ZeroValue)
    );
}

#[test]
fn it_can_verify_webauthn_signature_proofs() {
    let signature_proof = WebauthnSignatureProof {
        public_key: WebauthnPublicKey::from_bytes(&[
            2, 145, 87, 130, 102, 84, 114, 146, 139, 254, 114, 194, 134, 155, 187, 214, 188, 12,
            35, 147, 121, 213, 161, 80, 234, 94, 43, 25, 178, 5, 213, 54, 89,
        ])
        .unwrap(),
        merkle_path: Blake2bMerklePath::default(),
        signature: Signature::from_bytes(&[
            7, 185, 23, 233, 88, 246, 250, 252, 173, 116, 122, 201, 94, 32, 221, 241, 172, 99, 252,
            93, 153, 191, 69, 22, 233, 2, 233, 69, 145, 100, 16, 132, 1, 94, 247, 237, 70, 3, 74,
            241, 133, 18, 116, 58, 13, 203, 199, 167, 134, 170, 226, 113, 16, 184, 203, 209, 204,
            232, 27, 6, 43, 216, 12, 110,
        ])
        .unwrap(),
        host: "localhost:3000".to_string(),
        authenticator_data_suffix: vec![1, 101, 1, 154, 108],
        client_data_extra_fields: "".to_string(),
    };

    let tx_content = [
        0, 0, 154, 96, 106, 136, 176, 143, 11, 229, 208, 208, 107, 52, 170, 88, 232, 81, 173, 106,
        175, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        152, 150, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0,
    ];
    assert!(signature_proof.verify(&tx_content));
}
