#![no_std]
use gstd::{prelude::*, msg, exec};
use paralel_contract_io::*;

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe {
        CONTRACT = Some(Contract {
            iterating: false,
            waiting: false,
            iterated_by: None
        });
    };
    msg::reply(String::from("Contract started!"), 0)
        .expect("Error in sending init reply");
}

#[gstd::async_main]
async fn main() {
    let message: ContractAction = msg::load()
        .expect("Error laoding message");

    let state = state_mut();

    if state.iterating {
        msg::reply(ContractEvent::IterationInProgress, 0)
            .expect("Error sending reply");
        return;
    }
    
    match message {
        // will do the number of iterations indicated
        ContractAction::Iterate(times) => {
            state.iterating = true;
            for i in 0..times {}
            state.iterated_by = Some(msg::source());
            state.iterating = false;
            msg::reply(ContractEvent::Iterated, 0)
                .expect("Error sending reply");
        },
        // will do the the maximum iterations (max value in i128)
        ContractAction::IterateMax => {
            state.iterating = true;
            for i in 0..17014118346046923173168730371588410572i128 {}
            state.iterated_by = Some(msg::source());
            state.iterating = false;
            msg::reply(ContractEvent::Iterated, 0)
                .expect("Error sending reply");
        },
        // will wait the indicated minutes
        ContractAction::WaitAmountOfMinutes(minutes) => {
            if state.waiting {
                msg::reply(ContractEvent::IsWaiting, 0) 
                    .expect("Error sending reply");
                return;
            }

            let minutes = minutes * 20;
            state.waiting = true;
            exec::sleep_for(minutes).await;
            if !state.waiting {
                msg::reply(ContractEvent::SomeOneChangeWaitingState, 0)
                    .expect("Error sending reply");
                return;
            }

            state.waiting = false;

            msg::reply(ContractEvent::Waited, 0)
                .expect("Error sending reply");
        },
        // will wait the blocks minutes
        ContractAction::WaitAmountOfBlocks(blocks) => {
            if state.waiting {
                msg::reply(ContractEvent::IsWaiting, 0) 
                    .expect("Error sending reply");
                return;
            }

            state.waiting = true;
            
            exec::sleep_for(blocks).await;

            if !state.waiting {
                msg::reply(ContractEvent::SomeOneChangeWaitingState, 0)
                    .expect("Error sending reply");
                return;
            }

            state.waiting = false;

            msg::reply(ContractEvent::Waited, 0)
                .expect("Error sending reply");
        },
        // Change the state for waiting field
        ContractAction::ChangeStateWaitingTo(change) => {
            match change {
                WaitingState::Waiting => state.waiting = true,
                WaitingState::NoWaiting => state.waiting = false
            }

            msg::reply(ContractEvent::WaitingStateChanged, 0) 
                .expect("Error sending reply");
        } 
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(state_mut(), 0)
        .expect("Error sending state");
}

fn state_mut() -> &'static mut Contract {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}