fn find_longest_match(data: &[u8], pos: usize) -> (u8, u8) {
    let mut best_offset = 0u8;
    let mut best_length = 0u8;
    let start = if pos > 255 { pos - 255 } else { 0 };

    for offset in start..pos {
        let len = matcher(data, offset, pos);
        if len > best_length {
            best_offset = (pos - (offset as usize)) as u8;
            best_length = len;
        }
    }

    (best_offset, best_length)
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

    len
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut compressed: Vec<u8> = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        let (offset, length) = find_longest_match(data, pos);
        compressed.push(offset);
        if offset == 0 {
            compressed.push(data[pos]);
            pos += 1;
        } else {
            compressed.push(length);
            pos += length as usize;
        }
    }

    compressed
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decompressed: Vec<u8> = Vec::new();
    let mut pos = 0;
    while pos + 1 < data.len() {
        let header = data[pos];
        let item = data[pos + 1];
        pos += 2;
        if header == 0 {
            decompressed.push(item);
        } else {
            let start = decompressed.len() - (header as usize);
            for i in start..start + (item as usize) {
                let c = decompressed[i];
                decompressed.push(c);
            }
        }
    }
    decompressed
}
