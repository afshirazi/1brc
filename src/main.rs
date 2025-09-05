use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::Path,
    time::Instant,
};

fn main() {
    let start = Instant::now();
    let path = Path::new("./resources/measurements.txt");
    let mut f = fs::File::open(path).unwrap();
    let mut buf = String::new();

    let size = f.read_to_string(&mut buf).unwrap();
    println!("size of buffer: {size}");
    println!(
        "time taken to read: {}",
        Instant::now().duration_since(start).as_nanos()
    );

    let map = buf
        .split('\n')
        .filter(|el| !el.trim().is_empty())
        .map(|line| {
            let mut split = line.split(';');
            let name = split.next().unwrap();
            let temp = split
                .next()
                .map(|num| num.parse::<f64>().unwrap())
                .expect(line);
            (name, temp)
        })
        .fold(HashMap::new(), |mut map, (name, temp)| {
            if map.contains_key(name) {
                let (omin, omean, omax, on) = map.get(name).unwrap();
                let nn = on + 1;
                let nmin = if temp < *omin { temp } else { *omin };
                let nmean = (omean * *on as f64 + temp) / nn as f64;
                let nmax = if temp > *omax { temp } else { *omax };
                map.insert(name, (nmin, nmean, nmax, nn));
            } else {
                map.insert(name, (temp, temp, temp, 1));
            }
            map
        });

    println!(
        "time taken to map: {}",
        Instant::now().duration_since(start).as_nanos()
    );

    let mut output = fs::File::create_new(Path::new("./resources/results.txt")).unwrap();
    map.iter().for_each(|(name, (min, mean, max, _))| {
        output
            .write(format!("{}={}/{}/{}\n", name, min, mean, max).as_bytes())
            .unwrap();
    });
}
