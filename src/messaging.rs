#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MessageType {
    // Client indicates their ready status
    ClientReadyStatus(ClientReadyPayload), // When a client hits "ready" in waiting round
    GameInit,
    RoundInit(RoundInitPayload), // Determine magic number, select available axioms
    PlayerTurnStart,             // Whose turn is it
    SelectionMade(SelectionMadePayload), // Player selected something
    EvaluateSelections,          // Score the round
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClientReadyPayload {
    ready: bool,
}

pub type AxiomId = u8;
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct RoundInitPayload {
    available_axioms: [AxiomId; 6],
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct SelectionMadePayload {
    selection: AxiomId,
}

impl From<MessageType> for Vec<u8> {
    fn from(value: MessageType) -> Vec<u8> {
        match value {
            MessageType::ClientReadyStatus(cl) => vec![0x00, cl.ready as u8],
            MessageType::GameInit => vec![0x01],
            MessageType::RoundInit(ro) => {
                let mut result = vec![0x02];

                for ax_id in ro.available_axioms {
                    result.push(ax_id)
                }

                result
            }
            MessageType::PlayerTurnStart => vec![0x03],
            MessageType::SelectionMade(sl) => vec![0x04, sl.selection],
            MessageType::EvaluateSelections => vec![0x05],
        }
    }
}

impl TryFrom<Vec<u8>> for MessageType {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        fn vec_to_axiom_ids(vec: Vec<u8>) -> Result<[u8; 6], ()> {
            if vec.len() != 6 {
                return Err(());
            }

            vec.try_into().or(Err(()))
        }

        let mut iter = value.into_iter();

        if let Some(byte) = iter.next() {
            return match byte {
                0x00 => {
                    let payload = iter.next().ok_or(())?;
                    let payload = match payload {
                        0 => false,
                        1 => true,
                        _ => return Err(()),
                    };
                    Ok(MessageType::ClientReadyStatus(ClientReadyPayload {
                        ready: payload,
                    }))
                }

                0x01 => Ok(MessageType::GameInit),

                0x02 => {
                    // Read the rest of the payload into struct
                    let mut avail = vec![];
                    for byte in iter {
                        avail.push(byte);
                    }

                    let slice: [u8; 6] = vec_to_axiom_ids(avail)?;

                    Ok(MessageType::RoundInit(RoundInitPayload {
                        available_axioms: slice,
                    }))
                }
                _ => {
                    unimplemented!(
                        "TODO - Finish implementing deserialization for other msg types"
                    );
                }
            };
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_bin_serialization() {
        let cl_ready: Vec<u8> =
            MessageType::ClientReadyStatus(ClientReadyPayload { ready: true }).into();
        assert_eq!(cl_ready, vec![0x00, 0x01]);

        let gm_init: Vec<u8> = MessageType::GameInit.into();
        assert_eq!(gm_init, vec![0x01]);

        let rd_init: Vec<u8> = MessageType::RoundInit(RoundInitPayload {
            available_axioms: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        })
        .into();
        assert_eq!(rd_init, vec![0x02, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06])

        // TODO - Complete
    }

    #[test]
    fn test_msg_bin_deserialization() {
        let cl_ready_0: MessageType = vec![0x00, 0x00].try_into().unwrap();
        assert_eq!(
            cl_ready_0,
            MessageType::ClientReadyStatus(ClientReadyPayload { ready: false })
        );
        let cl_ready_1: MessageType = vec![0x00, 0x01].try_into().unwrap();
        assert_eq!(
            cl_ready_1,
            MessageType::ClientReadyStatus(ClientReadyPayload { ready: true })
        );

        let gm_init: MessageType = vec![0x01].try_into().unwrap();
        assert_eq!(gm_init, MessageType::GameInit);

        let rd_init: MessageType = vec![0x02, 0x01, 0x03, 0x03, 0x05, 0x01, 0x07]
            .try_into()
            .unwrap();
        assert_eq!(
            rd_init,
            MessageType::RoundInit(RoundInitPayload {
                available_axioms: [0x01, 0x03, 0x03, 0x05, 0x01, 0x07]
            })
        );

        // TODO - Complete
    }
}
