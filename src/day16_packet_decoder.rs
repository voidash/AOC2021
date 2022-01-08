
#[derive(Debug)]
pub struct Packet {
    pub packet_version : u8,
    pub packet_type: u8,
    pub subpackets: Option<Box<Vec<Packet>>>,
    pub data: Option<u64>,
    pub data_len: u32
}

pub fn map_chars_to_hex(data:char) -> u8 {
    match data  {
        '0'  => { return 0b0000; },
        '1'  => { return 0b0001; },
        '2'  => { return 0b0010; },
        '3'  => { return 0b0011; },
        '4'  => { return 0b0100; },
        '5'  => { return 0b0101; },
        '6'  => { return 0b0110; },
        '7'  => { return 0b0111; },
        '8'  => { return 0b1000; },
        '9'  => { return 0b1001; },
        'A'  => { return 0b1010; },
        'B'  => { return 0b1011; },
        'C'  => { return 0b1100; },
        'D'  => { return 0b1101; },
        'E'  => { return 0b1110; },
        'F'  => { return 0b1111; },
         _ => {return 0;}
    }
}

pub fn deduce_packet(packet_data: &str) -> Packet{
    let packet_version = u8::from_str_radix(&packet_data[0..3], 2).unwrap();
    let packet_type = u8::from_str_radix(&packet_data[3..6], 2).unwrap();
    match packet_type {
        4 => {
            let mut data_vec :Vec<&str> = vec![];
            let data_to_deduce = &packet_data[6..];
                // get 5 chunks 
                let mut exit = false;
                let mut position=0;
                while !exit {
                    if &data_to_deduce[position..position+1] == "0" {
                        exit = true;
                    }
                    let data =  &data_to_deduce[position+1..position+5];
                    data_vec.push(data);
                    position += 5;
            }
            
            let d = data_vec.iter().fold(String::new(),|acc,&d| {
                format!("{}{}",acc,d)
            });
            let d_int = u64::from_str_radix(d.as_str(),2 ).unwrap();


            return Packet {
                packet_version : packet_version,
                packet_type: packet_type,
                subpackets: None,
                data: Some(d_int),
                data_len: (position-5) as u32
            };

        },
        _ => {
            let data_to_deduce = &packet_data[6..];
            // check if the third bit is 0 or 1

            let mut packet_vector = Vec::new();
            if &data_to_deduce[0..1] == "1" {
                // gather first 11 bits
                let number_of_subpackets = u32::from_str_radix(&data_to_deduce[1..12], 2).unwrap();
                let mut pos = 12;
                for _ in 0..number_of_subpackets {
                    packet_vector.push(deduce_packet(&data_to_deduce[pos..pos+11]));
                    pos+=11;
                }
            } else if &data_to_deduce[0..1] == "0" {
                let number_of_bits = u32::from_str_radix(&data_to_deduce[1..16], 2).unwrap();
                let mut pos:usize = 16;
                loop {
                    if pos as u32> number_of_bits + 16 {
                        break;
                    }
                    let pac = deduce_packet(&data_to_deduce[pos..]);
                    pos += pac.data_len as usize;
                    packet_vector.push(pac);
                }

            }


            return Packet {
                packet_version : packet_version,
                packet_type: packet_type,
                subpackets: Some(Box::new(packet_vector)),
                data: None,
                data_len: 0 
            };
            // there might be recursion
        }     }
    

    // println!("{} {}", packet_version, packet_type);

}

pub fn parse_packets_and_return_sum_of_version_numbers(val: &str) -> u32 {
 let mut binary_representation = String::new();

 for c in val.chars(){
    binary_representation = format!("{}{:04b}",binary_representation,0xf & map_chars_to_hex(c));
 }

println!("{}",binary_representation);


  0 
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_packet_type_for_id_4() {
        let d = deduce_packet("110100101111111000101000");
        assert_eq!(d.data, Some(2021));
    }

    #[test]
    fn test_subpackets_for_id_other_than4() {
        let d = deduce_packet("11101110000000001101010000001100100000100011000001100000");
        assert!(d.subpackets.is_some());
        assert_eq!(d.subpackets.as_ref().unwrap().len(), 3);

        assert_eq!(d.subpackets.as_ref().unwrap()[0].data.unwrap(),1);
        assert_eq!(d.subpackets.as_ref().unwrap()[1].data.unwrap(),2);
        assert_eq!(d.subpackets.as_ref().unwrap()[2].data.unwrap(),3);

    }


    #[test]
    fn test_for_single_packet() {
    parse_packets_and_return_sum_of_version_numbers("620080001611562C8802118E34");
    // parse_packets_and_return_sum_of_version_numbers("D2FE28");
        // assert_eq!(parse_packets_and_return_sum_of_version_numbers("620080001611562C8802118E34"), 12);
    }
}