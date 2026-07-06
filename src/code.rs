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
}
