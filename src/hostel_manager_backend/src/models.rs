use candid::{Decode, Encode};
use std::borrow;
use ic_stable_structures::StableVec;

use crate::error;

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct User(pub String);

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize, Clone)]
pub struct CustomVec<T>(pub Vec<T>);

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum RoomState {
    Full,
    PartiallyOccupied,
    TotallyVacant,
}

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize, Clone)]
pub struct Room {
    pub no: u64,
    pub capacity: u64,
    pub price_per_occupant: u64,
    pub state: RoomState,
    pub occupants: CustomVec<User>,
    pub owner: User,
}

impl Room {
    pub fn new(number: u64, capacity: u64, price_per_occupant: u64, owner: User) -> Self {
        Room {
            no: number,
            capacity,
            price_per_occupant,
            state: RoomState::TotallyVacant,
            occupants: CustomVec(vec![]),
            owner,
        }
    }

    pub fn has_occupant(&self, occupant: User) -> Option<usize> {
        self.occupants.0.iter().position(|o| o == &occupant)
    }

    pub fn add_occupant(&mut self, occupant: User) -> Result<(), error::Error> {
        if self.occupants.0.len() == self.capacity as usize {
            return Err(error::Error::RoomFull);
        }

        match self.has_occupant(occupant.clone()) {
            Some(_) => Err(error::Error::RoomAlreadyBooked),
            None => {
                self.occupants.0.push(occupant);
                self.state = if self.occupants.0.len() == self.capacity as usize {
                    RoomState::Full
                } else {
                    RoomState::PartiallyOccupied
                };
                ic_cdk::println!("occupants: {:?}", self.occupants.0);
                Ok(())
            }
        }
    }

    pub fn remove_occupant(&mut self, occupant: User) -> Result<(), error::Error> {
        match self.has_occupant(occupant) {
            Some(index) => {
                self.occupants.0.remove(index);
                Ok(())
            }
            None => Err(error::Error::NotInRoom),
        }
    }
}

impl ic_stable_structures::Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for Room {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for Room {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct GetRoomByNumberPayload {
    pub number: u64,
}

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct CreateRoomPayload {
    pub number: u64,
    pub capacity: u64,
    pub price_per_occupant: u64,
}

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct BookRoomPayload {
    pub number: u64,
    pub price: u64,
}

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct UnbookRoomPayload {
    pub number: u64,
}

#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct DeleteRoomPayload {
    pub number: u64,
}
