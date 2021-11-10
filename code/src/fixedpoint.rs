// Tests für eigene Fixed Point Operationen

const ZERO: [u8;32] = [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
    0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];

pub fn test_alu(){

    let one = [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,1,
        0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
    let two = [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,1,0,
        0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];

    assert_eq!(alu(one, one, 0).0, two);
}

fn alu(num1: [u8; 32], num2: [u8; 32], sub_flag: u8) -> ([u8; 32], u8, u8, u8, u8){

    let mut result = ZERO;
    let mut carry = 0;
    // Erstmal die Flags ignorieren
    for i in (0..result.len()).rev(){
        // von hinten nach vorne
        let sum = num1[i] + num2[i] + carry;

        if sum == 0 || sum == 1{
            result[i] = sum;
            carry = 0;
        }else if sum == 2{
            result[i] = 0;
            carry = 1;
        }else { // sum == 3
            result[i] = 1;
            carry = 1;
        }
    }

    return (result, 0, 0, 0, 0);
}

// wird in diesem Fall schon gescaled
fn mul(num1: &[u8], num2: &[u8]) -> [u8; 32]{
    return ZERO;
}

// gibt 32 bit vorkomma und 32 bit rest aus
fn div(num1: &[u8], num2: &[u8]) -> ([u8; 32], [u8; 32]){
    return (ZERO, ZERO);
}