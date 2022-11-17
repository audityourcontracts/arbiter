pub use iuniswapv3poolimmutables_mod::*;
#[allow(clippy::too_many_arguments)]
mod iuniswapv3poolimmutables_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IUniswapV3PoolImmutables was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IUNISWAPV3POOLIMMUTABLES_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fee\",\"outputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxLiquidityPerTick\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tickSpacing\",\"outputs\":[{\"internalType\":\"int24\",\"name\":\"\",\"type\":\"int24\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token0\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IUNISWAPV3POOLIMMUTABLES_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IUniswapV3PoolImmutables<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IUniswapV3PoolImmutables<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IUniswapV3PoolImmutables<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3PoolImmutables))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IUniswapV3PoolImmutables<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV3POOLIMMUTABLES_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                IUNISWAPV3POOLIMMUTABLES_ABI.clone(),
                IUNISWAPV3POOLIMMUTABLES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fee` (0xddca3f43) function"]
        pub fn fee(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function"]
        pub fn max_liquidity_per_tick(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tickSpacing` (0xd0c93a7c) function"]
        pub fn tick_spacing(&self) -> ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IUniswapV3PoolImmutables<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `fee`function with signature `fee()` and selector `[221, 202, 63, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    #[doc = "Container type for all input parameters for the `maxLiquidityPerTick`function with signature `maxLiquidityPerTick()` and selector `[112, 207, 117, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    #[doc = "Container type for all input parameters for the `tickSpacing`function with signature `tickSpacing()` and selector `[208, 201, 58, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    #[doc = "Container type for all input parameters for the `token0`function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1`function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV3PoolImmutablesCalls {
        Factory(FactoryCall),
        Fee(FeeCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        TickSpacing(TickSpacingCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ethers::core::abi::AbiDecode for IUniswapV3PoolImmutablesCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolImmutablesCalls::Factory(decoded));
            }
            if let Ok(decoded) = <FeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV3PoolImmutablesCalls::Fee(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidityPerTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded) =
                <TickSpacingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolImmutablesCalls::TickSpacing(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolImmutablesCalls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV3PoolImmutablesCalls::Token1(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IUniswapV3PoolImmutablesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV3PoolImmutablesCalls::Factory(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::Fee(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::TickSpacing(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::Token0(element) => element.encode(),
                IUniswapV3PoolImmutablesCalls::Token1(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV3PoolImmutablesCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV3PoolImmutablesCalls::Factory(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::Fee(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::TickSpacing(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::Token0(element) => element.fmt(f),
                IUniswapV3PoolImmutablesCalls::Token1(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FactoryCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: FactoryCall) -> Self {
            IUniswapV3PoolImmutablesCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FeeCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: FeeCall) -> Self {
            IUniswapV3PoolImmutablesCalls::Fee(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityPerTickCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: MaxLiquidityPerTickCall) -> Self {
            IUniswapV3PoolImmutablesCalls::MaxLiquidityPerTick(var)
        }
    }
    impl ::std::convert::From<TickSpacingCall> for IUniswapV3PoolImmutablesCalls {
        fn from(var: TickSpacingCall) -> Self {
            IUniswapV3PoolImmutablesCalls::TickSpacing(var)
        }
    }
    impl ::std::convert::From<Token0Call> for IUniswapV3PoolImmutablesCalls {
        fn from(var: Token0Call) -> Self {
            IUniswapV3PoolImmutablesCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for IUniswapV3PoolImmutablesCalls {
        fn from(var: Token1Call) -> Self {
            IUniswapV3PoolImmutablesCalls::Token1(var)
        }
    }
}
