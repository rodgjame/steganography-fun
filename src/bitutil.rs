pub struct BitUtils {}

impl BitUtils {
    // Transforms a decimal represented byte into its bit representation
    // For example 4 would be [0,0,0,0,0,1,0,0]
    pub fn byte_to_bit(byte: u8) -> Vec<u8> {
        (0..8)
            .rev() // Itererates from 7 to 0
            .map(|i| (byte >> i) & 1) // Gets the bit in the current position
            .collect() // Collects the results
    }

    // Transforms a bit into a decimal represented byte
    pub fn byte_u32_to_bit(byte: u32) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();
        for i in (0..32).rev() {
            let bit = (byte >> i) & 1;
            bits.push(bit as u8);
        }
        bits
    }

    // Transforms a byte in its bit form into its decimal representation
    pub fn byte_to_decimal(byte: Vec<u8>) -> u8 {
        let mut output: u8 = 0;
        for i in 0..8 {
            if byte[i] == 1 {
                output += 2u8.pow(7 - i as u32) as u8;
            }
        }
        output
    }

    // Transforms 4 bytes in its bit form into its decimal representation
    pub fn byte_u32_to_decimal(byte: Vec<u8>) -> u32 {
        // Create an iterator over the bits and their positions
        let bit_positions = byte.iter().enumerate();
    
        // Filter out the bits that are set to 1
        let set_bits = bit_positions.filter(|(_, &bit)| bit == 1);
    
        // Calculate the decimal value by summing 2^(31 - position) for each set bit
        let decimal_value = set_bits.fold(0u32, |acc, (position, _)| {
            acc + 2u32.pow(31 - position as u32)
        });
    
        decimal_value
    }    

    // Reads the least significant bit (LSB) from a byte array
    pub fn read_lsb(bytes: Vec<u8>) -> Vec<u8> {
        bytes
            .iter()
            .map(|byte| byte % 2)
            .collect()
    }

    // Takes bits and transforms them into bytes
    pub fn bits_to_bytes(bits: Vec<u8>) -> Vec<u8> {
        // create an empty vector to store the resulting bytes
        let mut output: Vec<u8> = Vec::new();

        // iterate over the bits in chunks of 8 and convert them to bytes
        for byte in bits.chunks(8) {
            if byte.len() == 8 {
                output.push(Self::byte_to_decimal(byte.to_vec()));
            }
        }
        output
    }

    // Take in bytes and transform them into a bit array
    // For example a vector like [4, 8] would be [0,0,0,0,0,1,0,0,0,0,0,0,1,0,0,0]
    pub fn make_bits(bytes: Vec<u8>) -> Vec<u8> {
        bytes
            .iter() // Iterates over every byte 
            .flat_map(|byte| Self::byte_to_bit(*byte)) // Transforms the current byte into a bit
            .collect() // Collects the results
    }
}