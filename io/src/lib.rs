#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, Out, InOut};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = Out<String>;
    type Handle = InOut<ContractAction, ContractEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Contract>;
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Contract {
    pub iterating: bool,
    pub waiting: bool,
    pub iterated_by: Option<ActorId>
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum WaitingState {
    Waiting,
    NoWaiting
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractAction {
    Iterate(i128),
    IterateMax,
    ChangeStateWaitingTo(WaitingState),
    WaitAmountOfMinutes(u32),
    WaitAmountOfBlocks(u32)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractEvent {
    Iterated,
    Waited,
    IsWaiting,
    SomeOneChangeWaitingState,
    WaitingStateChanged,
    IterationInProgress,
}