use std::collections::HashMap;

use crate::utils::read_lines;

struct Devices(Box<[(Name, Connections)]>);

impl Devices {
    fn parse(iter: impl Iterator<Item = String>) -> Self {
        let mut data: Box<_> = iter
            .map(|l| {
                let (name, connections) = l.split_once(": ").unwrap();
                (
                    Name::new(name),
                    connections.split(' ').fold(Connections::new(), |mut a, i| {
                        a.push(Name::new(i));
                        a
                    }),
                )
            })
            .collect();
        data.sort_unstable_by_key(|e| e.0);
        Self(data)
    }

    fn get(&self, name: Name) -> Option<&Connections> {
        self.0.binary_search_by_key(&name, |e| e.0).ok().map(|n| &self.0[n].1)
    }

    fn count_paths(&self, start: Name, end: Name, memo: &mut HashMap<Name, u64>) -> u64 {
        if start == end {
            return 1;
        }
        if let Some(&result) = memo.get(&start) {
            return result;
        }
        let result = self
            .get(start)
            .map_or_default(|c| c.as_slice().iter().map(|&n| self.count_paths(n, end, memo)).sum());
        memo.insert(start, result);
        result
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Name(u32);

impl Name {
    fn new(s: &str) -> Self {
        let b = s.as_bytes();
        Self((u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]))
    }
}

struct Connections {
    count: usize,
    data: [Name; 32],
}

impl Connections {
    fn new() -> Self { Self { count: 0, data: [Name(0); 32] } }

    fn push(&mut self, name: Name) {
        self.data[self.count] = name;
        self.count += 1;
    }

    fn as_slice(&self) -> &[Name] { &self.data[..self.count] }
}

pub fn puzzle1() {
    let devices = Devices::parse(read_lines(11));
    let you = Name::new("you");
    let out = Name::new("out");
    let result = devices.count_paths(you, out, &mut HashMap::new());
    println!("{result}");
}

pub fn puzzle2() {
    let devices = Devices::parse(read_lines(11));
    let svr = Name::new("svr");
    let fft = Name::new("fft");
    let dac = Name::new("dac");
    let out = Name::new("out");
    let svr_fft = devices.count_paths(svr, fft, &mut HashMap::new());
    let svr_dac = devices.count_paths(svr, dac, &mut HashMap::new());
    let fft_dac = devices.count_paths(fft, dac, &mut HashMap::new());
    let dac_fft = devices.count_paths(dac, fft, &mut HashMap::new());
    let fft_out = devices.count_paths(fft, out, &mut HashMap::new());
    let dac_out = devices.count_paths(dac, out, &mut HashMap::new());
    let result = svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out;
    println!("{result}");
}
