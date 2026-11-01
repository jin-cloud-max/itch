use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::prelude::*;

use crate::message::MessageBody;
mod message;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(n) => n,
        None => panic!("Expected argument for file path"),
    };

    let f: File = File::open(file_path)?;
    let mut reader = BufReader::new(f);

    let mut message = [0u8; 65535];
    let mut counts = [0u64; 256];
    let mut total_size: u64 = 0;
    let mut count_a = 0;

    let mut total: usize = 0;
    loop {
        if reader.fill_buf()?.is_empty() {
            break;
        }

        match read_message(&mut reader, &mut message) {
            Ok(val) => {
                total += 1;
                counts[usize::from(message[0])] += 1;
                total_size += val as u64 + 2;

                match message::decode(&message[..val]) {
                    Some(r) => {
                        match r.body {
                            MessageBody::AddOrder {
                                order_ref,
                                side,
                                shares,
                                stock,
                                price,
                            } => {
                                count_a += 1;
                                if count_a <= 5 {
                                    println!(
                                        "locate {}, ref {}, side {}, shares {}, stock {}, price {}",
                                        r.stock_locate, order_ref, side as char, shares, std::str::from_utf8(&stock).unwrap().trim_end(), price
                                    )
                                }
                            }
                            MessageBody::SystemEvent { event_code } => {
                                if total <= 5 {
                                    println!(
                                        "locate {}, tracking {}, timestamp {}, event {}",
                                        r.stock_locate, r.tracking_number, r.timestamp, event_code as char
                                    )
                                }
                            }
                        };
                    }
                    None => (),
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                println!("warn: message malformed after {total} messages");
                break;
            }
            Err(e) => return Err(e),
        }
    }

    for (kind, count) in counts.iter().enumerate() {
        if *count > 0 {
            println!("{} {}", kind as u8 as char, count)
        }
    }

    println!("total messages: {total}");
    println!("total size: {}", total_size);
    Ok(())
}

fn read_message(r: &mut BufReader<File>, message: &mut [u8; 65535]) -> std::io::Result<usize> {
    let mut size = [0u8; 2];
    r.read_exact(&mut size)?;

    let n = usize::from(u16::from_be_bytes(size));
    r.read_exact(&mut message[..n])?;

    Ok(n)
}
