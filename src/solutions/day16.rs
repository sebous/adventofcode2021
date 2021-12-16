use crate::lib::input::load_input;

fn hex_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("wrong hex char"),
    }
}

fn bin_to_dec(str: &str) -> usize {
    usize::from_str_radix(str, 2).unwrap()
}

struct PacketReader {
    binary: String,
    i: usize,
}

impl PacketReader {
    fn take(&mut self, count: usize) -> &str {
        let slice = &self.binary[self.i..self.i + count];
        self.i += count;
        slice
    }

    fn take_n_dec(&mut self, count: usize) -> usize {
        let slice = self.take(count);
        bin_to_dec(slice)
    }

    fn bit(&mut self) -> bool {
        match self.take(1) {
            "1" => true,
            "0" => false,
            _ => unimplemented!(),
        }
    }
}

fn process(reader: &mut PacketReader) -> (u64, u64) {
    let version = reader.take_n_dec(3);
    let type_id = reader.take_n_dec(3);

    if type_id == 4 {
        let mut b_value = String::new();
        loop {
            let last = !reader.bit();
            b_value.push_str(reader.take(4));
            if last {
                break;
            }
        }
        return (version as u64, bin_to_dec(b_value.as_str()) as u64);
    }

    let results = if reader.bit() {
        // number of sub-packets immediately contained
        let subpackets_count = reader.take_n_dec(11);
        (0..subpackets_count).map(|_| process(reader)).collect()
    } else {
        // total length in bits
        let subpackets_len = reader.take_n_dec(15);
        let target_i = reader.i + subpackets_len;
        let mut results = vec![];
        while reader.i != target_i {
            results.push(process(reader));
        }
        results
    };

    let mut values = results.iter().map(|x| x.1);
    let result = match type_id {
        0 => values.sum(),
        1 => values.product(),
        2 => values.min().unwrap(),
        3 => values.max().unwrap(),
        5 => (values.next().unwrap() > values.next().unwrap()) as u64,
        6 => (values.next().unwrap() < values.next().unwrap()) as u64,
        7 => (values.next().unwrap() == values.next().unwrap()) as u64,
        _ => panic!("invalid type id"),
    };

    let version_sum: u64 = results.iter().map(|x| x.0).sum();
    (version_sum + version as u64, result)
}

pub fn run() {
    let input = load_input("16");
    let p1 = process(&mut PacketReader {
        binary: input.clone().chars().map(|c| hex_to_bin(c)).collect(),
        i: 0,
    });

    println!("{:?}", p1)
}
