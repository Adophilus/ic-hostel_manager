#[derive(candid::CandidType, serde::Serialize, serde::Deserialize)]
pub enum Error {
    RoomNotFound,
    RoomNotAvailable,
    RoomFull,
    RoomAlreadyBooked,
    InsufficientPrice,
    NotOwner,
    NotInRoom,
    RoomAlreadyExists,
    Overspent,
}

