pub use ed_25519::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod ed_25519 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("scalarMult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scalarMult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("p"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Ed25519.Point"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("scalarMultBase"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scalarMultBase"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("splitSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("splitSignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("R"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifySignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::Some(true),
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ED25519_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0B\xC5\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\n\xB7$(\x14a\0QW\x80cu\x0FA\xDC\x14a\0{W\x80c\xA7\xBBX\x03\x14a\0\x9BW\x80c\xC4\xF4\x91+\x14a\0\xAEW[`\0\x80\xFD[a\0da\0_6`\x04a\t\\V[a\0\xC1V[`@Qa\0r\x92\x91\x90a\n\xA5V[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ea\0\x896`\x04a\x08\x9CV[a\x01YV[`@Qa\0r\x91\x90a\n\x97V[a\0da\0\xA96`\x04a\t\x01V[a\x02\x83V[a\0da\0\xBC6`\x04a\t>V[a\x02\xC1V[`\0\x80a\0\xCCa\x06\xEDV[`\0\x81R`\x01` \x82\x01\x81\x90R`@\x82\x01R[\x84\x15a\x01\x15W\x84`\x01\x16`\x01\x14\x15a\0\xFEWa\0\xFB\x81\x85a\x03\xB4V[\x90P[`\x01\x85\x90\x1C\x94Pa\x01\x0E\x84a\x05JV[\x93Pa\0\xDFV[`\0a\x01$\x82`@\x01Qa\x06\x8DV[\x90P`\x13`\x01`\xFF\x1B\x03\x82Q\x82\x90\t\x82R`\x13`\x01`\xFF\x1B\x03\x81\x83` \x01Q\t` \x83\x01\x81\x90R\x91Q\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0a\x01g\x85a\x02\x83V[\x91P\x91Pa\x01sa\x07\x0EV[\x82\x81R` \x81\x01\x82\x90R`\0\x80a\x01\x89\x84a\x02\xC1V[\x91P\x91P`\0\x85\x88\x8B`@Q` \x01a\x01\xA4\x93\x92\x91\x90a\n`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80a\x02\t\x83`\0\x1C`@Q\x80``\x01`@R\x80\x8D`\0`\x02\x81\x10a\x01\xDEW\xFE[` \x02\x01Q\x81R` \x01\x8D`\x01`\x02\x81\x10a\x01\xF5W\xFE[` \x02\x01Q\x81R` \x01`\x01\x81RPa\0\xC1V[\x91P\x91Pa\x02\x15a\x06\xEDV[a\x02U`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x84\x81R` \x01`\x01\x81RP`@Q\x80``\x01`@R\x80\x89\x81R` \x01\x88\x81R` \x01`\x01\x81RPa\x03\xB4V[\x87Q\x81Q\x91\x92P\x14\x80\x15a\x02pWP` \x80\x88\x01Q\x90\x82\x01Q\x14[\x99PPPPPPPPPP[\x93\x92PPPV[`\0\x80\x82Q`@\x14a\x02\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xA7\x90a\n\xC0V[`@Q\x80\x91\x03\x90\xFD[PP` \x81\x01Q`@\x90\x91\x01Q\x90\x91V[`\0\x80a\x02\xCCa\x06\xEDV[a\x02\xD4a\x06\xEDV[\x7F!i6\xD3\xCDnS\xFE\xC0\xA4\xE21\xFD\xD6\xDC\\i,\xC7`\x95%\xA7\xB2\xC9V-`\x8F%\xD5\x1A\x82R\x7FfffffffffffffffffffffffffffffffX` \x80\x84\x01\x91\x90\x91R`\x01`@\x80\x85\x01\x82\x90R`\0\x84R\x91\x83\x01\x81\x90R\x90\x82\x01R[\x84\x15a\x03pW\x84`\x01\x16`\x01\x14\x15a\x03YWa\x03V\x81\x83a\x03\xB4V[\x90P[`\x01\x85\x90\x1C\x94Pa\x03i\x82a\x05JV[\x91Pa\x03:V[`\0a\x03\x7F\x82`@\x01Qa\x06\x8DV[\x90P`\x13`\x01`\xFF\x1B\x03\x82Q\x82\x90\t\x82R`\x13`\x01`\xFF\x1B\x03\x81\x83` \x01Q\t` \x83\x01\x81\x90R\x91Q\x94P\x90\x92PPP\x91P\x91V[a\x03\xBCa\x06\xEDV[a\x03\xC4a\x07,V[`\x13`\x01`\xFF\x1B\x03\x83`@\x01Q\x85`@\x01Q\t\x81R`\x13`\x01`\xFF\x1B\x03\x81Q\x80\t` \x82\x01R`\x13`\x01`\xFF\x1B\x03\x83Q\x85Q\t`@\x82\x01R`\x13`\x01`\xFF\x1B\x03\x83` \x01Q\x85` \x01Q\t``\x82\x01R`\x13`\x01`\xFF\x1B\x03\x80\x82``\x01Q\x83`@\x01Q\t\x7FR\x03l\xEE+o\xFEs\x8C\xC7@ywy\xE8\x98\0p\nMAA\xD8\xABu\xEBM\xCA\x13Yx\xA3\t`\x80\x82\x01R`\x13`\x01`\xFF\x1B\x03\x81`\x80\x01Q`\x13`\x01`\xFF\x1B\x03\x03\x82` \x01Q\x08`\xA0\x82\x01R`\x13`\x01`\xFF\x1B\x03\x81`\x80\x01Q\x82` \x01Q\x08`\xC0\x82\x01R`\x13`\x01`\xFF\x1B\x03\x80\x82``\x01Q`\x13`\x01`\xFF\x1B\x03\x03`\x13`\x01`\xFF\x1B\x03\x80a\x04\xAEW\xFE[\x84`@\x01Q`\x13`\x01`\xFF\x1B\x03\x03`\x13`\x01`\xFF\x1B\x03\x80a\x04\xCBW\xFE[`\x13`\x01`\xFF\x1B\x03` \x8A\x01Q\x8AQ\x08`\x13`\x01`\xFF\x1B\x03` \x8C\x01Q\x8CQ\x08\t\x08\x08`\x13`\x01`\xFF\x1B\x03`\xA0\x84\x01Q\x84Q\t\t\x82R`\x13`\x01`\xFF\x1B\x03\x80\x82`@\x01Q\x83``\x01Q\x08`\x13`\x01`\xFF\x1B\x03`\xC0\x84\x01Q\x84Q\t\t` \x83\x01R`\x13`\x01`\xFF\x1B\x03\x81`\xC0\x01Q\x82`\xA0\x01Q\t`@\x83\x01RP\x92\x91PPV[a\x05Ra\x06\xEDV[a\x05Za\x07,V[`\x13`\x01`\xFF\x1B\x03` \x84\x01Q\x84Q\x08\x81R`\x13`\x01`\xFF\x1B\x03\x81Q\x80\t` \x82\x01R`\x13`\x01`\xFF\x1B\x03\x83Q\x80\t`@\x82\x01R`\x13`\x01`\xFF\x1B\x03` \x84\x01Q\x80\t``\x82\x01\x81\x90R`@\x82\x01Q`\x13`\x01`\xFF\x1B\x03\x90\x81\x03`\x80\x84\x01\x81\x90R\x90\x91\x90\x08`\xA0\x82\x01R`\x13`\x01`\xFF\x1B\x03`@\x84\x01Q\x80\t`\xE0\x82\x01R`\x13`\x01`\xFF\x1B\x03\x80\x82`\xE0\x01Q`\x02\t`\x13`\x01`\xFF\x1B\x03\x03\x82`\xA0\x01Q\x08`\xC0\x82\x01R`\x13`\x01`\xFF\x1B\x03`\xC0\x82\x01Q`\x13`\x01`\xFF\x1B\x03\x83``\x01Q`\x13`\x01`\xFF\x1B\x03\x03`\x13`\x01`\xFF\x1B\x03\x80a\x06/W\xFE[\x85`@\x01Q`\x13`\x01`\xFF\x1B\x03\x03\x86` \x01Q\x08\x08\t\x82R`\x13`\x01`\xFF\x1B\x03\x80\x82``\x01Q`\x13`\x01`\xFF\x1B\x03\x03\x83`\x80\x01Q\x08\x82`\xA0\x01Q\t` \x83\x01R`\x13`\x01`\xFF\x1B\x03\x81`\xC0\x01Q\x82`\xA0\x01Q\t`@\x83\x01RP\x91\x90PV[`\0\x80`\x02`\x13`\x01`\xFF\x1B\x03\x03\x90P`\0`\x13`\x01`\xFF\x1B\x03\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` \x81`\xC0\x83`\x05`\0\x19\xFAa\x06\xE4W`\0\x80\xFD[Q\x94\x93PPPPV[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x808\x839P\x91\x92\x91PPV[`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x82W`\0\x80\xFD[`\x02a\x07\x95a\x07\x90\x82a\n\xF7V[a\n\xD0V[\x91P\x81\x83\x85` \x84\x02\x82\x01\x11\x15a\x07\xABW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x07\xD7W\x81a\x07\xC1\x88\x82a\x07\xE1V[\x84RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a\x07\xAEV[PPPP\x92\x91PPV[\x805a\x07\xEC\x81a\x0BkV[\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x08\x03W`\0\x80\xFD[\x815a\x08\x11a\x07\x90\x82a\x0B\x15V[\x91P\x80\x82R` \x83\x01` \x83\x01\x85\x83\x83\x01\x11\x15a\x08-W`\0\x80\xFD[a\x088\x83\x82\x84a\x0B_V[PPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x08SW`\0\x80\xFD[a\x08]``a\n\xD0V[\x90P`\0a\x08k\x84\x84a\x07\xE1V[\x82RP` a\x08|\x84\x84\x83\x01a\x07\xE1V[` \x83\x01RP`@a\x08\x90\x84\x82\x85\x01a\x07\xE1V[`@\x83\x01RP\x92\x91PPV[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a\x08\xB1W`\0\x80\xFD[`\0a\x08\xBD\x86\x86a\x07\xE1V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xDAW`\0\x80\xFD[a\x08\xE6\x86\x82\x87\x01a\x07\xF2V[\x92PP`@a\x08\xF7\x86\x82\x87\x01a\x07qV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\t\x13W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t*W`\0\x80\xFD[a\t6\x84\x82\x85\x01a\x07\xF2V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\tPW`\0\x80\xFD[`\0a\t6\x84\x84a\x07\xE1V[`\0\x80`\x80\x83\x85\x03\x12\x15a\toW`\0\x80\xFD[`\0a\t{\x85\x85a\x07\xE1V[\x92PP` a\t\x8C\x85\x82\x86\x01a\x08AV[\x91PP\x92P\x92\x90PV[`\0a\t\xA2\x83\x83a\n\rV[PP` \x01\x90V[a\t\xB3\x81a\x0B@V[a\t\xBD\x81\x84a\x0BLV[\x92Pa\t\xC8\x82a\x0B=V[\x80`\0[\x83\x81\x10\x15a\t\xF6W\x81Qa\t\xE0\x87\x82a\t\x96V[\x96Pa\t\xEB\x83a\x0BFV[\x92PP`\x01\x01a\t\xCCV[PPPPPPV[a\n\x07\x81a\x0BZV[\x82RPPV[a\n\x07\x81a\x0B=V[a\n\x07a\n\"\x82a\x0B=V[a\x0B=V[`\0a\n4`\x18\x83a\x0BQV[\x7FInvalid signature length\0\0\0\0\0\0\0\0\x81R` \x01\x92\x91PPV[`\0a\nl\x82\x86a\n\x16V[` \x82\x01\x91Pa\n|\x82\x85a\t\xAAV[`@\x82\x01\x91Pa\n\x8C\x82\x84a\n\x16V[P` \x01\x93\x92PPPV[` \x81\x01a\x07\xEC\x82\x84a\t\xFEV[`@\x81\x01a\n\xB3\x82\x85a\n\rV[a\x02|` \x83\x01\x84a\n\rV[` \x80\x82R\x81\x01a\x07\xEC\x81a\n'V[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xEFW`\0\x80\xFD[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\x0EW`\0\x80\xFD[P` \x02\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B,W`\0\x80\xFD[P` `\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01\x90V[\x90V[P`\x02\x90V[` \x01\x90V[\x91\x90PV[\x90\x81R` \x01\x90V[\x15\x15\x90V[\x82\x81\x837P`\0\x91\x01RV[a\x0Bt\x81a\x0B=V[\x81\x14a\x0B\x7FW`\0\x80\xFD[PV\xFE\xA3ebzzr1X Yr\t^\x87\xEB\"\xF45\xD6\x9D\x95\xAC1\0\x81\x90\xDDX\xC5\xD4\x1C\x97\x04\t9I\xFC\x05[`-lexperimental\xF5dsolcC\0\x05\x11\0@";
    /// The bytecode of the contract.
    pub static ED25519_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\n\xB7$(\x14a\0QW\x80cu\x0FA\xDC\x14a\0{W\x80c\xA7\xBBX\x03\x14a\0\x9BW\x80c\xC4\xF4\x91+\x14a\0\xAEW[`\0\x80\xFD[a\0da\0_6`\x04a\t\\V[a\0\xC1V[`@Qa\0r\x92\x91\x90a\n\xA5V[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ea\0\x896`\x04a\x08\x9CV[a\x01YV[`@Qa\0r\x91\x90a\n\x97V[a\0da\0\xA96`\x04a\t\x01V[a\x02\x83V[a\0da\0\xBC6`\x04a\t>V[a\x02\xC1V[`\0\x80a\0\xCCa\x06\xEDV[`\0\x81R`\x01` \x82\x01\x81\x90R`@\x82\x01R[\x84\x15a\x01\x15W\x84`\x01\x16`\x01\x14\x15a\0\xFEWa\0\xFB\x81\x85a\x03\xB4V[\x90P[`\x01\x85\x90\x1C\x94Pa\x01\x0E\x84a\x05JV[\x93Pa\0\xDFV[`\0a\x01$\x82`@\x01Qa\x06\x8DV[\x90P`\x13`\x01`\xFF\x1B\x03\x82Q\x82\x90\t\x82R`\x13`\x01`\xFF\x1B\x03\x81\x83` \x01Q\t` \x83\x01\x81\x90R\x91Q\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0a\x01g\x85a\x02\x83V[\x91P\x91Pa\x01sa\x07\x0EV[\x82\x81R` \x81\x01\x82\x90R`\0\x80a\x01\x89\x84a\x02\xC1V[\x91P\x91P`\0\x85\x88\x8B`@Q` \x01a\x01\xA4\x93\x92\x91\x90a\n`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80a\x02\t\x83`\0\x1C`@Q\x80``\x01`@R\x80\x8D`\0`\x02\x81\x10a\x01\xDEW\xFE[` \x02\x01Q\x81R` \x01\x8D`\x01`\x02\x81\x10a\x01\xF5W\xFE[` \x02\x01Q\x81R` \x01`\x01\x81RPa\0\xC1V[\x91P\x91Pa\x02\x15a\x06\xEDV[a\x02U`@Q\x80``\x01`@R\x80\x85\x81R` \x01\x84\x81R` \x01`\x01\x81RP`@Q\x80``\x01`@R\x80\x89\x81R` \x01\x88\x81R` \x01`\x01\x81RPa\x03\xB4V[\x87Q\x81Q\x91\x92P\x14\x80\x15a\x02pWP` \x80\x88\x01Q\x90\x82\x01Q\x14[\x99PPPPPPPPPP[\x93\x92PPPV[`\0\x80\x82Q`@\x14a\x02\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xA7\x90a\n\xC0V[`@Q\x80\x91\x03\x90\xFD[PP` \x81\x01Q`@\x90\x91\x01Q\x90\x91V[`\0\x80a\x02\xCCa\x06\xEDV[a\x02\xD4a\x06\xEDV[\x7F!i6\xD3\xCDnS\xFE\xC0\xA4\xE21\xFD\xD6\xDC\\i,\xC7`\x95%\xA7\xB2\xC9V-`\x8F%\xD5\x1A\x82R\x7FfffffffffffffffffffffffffffffffX` \x80\x84\x01\x91\x90\x91R`\x01`@\x80\x85\x01\x82\x90R`\0\x84R\x91\x83\x01\x81\x90R\x90\x82\x01R[\x84\x15a\x03pW\x84`\x01\x16`\x01\x14\x15a\x03YWa\x03V\x81\x83a\x03\xB4V[\x90P[`\x01\x85\x90\x1C\x94Pa\x03i\x82a\x05JV[\x91Pa\x03:V[`\0a\x03\x7F\x82`@\x01Qa\x06\x8DV[\x90P`\x13`\x01`\xFF\x1B\x03\x82Q\x82\x90\t\x82R`\x13`\x01`\xFF\x1B\x03\x81\x83` \x01Q\t` \x83\x01\x81\x90R\x91Q\x94P\x90\x92PPP\x91P\x91V[a\x03\xBCa\x06\xEDV[a\x03\xC4a\x07,V[`\x13`\x01`\xFF\x1B\x03\x83`@\x01Q\x85`@\x01Q\t\x81R`\x13`\x01`\xFF\x1B\x03\x81Q\x80\t` \x82\x01R`\x13`\x01`\xFF\x1B\x03\x83Q\x85Q\t`@\x82\x01R`\x13`\x01`\xFF\x1B\x03\x83` \x01Q\x85` \x01Q\t``\x82\x01R`\x13`\x01`\xFF\x1B\x03\x80\x82``\x01Q\x83`@\x01Q\t\x7FR\x03l\xEE+o\xFEs\x8C\xC7@ywy\xE8\x98\0p\nMAA\xD8\xABu\xEBM\xCA\x13Yx\xA3\t`\x80\x82\x01R`\x13`\x01`\xFF\x1B\x03\x81`\x80\x01Q`\x13`\x01`\xFF\x1B\x03\x03\x82` \x01Q\x08`\xA0\x82\x01R`\x13`\x01`\xFF\x1B\x03\x81`\x80\x01Q\x82` \x01Q\x08`\xC0\x82\x01R`\x13`\x01`\xFF\x1B\x03\x80\x82``\x01Q`\x13`\x01`\xFF\x1B\x03\x03`\x13`\x01`\xFF\x1B\x03\x80a\x04\xAEW\xFE[\x84`@\x01Q`\x13`\x01`\xFF\x1B\x03\x03`\x13`\x01`\xFF\x1B\x03\x80a\x04\xCBW\xFE[`\x13`\x01`\xFF\x1B\x03` \x8A\x01Q\x8AQ\x08`\x13`\x01`\xFF\x1B\x03` \x8C\x01Q\x8CQ\x08\t\x08\x08`\x13`\x01`\xFF\x1B\x03`\xA0\x84\x01Q\x84Q\t\t\x82R`\x13`\x01`\xFF\x1B\x03\x80\x82`@\x01Q\x83``\x01Q\x08`\x13`\x01`\xFF\x1B\x03`\xC0\x84\x01Q\x84Q\t\t` \x83\x01R`\x13`\x01`\xFF\x1B\x03\x81`\xC0\x01Q\x82`\xA0\x01Q\t`@\x83\x01RP\x92\x91PPV[a\x05Ra\x06\xEDV[a\x05Za\x07,V[`\x13`\x01`\xFF\x1B\x03` \x84\x01Q\x84Q\x08\x81R`\x13`\x01`\xFF\x1B\x03\x81Q\x80\t` \x82\x01R`\x13`\x01`\xFF\x1B\x03\x83Q\x80\t`@\x82\x01R`\x13`\x01`\xFF\x1B\x03` \x84\x01Q\x80\t``\x82\x01\x81\x90R`@\x82\x01Q`\x13`\x01`\xFF\x1B\x03\x90\x81\x03`\x80\x84\x01\x81\x90R\x90\x91\x90\x08`\xA0\x82\x01R`\x13`\x01`\xFF\x1B\x03`@\x84\x01Q\x80\t`\xE0\x82\x01R`\x13`\x01`\xFF\x1B\x03\x80\x82`\xE0\x01Q`\x02\t`\x13`\x01`\xFF\x1B\x03\x03\x82`\xA0\x01Q\x08`\xC0\x82\x01R`\x13`\x01`\xFF\x1B\x03`\xC0\x82\x01Q`\x13`\x01`\xFF\x1B\x03\x83``\x01Q`\x13`\x01`\xFF\x1B\x03\x03`\x13`\x01`\xFF\x1B\x03\x80a\x06/W\xFE[\x85`@\x01Q`\x13`\x01`\xFF\x1B\x03\x03\x86` \x01Q\x08\x08\t\x82R`\x13`\x01`\xFF\x1B\x03\x80\x82``\x01Q`\x13`\x01`\xFF\x1B\x03\x03\x83`\x80\x01Q\x08\x82`\xA0\x01Q\t` \x83\x01R`\x13`\x01`\xFF\x1B\x03\x81`\xC0\x01Q\x82`\xA0\x01Q\t`@\x83\x01RP\x91\x90PV[`\0\x80`\x02`\x13`\x01`\xFF\x1B\x03\x03\x90P`\0`\x13`\x01`\xFF\x1B\x03\x90P`@Q` \x81R` \x80\x82\x01R` `@\x82\x01R\x84``\x82\x01R\x82`\x80\x82\x01R\x81`\xA0\x82\x01R` \x81`\xC0\x83`\x05`\0\x19\xFAa\x06\xE4W`\0\x80\xFD[Q\x94\x93PPPPV[`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x808\x839P\x91\x92\x91PPV[`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x82W`\0\x80\xFD[`\x02a\x07\x95a\x07\x90\x82a\n\xF7V[a\n\xD0V[\x91P\x81\x83\x85` \x84\x02\x82\x01\x11\x15a\x07\xABW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x07\xD7W\x81a\x07\xC1\x88\x82a\x07\xE1V[\x84RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a\x07\xAEV[PPPP\x92\x91PPV[\x805a\x07\xEC\x81a\x0BkV[\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x08\x03W`\0\x80\xFD[\x815a\x08\x11a\x07\x90\x82a\x0B\x15V[\x91P\x80\x82R` \x83\x01` \x83\x01\x85\x83\x83\x01\x11\x15a\x08-W`\0\x80\xFD[a\x088\x83\x82\x84a\x0B_V[PPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x08SW`\0\x80\xFD[a\x08]``a\n\xD0V[\x90P`\0a\x08k\x84\x84a\x07\xE1V[\x82RP` a\x08|\x84\x84\x83\x01a\x07\xE1V[` \x83\x01RP`@a\x08\x90\x84\x82\x85\x01a\x07\xE1V[`@\x83\x01RP\x92\x91PPV[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a\x08\xB1W`\0\x80\xFD[`\0a\x08\xBD\x86\x86a\x07\xE1V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xDAW`\0\x80\xFD[a\x08\xE6\x86\x82\x87\x01a\x07\xF2V[\x92PP`@a\x08\xF7\x86\x82\x87\x01a\x07qV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\t\x13W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t*W`\0\x80\xFD[a\t6\x84\x82\x85\x01a\x07\xF2V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\tPW`\0\x80\xFD[`\0a\t6\x84\x84a\x07\xE1V[`\0\x80`\x80\x83\x85\x03\x12\x15a\toW`\0\x80\xFD[`\0a\t{\x85\x85a\x07\xE1V[\x92PP` a\t\x8C\x85\x82\x86\x01a\x08AV[\x91PP\x92P\x92\x90PV[`\0a\t\xA2\x83\x83a\n\rV[PP` \x01\x90V[a\t\xB3\x81a\x0B@V[a\t\xBD\x81\x84a\x0BLV[\x92Pa\t\xC8\x82a\x0B=V[\x80`\0[\x83\x81\x10\x15a\t\xF6W\x81Qa\t\xE0\x87\x82a\t\x96V[\x96Pa\t\xEB\x83a\x0BFV[\x92PP`\x01\x01a\t\xCCV[PPPPPPV[a\n\x07\x81a\x0BZV[\x82RPPV[a\n\x07\x81a\x0B=V[a\n\x07a\n\"\x82a\x0B=V[a\x0B=V[`\0a\n4`\x18\x83a\x0BQV[\x7FInvalid signature length\0\0\0\0\0\0\0\0\x81R` \x01\x92\x91PPV[`\0a\nl\x82\x86a\n\x16V[` \x82\x01\x91Pa\n|\x82\x85a\t\xAAV[`@\x82\x01\x91Pa\n\x8C\x82\x84a\n\x16V[P` \x01\x93\x92PPPV[` \x81\x01a\x07\xEC\x82\x84a\t\xFEV[`@\x81\x01a\n\xB3\x82\x85a\n\rV[a\x02|` \x83\x01\x84a\n\rV[` \x80\x82R\x81\x01a\x07\xEC\x81a\n'V[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xEFW`\0\x80\xFD[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\x0EW`\0\x80\xFD[P` \x02\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B,W`\0\x80\xFD[P` `\x1F\x91\x90\x91\x01`\x1F\x19\x16\x01\x90V[\x90V[P`\x02\x90V[` \x01\x90V[\x91\x90PV[\x90\x81R` \x01\x90V[\x15\x15\x90V[\x82\x81\x837P`\0\x91\x01RV[a\x0Bt\x81a\x0B=V[\x81\x14a\x0B\x7FW`\0\x80\xFD[PV\xFE\xA3ebzzr1X Yr\t^\x87\xEB\"\xF45\xD6\x9D\x95\xAC1\0\x81\x90\xDDX\xC5\xD4\x1C\x97\x04\t9I\xFC\x05[`-lexperimental\xF5dsolcC\0\x05\x11\0@";
    /// The deployed bytecode of the contract.
    pub static ED25519_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Ed25519<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Ed25519<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Ed25519<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Ed25519<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Ed25519<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Ed25519)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Ed25519<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ED25519_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ED25519_ABI.clone(),
                ED25519_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `scalarMult` (0x0ab72428) function
        pub fn scalar_mult(
            &self,
            s: ::ethers::core::types::U256,
            p: Point,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 183, 36, 40], (s, p))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `scalarMultBase` (0xc4f4912b) function
        pub fn scalar_mult_base(
            &self,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([196, 244, 145, 43], s)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `splitSignature` (0xa7bb5803) function
        pub fn split_signature(
            &self,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([167, 187, 88, 3], signature)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySignature` (0x750f41dc) function
        pub fn verify_signature(
            &self,
            msg_hash: [u8; 32],
            signature: ::ethers::core::types::Bytes,
            public_key: [::ethers::core::types::U256; 2],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([117, 15, 65, 220], (msg_hash, signature, public_key))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Ed25519<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `scalarMult` function with signature `scalarMult(uint256,(uint256,uint256,uint256))` and selector `0x0ab72428`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "scalarMult",
        abi = "scalarMult(uint256,(uint256,uint256,uint256))"
    )]
    pub struct ScalarMultCall {
        pub s: ::ethers::core::types::U256,
        pub p: Point,
    }
    ///Container type for all input parameters for the `scalarMultBase` function with signature `scalarMultBase(uint256)` and selector `0xc4f4912b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "scalarMultBase", abi = "scalarMultBase(uint256)")]
    pub struct ScalarMultBaseCall {
        pub s: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `splitSignature` function with signature `splitSignature(bytes)` and selector `0xa7bb5803`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "splitSignature", abi = "splitSignature(bytes)")]
    pub struct SplitSignatureCall {
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verifySignature` function with signature `verifySignature(bytes32,bytes,uint256[2])` and selector `0x750f41dc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifySignature",
        abi = "verifySignature(bytes32,bytes,uint256[2])"
    )]
    pub struct VerifySignatureCall {
        pub msg_hash: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
        pub public_key: [::ethers::core::types::U256; 2],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Ed25519Calls {
        ScalarMult(ScalarMultCall),
        ScalarMultBase(ScalarMultBaseCall),
        SplitSignature(SplitSignatureCall),
        VerifySignature(VerifySignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for Ed25519Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ScalarMultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ScalarMult(decoded));
            }
            if let Ok(decoded)
                = <ScalarMultBaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ScalarMultBase(decoded));
            }
            if let Ok(decoded)
                = <SplitSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SplitSignature(decoded));
            }
            if let Ok(decoded)
                = <VerifySignatureCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifySignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Ed25519Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ScalarMult(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScalarMultBase(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SplitSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Ed25519Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ScalarMult(element) => ::core::fmt::Display::fmt(element, f),
                Self::ScalarMultBase(element) => ::core::fmt::Display::fmt(element, f),
                Self::SplitSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ScalarMultCall> for Ed25519Calls {
        fn from(value: ScalarMultCall) -> Self {
            Self::ScalarMult(value)
        }
    }
    impl ::core::convert::From<ScalarMultBaseCall> for Ed25519Calls {
        fn from(value: ScalarMultBaseCall) -> Self {
            Self::ScalarMultBase(value)
        }
    }
    impl ::core::convert::From<SplitSignatureCall> for Ed25519Calls {
        fn from(value: SplitSignatureCall) -> Self {
            Self::SplitSignature(value)
        }
    }
    impl ::core::convert::From<VerifySignatureCall> for Ed25519Calls {
        fn from(value: VerifySignatureCall) -> Self {
            Self::VerifySignature(value)
        }
    }
    ///Container type for all return fields from the `scalarMult` function with signature `scalarMult(uint256,(uint256,uint256,uint256))` and selector `0x0ab72428`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ScalarMultReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `scalarMultBase` function with signature `scalarMultBase(uint256)` and selector `0xc4f4912b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ScalarMultBaseReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `splitSignature` function with signature `splitSignature(bytes)` and selector `0xa7bb5803`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SplitSignatureReturn {
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all return fields from the `verifySignature` function with signature `verifySignature(bytes32,bytes,uint256[2])` and selector `0x750f41dc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VerifySignatureReturn(pub bool);
    ///`Point(uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Point {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
        pub z: ::ethers::core::types::U256,
    }
}
