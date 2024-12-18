/* mov instructions */
/**
 * MOV Rn, #data
 */
pub fn mov_rn_data(rn: u8, data: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x74 + rn);
    bytes.push(data);
    bytes
}

/**
 * MOV Rn, Rn
 */
pub fn mov_rn_rn(rn1: u8, rn2: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x7c + rn1 * 8 + rn2);
    bytes
}

/**
 * MOV A, #data
 */
pub fn mov_a_data(data: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x8c);
    bytes.push(data);
    bytes
}

/**
 * MOV A, Rn
 */
pub fn mov_a_rn(rn: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x84 + rn);
    bytes
}

/**
 * MOV Rn, A
 */
pub fn mov_rn_a(rn: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x8d + rn);
    bytes
}

/**
 * MOV A, B
 */
pub fn mov_a_b() -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0xA6);
    bytes
}

/**
 * MOV B, A
 */
pub fn mov_b_a() -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0xA7);
    bytes
}

/* add instructions */

/**
 * ADD A, #data
 */
pub fn add_a_data(data: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x50);
    bytes.push(data);
    bytes
}

/**
 * ADD A, B
 */
pub fn add_a_b() -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x51);
    bytes
}

/**
 * ADD A, Rn
 */
pub fn add_a_rn(rn: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x52 + rn);
    bytes
}
