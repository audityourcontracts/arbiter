pub use uniswap_v2_migrator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod uniswap_v2_migrator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factoryV1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_router"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("migrate"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("migrate"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountTokenMin"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountETHMin"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("to"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("deadline"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                    ],
                    outputs: ::std::vec![],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV2MIGRATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0VWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[P`@Qa\x0C\xD68\x03\x80a\x0C\xD6\x839\x81\x81\x01`@R`@\x81\x10\x15a\0\xABWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x91\x1B\x16`\xA0R`\x80Q``\x1C`\xA0Q``\x1Ca\x0B\xD6a\x01\0`\09\x80a\x05pR\x80a\x05\xE0R\x80a\x06\xDBRP\x80a\x01HRPa\x0B\xD6`\0\xF3\xFE`\x80`@R`\x046\x10a\0\"W`\x005`\xE0\x1C\x80c\xB7\xDF\x1D%\x14a\0{Wa\0)V[6a\0)W\0[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FUnknown signature and no fallbac`D\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\0\xCDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01B`\x04\x806\x03`\xA0\x81\x10\x15a\x01\x16WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x90\x91\x16\x90`\x80\x015a\x01DV[\0[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x06\xF2\xBFb\x87`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x01\xF3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x02\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02OWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x84\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x02\xD4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x02\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x030WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a\x03\xC2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x03\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x04\x1EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQa\x04hW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Qc|E\xF8\xAD`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`$\x82\x01\x81\x90R`D\x82\x01R`\0\x19`d\x82\x01R\x81Q`\0\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x87\x16\x92c\xF8\x8B\xF1Z\x92`\x84\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x87\x87\x80;\x15\x80\x15a\x05\0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x05\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`@\x81\x10\x15a\x05\\WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90Pa\x05\x95\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x074V[`@\x80Qc\xF3\x05\xD7\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`D\x82\x01\x8B\x90R`d\x82\x01\x8A\x90R\x88\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x88\x90R\x91Q`\0\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91c\xF3\x05\xD7\x19\x91\x87\x91`\xC4\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x85\x88\x80;\x15\x80\x15a\x06bWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x06vW=`\0\x80>=`\0\xFD[PPPPP`@Q=``\x81\x10\x15a\x06\xBFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90P\x81\x83\x11\x15a\x07\x13Wa\x07\x01\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0a\x074V[a\x07\x0E\x8B3\x84\x86\x03a\x08\xD0V[a\x07'V[\x80\x84\x11\x15a\x07'Wa\x07'3\x82\x86\x03a\neV[PPPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x07\xB1W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x07\x92V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08\x13W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x18V[``\x91P[P\x91P\x91P\x81\x80\x15a\x08xWP\x80Q\x15\x80a\x08xWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x08uWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\x08\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FTransferHelper: APPROVE_FAILED\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\tMW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t.V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xB4V[``\x91P[P\x91P\x91P\x81\x80\x15a\n\x14WP\x80Q\x15\x80a\n\x14WP\x80\x80` \x01\x90Q` \x81\x10\x15a\n\x11WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\x08\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FTransferHelper: TRANSFER_FAILED\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\n\xB1W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\n\x92V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\x13W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\x18V[``\x91P[PP\x90P\x80a\x0BXW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80a\x0B~`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPV\xFETarget contract does not containTransferHelper: ETH_TRANSFER_FAILED\xA2dipfsX\"\x12 G>g\xBE\xA4\xD0\x95\xA6\x17q\r\xF5Y\xA4\x8B\xAD\x0B\xAD\x82\xCE\r\xCA\x01\xF3\x95\x83\xFD\xB8\\\x9E\x9D\x90dsolcC\0\x06\x06\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV2MIGRATOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\"W`\x005`\xE0\x1C\x80c\xB7\xDF\x1D%\x14a\0{Wa\0)V[6a\0)W\0[bF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`)`$R\x7FUnknown signature and no fallbac`D\x90\x81Rh\x1A\xC8\x19\x19Y\x9A[\x99Y`\xBA\x1B`dR\x90`\x84\x90\xFD[4\x80\x15a\0\xCDWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\"`$R\x7FEther sent to non-payable functi`D\x90\x81Ra7\xB7`\xF1\x1B`dR\x90`\x84\x90\xFD[Pa\x01B`\x04\x806\x03`\xA0\x81\x10\x15a\x01\x16WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x91`@\x82\x015\x91``\x81\x015\x90\x91\x16\x90`\x80\x015a\x01DV[\0[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x06\xF2\xBFb\x87`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x01\xF3WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x02\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02OWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90Q\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x84\x16\x91cp\xA0\x821\x91`$\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x02\xD4WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xFA\x15\x80\x15a\x02\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x030WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ`@\x80Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x83\x90R\x90Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x84\x16\x91c#\xB8r\xDD\x91`d\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81`\0\x87\x80;\x15\x80\x15a\x03\xC2WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x03\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x04\x1EWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQa\x04hW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Qc|E\xF8\xAD`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`$\x82\x01\x81\x90R`D\x82\x01R`\0\x19`d\x82\x01R\x81Q`\0\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x87\x16\x92c\xF8\x8B\xF1Z\x92`\x84\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x87\x87\x80;\x15\x80\x15a\x05\0WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x05\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`@\x81\x10\x15a\x05\\WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90Pa\x05\x95\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x074V[`@\x80Qc\xF3\x05\xD7\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`D\x82\x01\x8B\x90R`d\x82\x01\x8A\x90R\x88\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x88\x90R\x91Q`\0\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91c\xF3\x05\xD7\x19\x91\x87\x91`\xC4\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x85\x88\x80;\x15\x80\x15a\x06bWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`%`$R`\0\x80Q` a\x0B^\x839\x81Q\x91R`D\x90\x81Rd code`\xD8\x1B`dR\x90`\x84\x90\xFD[PZ\xF1\x15\x80\x15a\x06vW=`\0\x80>=`\0\xFD[PPPPP`@Q=``\x81\x10\x15a\x06\xBFWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[P\x80Q` \x90\x91\x01Q\x90\x92P\x90P\x81\x83\x11\x15a\x07\x13Wa\x07\x01\x8B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0a\x074V[a\x07\x0E\x8B3\x84\x86\x03a\x08\xD0V[a\x07'V[\x80\x84\x11\x15a\x07'Wa\x07'3\x82\x86\x03a\neV[PPPPPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x07\xB1W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x07\x92V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08\x13W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x18V[``\x91P[P\x91P\x91P\x81\x80\x15a\x08xWP\x80Q\x15\x80a\x08xWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x08uWbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\x08\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FTransferHelper: APPROVE_FAILED\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94``\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\tMW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t.V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xB4V[``\x91P[P\x91P\x91P\x81\x80\x15a\n\x14WP\x80Q\x15\x80a\n\x14WP\x80\x80` \x01\x90Q` \x81\x10\x15a\n\x11WbF\x1B\xCD`\xE5\x1B`\0\x90\x81R` `\x04R`\x12`$Rq\x10\xD8[\x1B\x19\x18]\x18H\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`r\x1B`D\x90\x81R\x90`d\x90\xFD[PQ[a\x08\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FTransferHelper: TRANSFER_FAILED\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\n\xB1W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\n\x92V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0B\x13W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\x18V[``\x91P[PP\x90P\x80a\x0BXW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80a\x0B~`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[PPPV\xFETarget contract does not containTransferHelper: ETH_TRANSFER_FAILED\xA2dipfsX\"\x12 G>g\xBE\xA4\xD0\x95\xA6\x17q\r\xF5Y\xA4\x8B\xAD\x0B\xAD\x82\xCE\r\xCA\x01\xF3\x95\x83\xFD\xB8\\\x9E\x9D\x90dsolcC\0\x06\x06\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV2MIGRATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct UniswapV2Migrator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV2Migrator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV2Migrator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV2Migrator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV2Migrator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV2Migrator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV2Migrator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UNISWAPV2MIGRATOR_ABI.clone(),
                client,
            ))
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
                UNISWAPV2MIGRATOR_ABI.clone(),
                UNISWAPV2MIGRATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `migrate` (0xb7df1d25) function
        pub fn migrate(
            &self,
            token: ::ethers::core::types::Address,
            amount_token_min: ::ethers::core::types::U256,
            amount_eth_min: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 223, 29, 37],
                    (token, amount_token_min, amount_eth_min, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UniswapV2Migrator<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `migrate` function with signature `migrate(address,uint256,uint256,address,uint256)` and selector `0xb7df1d25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "migrate",
        abi = "migrate(address,uint256,uint256,address,uint256)"
    )]
    pub struct MigrateCall {
        pub token: ::ethers::core::types::Address,
        pub amount_token_min: ::ethers::core::types::U256,
        pub amount_eth_min: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
    }
}
