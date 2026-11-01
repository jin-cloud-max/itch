pub struct Message {
    pub stock_locate: u16,
    pub tracking_number: u16,
    pub timestamp: u64,
    pub body: MessageBody,
}

pub enum MessageBody {
    AddOrder {
        order_ref: u64,
        side: u8,
        shares: u32,
        stock: [u8; 8],
        price: u32,
    },
    SystemEvent {
        event_code: u8,
    },
}

pub fn decode(bytes: &[u8]) -> Option<Message> {
    let locate = u16::from_be_bytes(bytes[1..3].try_into().unwrap());

    let mut timestamp_bytes = [0u8; 8];
    timestamp_bytes[2..8].copy_from_slice(&bytes[5..11]);
    let timestamp = u64::from_be_bytes(timestamp_bytes);
    let tracking_number = u16::from_be_bytes(bytes[3..5].try_into().unwrap());

    let body = match bytes[0] {
        b'A' => MessageBody::AddOrder {
            order_ref: u64::from_be_bytes(bytes[11..19].try_into().unwrap()),
            side: bytes[19],
            shares: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
            stock: bytes[24..32].try_into().unwrap(),
            price: u32::from_be_bytes(bytes[32..36].try_into().unwrap()),
        },
        b'S' => MessageBody::SystemEvent {
            event_code: bytes[11],
        },
        _ => return None,
    };

    Some(Message {
        stock_locate: locate,
        tracking_number,
        timestamp,
        body,
    })
}
