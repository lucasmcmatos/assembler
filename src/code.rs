pub fn dest(dest: Option<&str>) -> &'static str {
    match dest {
        None => "000",
        Some("M") => "001",
        Some("D") => "010",
        Some("MD") => "011",
        Some("A") => "100",
        Some("AM") => "101",
        Some("AD") => "110",
        Some("AMD") => "111",
        Some(other) => panic!("dest inválido: {other}"),
    }
}

pub fn comp(comp: &str) -> &'static str {
    match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        other => panic!("comp inválido: {other}"),
    }
}

pub fn jump(jump: Option<&str>) -> &'static str {
    match jump {
        None => "000",
        Some("JGT") => "001",
        Some("JEQ") => "010",
        Some("JGE") => "011",
        Some("JLT") => "100",
        Some("JNE") => "101",
        Some("JLE") => "110",
        Some("JMP") => "111",
        Some(other) => panic!("jump inválido: {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_all_dest_combinations() {
        assert_eq!(dest(None), "000");
        assert_eq!(dest(Some("M")), "001");
        assert_eq!(dest(Some("D")), "010");
        assert_eq!(dest(Some("MD")), "011");
        assert_eq!(dest(Some("A")), "100");
        assert_eq!(dest(Some("AM")), "101");
        assert_eq!(dest(Some("AD")), "110");
        assert_eq!(dest(Some("AMD")), "111");
    }

    #[test]
    fn encodes_all_jump_combinations() {
        assert_eq!(jump(None), "000");
        assert_eq!(jump(Some("JGT")), "001");
        assert_eq!(jump(Some("JEQ")), "010");
        assert_eq!(jump(Some("JGE")), "011");
        assert_eq!(jump(Some("JLT")), "100");
        assert_eq!(jump(Some("JNE")), "101");
        assert_eq!(jump(Some("JLE")), "110");
        assert_eq!(jump(Some("JMP")), "111");
    }

    #[test]
    #[should_panic(expected = "dest inválido")]
    fn panics_on_invalid_dest() {
        dest(Some("XYZ"));
    }

    #[test]
    #[should_panic(expected = "jump inválido")]
    fn panics_on_invalid_jump() {
        jump(Some("XYZ"));
    }

    #[test]
    fn encodes_constants_and_d_with_a_equal_zero() {
        assert_eq!(comp("0"), "0101010");
        assert_eq!(comp("1"), "0111111");
        assert_eq!(comp("-1"), "0111010");
        assert_eq!(comp("D"), "0001100");
        assert_eq!(comp("D+1"), "0011111");
        assert_eq!(comp("D-1"), "0001110");
    }

    #[test]
    fn encodes_a_variants_with_a_bit_zero() {
        assert_eq!(comp("A"), "0110000");
        assert_eq!(comp("!A"), "0110001");
        assert_eq!(comp("-A"), "0110011");
        assert_eq!(comp("A+1"), "0110111");
        assert_eq!(comp("A-1"), "0110010");
        assert_eq!(comp("D+A"), "0000010");
        assert_eq!(comp("D-A"), "0010011");
        assert_eq!(comp("A-D"), "0000111");
        assert_eq!(comp("D&A"), "0000000");
        assert_eq!(comp("D|A"), "0010101");
        assert_eq!(comp("!D"), "0001101");
    }

    #[test]
    fn encodes_m_variants_with_a_bit_one() {
        assert_eq!(comp("M"), "1110000");
        assert_eq!(comp("!M"), "1110001");
        assert_eq!(comp("-M"), "1110011");
        assert_eq!(comp("M+1"), "1110111");
        assert_eq!(comp("M-1"), "1110010");
        assert_eq!(comp("D+M"), "1000010");
        assert_eq!(comp("D-M"), "1010011");
        assert_eq!(comp("M-D"), "1000111");
        assert_eq!(comp("D&M"), "1000000");
        assert_eq!(comp("D|M"), "1010101");
    }

    #[test]
    #[should_panic(expected = "comp inválido")]
    fn panics_on_invalid_comp() {
        comp("XYZ");
    }
}
