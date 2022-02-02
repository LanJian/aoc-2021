use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Transmission {
    bits: Vec<u64>,
    len: usize,
}

impl FromStr for Transmission {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shorts = s
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16))
            .collect::<Result<Vec<u8>, ParseIntError>>()?;

        let bits: Vec<u64> = shorts
            .chunks(16)
            .map(|chunk| {
                chunk
                    .iter()
                    .rfold(0_u64, |a, e| a >> 4 | ((*e as u64) << 60))
            })
            .collect();

        Ok(Self {
            bits,
            len: shorts.len() * 4,
        })
    }
}

impl Transmission {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn at(&self, index: usize) -> Option<u16> {
        self.slice(index, 1)
    }

    pub fn slice(&self, index: usize, len: usize) -> Option<u16> {
        let end = index + len; // exclusive

        if end > self.len() || len == 0 || len > 16 {
            return None;
        }

        let start_i = index / 64;
        let start_j = index % 64;
        let start_chunk = self.bits[start_i];

        if start_j + len <= 64 {
            return Some((start_chunk << start_j >> (64 - len)) as u16);
        }

        // end index is exclusive
        let end_i = end / 64;
        let end_j = end % 64;
        let end_chunk = self.bits[end_i];

        Some(((start_chunk << start_j >> start_j << end_j) | (end_chunk >> (64 - end_j))) as u16)
    }

    pub fn parse_packet(&self, index: &mut usize) -> Result<Packet, String> {
        let version = self
            .slice(*index, 3)
            .ok_or_else(|| "could not parse version")? as u8;
        *index += 3;

        let type_id = self
            .slice(*index, 3)
            .ok_or_else(|| "could not parse type id")? as u8;
        *index += 3;

        let payload = match type_id {
            4 => PacketPayload::Literal(self.parse_literal(index)),
            _ => PacketPayload::Operator(self.parse_operator(index)?),
        };

        Ok(Packet {
            version,
            type_id,
            payload,
        })
    }

    fn parse_operator(&self, index: &mut usize) -> Result<OperatorPayload, String> {
        let length_type_id = self.at(*index);
        *index += 1;

        let mut subpackets: Vec<Packet> = Vec::default();
        match length_type_id {
            Some(0) => {
                // length is total length in bits
                let length = self
                    .slice(*index, 15)
                    .ok_or_else(|| "could not parse type 0 length")?
                    as usize;
                *index += 15;

                let end = *index + length;
                while *index < end {
                    let subpacket = self.parse_packet(index)?;
                    subpackets.push(subpacket);
                }
            }
            Some(1) => {
                // length is number of subpackets
                let length = self
                    .slice(*index, 11)
                    .ok_or_else(|| "could not parse type 1 length")?
                    as usize;
                *index += 11;

                for _ in 0..length {
                    let subpacket = self.parse_packet(index)?;
                    subpackets.push(subpacket);
                }
            }
            _ => return Err("invalid length type id".to_string()),
        };

        Ok(OperatorPayload {
            packets: subpackets,
        })
    }

    fn parse_literal(&self, index: &mut usize) -> LiteralPayload {
        let mut number = 0_u64;

        while let Some(chunk) = self.slice(*index, 5) {
            *index += 5;
            number = number << 4 | ((chunk & 0b1111) as u64);
            if chunk >> 4 == 0 {
                break;
            }
        }

        LiteralPayload { number }
    }
}

pub struct Packet {
    version: u8,
    type_id: u8,
    payload: PacketPayload,
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        match &self.payload {
            PacketPayload::Operator(OperatorPayload { packets }) => {
                packets.iter().map(|x| x.version_sum()).sum::<usize>() + (self.version as usize)
            }
            PacketPayload::Literal(_) => self.version as usize,
        }
    }

    pub fn eval(&self) -> Result<u64, String> {
        match &self.payload {
            PacketPayload::Literal(LiteralPayload { number }) => Ok(*number),
            PacketPayload::Operator(OperatorPayload { packets }) => {
                let terms = packets
                    .iter()
                    .map(|x| x.eval())
                    .collect::<Result<Vec<u64>, String>>()?;
                match &self.type_id {
                    0 => Ok(terms.iter().sum()),
                    1 => Ok(terms.iter().product()),
                    2 => match terms.iter().min() {
                        Some(min) => Ok(*min),
                        None => Err("not enough terms for min".to_string()),
                    },
                    3 => match terms.iter().max() {
                        Some(max) => Ok(*max),
                        None => Err("not enough terms for max".to_string()),
                    },
                    5 => Ok((terms[0] > terms[1]) as u64),
                    6 => Ok((terms[0] < terms[1]) as u64),
                    7 => Ok((terms[0] == terms[1]) as u64),
                    _ => Err("invalid operator type id".to_string()),
                }
            }
        }
    }
}

pub enum PacketPayload {
    Operator(OperatorPayload),
    Literal(LiteralPayload),
}

pub struct OperatorPayload {
    packets: Vec<Packet>,
}

pub struct LiteralPayload {
    number: u64,
}

pub fn parse_input(lines: Vec<String>) -> Result<Transmission, String> {
    if lines.len() != 1 {
        return Err("invalid input".to_string());
    }

    Transmission::from_str(&lines[0]).or_else(|_| Err("could not parse input".to_string()))
}

pub fn part_one(transmission: &Transmission) -> usize {
    let packet = transmission
        .parse_packet(&mut 0)
        .expect("could not parse packet");
    packet.version_sum()
}

pub fn part_two(transmission: &Transmission) -> u64 {
    let packet = transmission
        .parse_packet(&mut 0)
        .expect("could not parse packet");
    packet.eval().expect("could not eval the transmission")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        assert_eq!(
            parse_input(vec!["D2FE28".to_string()]).expect("could not parse input"),
            Transmission {
                bits: vec![0b1101001011111110001010000000000000000000000000000000000000000000_u64],
                len: 24
            }
        );
        assert_eq!(
            parse_input(vec!["38006F45291200".to_string()]).expect("could not parse input"),
            Transmission {
                bits: vec![0b0011100000000000011011110100010100101001000100100000000000000000_u64],
                len: 56
            }
        );
        assert_eq!(
            parse_input(vec!["A0016C880162017C3686B18A3D4780".to_string()])
                .expect("could not parse input"),
            Transmission {
                bits: vec![
                    0b1010000000000001011011001000100000000001011000100000000101111100_u64,
                    0b0011011010000110101100011000101000111101010001111000000000000000_u64
                ],
                len: 120
            }
        );
    }

    #[test]
    fn part_one_test() {
        let mut trans = parse_input(vec!["D2FE28".to_string()]).expect("could not parse input");
        assert_eq!(part_one(&trans), 6);

        trans = parse_input(vec!["38006F45291200".to_string()]).expect("could not parse input");
        assert_eq!(part_one(&trans), 9);

        trans = parse_input(vec!["EE00D40C823060".to_string()]).expect("could not parse input");
        assert_eq!(part_one(&trans), 14);

        trans = parse_input(vec!["8A004A801A8002F478".to_string()]).expect("could not parse input");
        assert_eq!(part_one(&trans), 16);

        trans = parse_input(vec!["620080001611562C8802118E34".to_string()])
            .expect("could not parse input");
        assert_eq!(part_one(&trans), 12);

        trans = parse_input(vec!["C0015000016115A2E0802F182340".to_string()])
            .expect("could not parse input");
        assert_eq!(part_one(&trans), 23);

        trans = parse_input(vec!["A0016C880162017C3686B18A3D4780".to_string()])
            .expect("could not parse input");
        assert_eq!(part_one(&trans), 31);
    }

    #[test]
    fn part_two_test() {
        let mut trans = parse_input(vec!["D2FE28".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 2021);

        trans = parse_input(vec!["C200B40A82".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 3);

        trans = parse_input(vec!["04005AC33890".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 54);

        trans = parse_input(vec!["880086C3E88112".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 7);

        trans = parse_input(vec!["CE00C43D881120".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 9);

        trans = parse_input(vec!["D8005AC2A8F0".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 1);

        trans = parse_input(vec!["F600BC2D8F".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 0);

        trans = parse_input(vec!["9C005AC2F8F0".to_string()]).expect("could not parse input");
        assert_eq!(part_two(&trans), 0);

        trans = parse_input(vec!["9C0141080250320F1802104A08".to_string()])
            .expect("could not parse input");
        assert_eq!(part_two(&trans), 1);
    }

    #[test]
    fn at_test() {
        let trans = parse_input(vec!["A0016C880162017C3686B18A3D4780".to_string()])
            .expect("could not parse input");
        assert_eq!(trans.at(0), Some(1));
        assert_eq!(trans.at(1), Some(0));
        assert_eq!(trans.at(63), Some(0));
        assert_eq!(trans.at(64), Some(0));
        assert_eq!(trans.at(66), Some(1));
        assert_eq!(trans.at(119), Some(0));
        assert_eq!(trans.at(120), None);
    }

    #[test]
    fn slice_test() {
        let trans = parse_input(vec!["A0016C880162017C3686B18A3D4780".to_string()])
            .expect("could not parse input");
        assert_eq!(trans.slice(0, 5), Some(0b10100));
        assert_eq!(trans.slice(10, 10), Some(0b0000010110));
        assert_eq!(trans.slice(60, 4), Some(0b1100));
        assert_eq!(trans.slice(63, 1), Some(0b0));
        assert_eq!(trans.slice(64, 1), Some(0b0));
        assert_eq!(trans.slice(0, 1), Some(0b1));
        assert_eq!(trans.slice(60, 10), Some(0b1100001101));
        assert_eq!(trans.slice(70, 5), Some(0b10100));
        assert_eq!(trans.slice(63, 2), Some(0b00));
        assert_eq!(trans.slice(0, 16), Some(0b1010000000000001));
        assert_eq!(trans.slice(110, 5), Some(0b11100));
        assert_eq!(trans.slice(110, 10), Some(0b1110000000));
        assert_eq!(trans.slice(0, 17), None);
        assert_eq!(trans.slice(0, 0), None);
        assert_eq!(trans.slice(5, 0), None);
        assert_eq!(trans.slice(120, 1), None);
        assert_eq!(trans.slice(110, 11), None);
    }
}
