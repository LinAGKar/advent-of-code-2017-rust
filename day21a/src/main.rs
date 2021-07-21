use std::collections::HashMap;
use std::io::Read;

fn rotate((len, grid): (u8, u64)) -> (u8, u64) {
    (len, (0..len).flat_map(|line| {
        (0..len).map(move |col| (line, col))
    }).fold(0, |acc, (line, col)| {
        acc | (grid >> line * len + col & 0b1) << col * len + len - 1 - line
    }))
}

fn flip((len, grid): (u8, u64)) -> (u8, u64) {
    let line_mask = !(!0 << len);
    (len, (0..len).fold(0, |acc, i| {
        acc << len | grid >> i * len & line_mask
    }))
}

fn parse_grid(string: &str) -> (u8, u64) {
    let (len, grid) = string.chars().filter(|x| ['#', '.'].contains(x)).enumerate().fold((0, 0), |(len, grid), (i, tile)| {
        (len + 1, grid | if tile == '#' { 1 } else { 0 } << i)
    });
    let size = (len as f32).sqrt() as u8;
    assert_eq!(size * size, len);
    (size, grid)
}

fn pixel_count(
    cache: &mut HashMap<((u8, u64), u8), u32>,
    mappings: &HashMap<(u8, u64), Vec<(u8, u64)>>,
    source: (u8, u64),
    depth: u8,
) -> u32 {
    if let Some(&val) = cache.get(&(source, depth)) {
        val
    } else {
        let val = if depth == 0 {
            (0..source.0.pow(2)).map(|i| (source.1 >> i & 0b1) as u32).sum()
        } else {
            mappings[&source].iter().map(|&target| {
                pixel_count(cache, mappings, target, depth - 1)
            }).sum()
        };
        cache.insert((source, depth), val);
        val
    }
}

fn fit(
    mappings: &HashMap<(u8, u64), (u8, u64)>,
    source: (u8, u64),
) -> (u8, u64) {
    (0..8).try_fold(source, |source, i| {
        let source = if i == 4 { flip(source) } else { source };
        if mappings.contains_key(&source) {
            Err(source)
        } else {
            Ok(rotate(source))
        }
    }).unwrap_err()
}

fn join(tiles: &[(u8, u64)]) -> (u8, u128) {
    let side_len = (tiles.len() as f32).sqrt() as u8;
    assert_eq!(side_len.pow(2) as usize, tiles.len());
    let tile_size = tiles[0].0;

    let line_mask = !(!0 << tile_size);

    (tile_size * side_len, (0..side_len).flat_map(|x| {
        (0..side_len).flat_map(move |y| {
            (0..tile_size).map(move |tile_line| (x, y, tile_line))
        })
    }).fold(0u128, |acc, (x, y, tile_line)| {
        let (this_tile_size, tile) = tiles[(y * side_len + x) as usize];
        assert_eq!(this_tile_size, tile_size);
        let line = (tile >> tile_line * tile_size & line_mask) as u128;
        acc | line << (x + (y * tile_size + tile_line) * side_len) * tile_size
    }))
}

fn split((full_size, grid): (u8, u128), tile_size: u8) -> Vec<(u8, u64)> {
    let side_len = full_size / tile_size;
    let line_mask = !(!0 << tile_size);

    (0..side_len).flat_map(|y| {
        (0..side_len).map(move |x| {
            (tile_size, (0..tile_size).fold(0, |acc, tile_line| {
                let offset = (x + (y * tile_size + tile_line) * side_len) * tile_size;
                let line = (grid >> offset & line_mask) as u64;
                acc | line << tile_size * tile_line
            }))
        })
    }).collect()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mappings: HashMap<_, _> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        (parse_grid(words.next().unwrap()), parse_grid(words.nth(1).unwrap()))
    }).collect();

    let source = fit(&mappings, parse_grid(".#./..#/###"));

    let mut new_mappings = HashMap::new();

    for (&source, &target_0) in mappings.iter().filter(|(&(source_size, _), _)| source_size == 3) {
        let target_1 = join(
            &split((target_0.0, target_0.1 as u128), 2)
                .into_iter()
                .map(|tile| mappings[&fit(&mappings, tile)])
                .collect::<Vec<_>>(),
        );
        let target_2 = split(target_1, 2).into_iter().map(|tile| {
            let new_tile = mappings[&fit(&mappings, tile)];
            fit(&mappings, (new_tile.0, new_tile.1 as u64))
        }).collect();

        let source = (source.0, source.1 as u64);
        let target_0 = (target_0.0, target_0.1 as u64);
        let target_1 = (target_1.0, target_1.1 as u64);

        new_mappings.insert(source, vec![target_0]);
        new_mappings.insert(target_0, vec![target_1]);
        new_mappings.insert(target_1, target_2);
    }

    let mut cache = HashMap::new();
    println!("{}", pixel_count(&mut cache, &new_mappings, (source.0, source.1 as u64), 5));
}
