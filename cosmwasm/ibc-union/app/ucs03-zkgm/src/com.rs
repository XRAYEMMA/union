use alloy::{
    primitives::{bytes, U256},
    sol_types::SolValue,
};
use unionlabs::primitives::Bytes;

pub const INSTR_VERSION_0: u8 = 0x00;
pub const INSTR_VERSION_1: u8 = 0x01;

pub const OP_FORWARD: u8 = 0x00;
pub const OP_MULTIPLEX: u8 = 0x01;
pub const OP_BATCH: u8 = 0x02;
pub const OP_FUNGIBLE_ASSET_ORDER: u8 = 0x03;

pub const ACK_ERR_ONLY_MAKER: &[u8] = &[0xDE, 0xAD, 0xC0, 0xDE];

pub const TAG_ACK_FAILURE: U256 = U256::ZERO;
pub const TAG_ACK_SUCCESS: U256 = U256::from_be_slice(&[1]);

pub const FILL_TYPE_PROTOCOL: U256 = U256::from_be_slice(&[0xB0, 0xCA, 0xD0]);
pub const FILL_TYPE_MARKETMAKER: U256 = U256::from_be_slice(&[0xD1, 0xCE, 0xC4, 0x5E]);

pub const FORWARD_SALT_MAGIC: U256 = U256::from_be_slice(&[
    0xC0, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xBA, 0xBE,
]);

alloy::sol! {
    #[derive(Debug)]
    struct ZkgmPacket {
        bytes32 salt;
        uint256 path;
        Instruction instruction;
    }

    #[derive(Debug)]
    struct Instruction {
        uint8 version;
        uint8 opcode;
        bytes operand;
    }

    struct Forward {
        uint256 path;
        uint64 timeout_height;
        uint64 timeout_timestamp;
        Instruction instruction;
    }

    struct Multiplex {
        bytes sender;
        bool eureka;
        bytes contract_address;
        bytes contract_calldata;
    }

    struct Batch {
        Instruction[] instructions;
    }

    #[derive(Debug, PartialEq)]
    struct FungibleAssetOrder {
        bytes sender;
        bytes receiver;
        bytes base_token;
        uint256 base_amount;
        string base_token_symbol;
        string base_token_name;
        uint8 base_token_decimals;
        uint256 base_token_path;
        bytes quote_token;
        uint256 quote_amount;
    }

    #[derive(Debug)]
    struct Ack {
        uint256 tag;
        bytes inner_ack;
    }

    struct BatchAck {
        bytes[] acknowledgements;
    }

    #[derive(Debug)]
    struct FungibleAssetOrderAck {
        uint256 fill_type;
        bytes market_maker;
    }
}

#[test]
fn lksadjf() {
    let instruction: Bytes = Instruction {
        version: INSTR_VERSION_1,
        opcode: OP_FUNGIBLE_ASSET_ORDER,
        operand: FungibleAssetOrder {
            sender: b"union19lnpcs0pvz9htcvm58jkp6ak55m49x5n8cn2cm".into(),
            receiver: bytes!("0x2C96e52fCE14BAa13868CA8182f8A7903e4e76E0"),
            base_token: b"muno".into(),
            base_amount: "1".parse().unwrap(),
            base_token_symbol: "muno".to_owned(),
            base_token_name: "muno".to_owned(),
            base_token_decimals: 6,
            base_token_path: "0".parse().unwrap(),
            quote_token: bytes!("0x05e0db400bf3f11e4107dca8ac8d61023e7d42af"),
            quote_amount: "1".parse().unwrap(),
        }
        .abi_encode_params()
        .into(),
    }
    .abi_encode_params()
    .into();

    println!("{instruction}, {}", u64::MAX);
}
