use ethers::{
    abi::{Error, RawLog},
    prelude::EthLogDecode,
};

use crate::pike_events::{Approval, Deposit, LiquidateBorrow, Transfer, Withdraw};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PikeEvents {
    Transfer(Transfer),
    Deposit(Deposit),
    Withdraw(Withdraw),
    Approval(Approval),
    LiquidateBorrow(LiquidateBorrow),
}

impl EthLogDecode for PikeEvents {
    fn decode_log(log: &RawLog) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let Ok(decoded) = Transfer::decode_log(log) {
            return Ok(PikeEvents::Transfer(decoded));
        }
        if let Ok(decoded) = Deposit::decode_log(log) {
            return Ok(PikeEvents::Deposit(decoded));
        }
        if let Ok(decoded) = Withdraw::decode_log(log) {
            return Ok(PikeEvents::Withdraw(decoded));
        }
        if let Ok(decoded) = Approval::decode_log(log) {
            return Ok(PikeEvents::Approval(decoded));
        }
        if let Ok(decoded) = LiquidateBorrow::decode_log(log) {
            return Ok(PikeEvents::LiquidateBorrow(decoded));
        }

        // TODO Gracefull event handling
        Err(Error::InvalidData)
    }
}
