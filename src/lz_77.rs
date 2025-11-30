#[derive(Copy, Clone, Debug)]
pub enum Token {
    Empty(u8),
    Offset { offset: u16, length: u8 },
}

fn find_longest_match(data: &[u8], pos: usize) -> Token {
    let mut best_offset = 0u16;
    let mut best_len = 0u8;
    let start = if pos > u16::MAX as usize {
        pos - u16::MAX as usize
    } else {
        0
    };

    for offset in start..pos {
        let len = matcher(data, offset, pos);
        if len > best_len {
            best_offset = (pos - (offset as usize)) as u16;
            best_len = len;
        }
    }

    if best_offset == 0 {
        return Token::Empty(data[pos]);
    }

    Token::Offset {
        offset: best_offset,
        length: best_len,
    }
}

fn matcher(data: &[u8], offset: usize, end: usize) -> u8 {
    let mut offset = offset;
    let mut pos = end;
    let mut len = 0u8;

    while offset < pos && pos < data.len() && data[offset] == data[pos] && len < 255 {
        offset += 1;
        pos += 1;
        len += 1;
    }

    return len;
}

pub fn compress(data: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        let token = find_longest_match(data, pos);
        tokens.push(token);

        println!("{:?}", token);

        match token {
            Token::Empty(_) => {
                pos += 1;
            }
            Token::Offset { offset: _, length } => {
                pos += length as usize;
            }
        }
    }

    tokens
}
