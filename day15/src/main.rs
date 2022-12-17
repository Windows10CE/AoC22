use std::time::Instant;

struct Sensor {
    location: (i64, i64),
    distance: i64,
}

impl Sensor {
    fn within_bounds(&self, point: (i64, i64)) -> bool {
        self.distance >= abs_diff(self.location.0, point.0) + abs_diff(self.location.1, point.1)
    }
}

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let sensors: Vec<Sensor> = input
        .lines()
        .map(|mut line| {
            line = &line[12..];
            let end = line.find(',').unwrap();
            let sensor_x = line[..end].parse().unwrap();
            line = &line[end + 4..];
            let end = line.find(':').unwrap();
            let sensor_y = line[..end].parse().unwrap();
            line = &line[end + 25..];
            let end = line.find(',').unwrap();
            let beacon_x = line[..end].parse().unwrap();
            line = &line[end + 4..];
            let beacon_y = line.parse().unwrap();
            Sensor {
                location: (sensor_x, sensor_y),
                distance: abs_diff(sensor_x, beacon_x) + abs_diff(sensor_y, beacon_y),
            }
        })
        .collect();

    const TARGET_ROW: i64 = 2000000;

    let viable_sensors: Vec<_> = sensors
        .iter()
        .filter(|s| abs_diff(s.location.1, TARGET_ROW) <= s.distance)
        .collect();

    let min_x = viable_sensors
        .iter()
        .map(|s| {
            let remaining_range = s.distance - abs_diff(TARGET_ROW, s.location.1);
            s.location.0 - remaining_range
        })
        .min()
        .unwrap();
    let max_x = viable_sensors
        .iter()
        .map(|s| {
            let remaining_range = s.distance - abs_diff(TARGET_ROW, s.location.1);
            s.location.0 + remaining_range
        })
        .max()
        .unwrap();

    let count_covered = (min_x..max_x)
        .filter(|x| {
            viable_sensors.iter().any(|s| {
                abs_diff(s.location.0, *x) <= s.distance - abs_diff(s.location.1, TARGET_ROW)
            })
        })
        .count();

    const BOUNDS: ((i64, i64), (i64, i64)) = ((0, 4000000), (0, 4000000));

    let location = sensors
        .iter()
        .filter_map(|s| {
            let mut test_loc = (s.location.0 + s.distance + 1, s.location.1);
            for _ in 0..=s.distance + 1 {
                test_loc.0 -= 1;
                test_loc.1 += 1;
                if within(test_loc, BOUNDS) && sensors.iter().all(|s| !s.within_bounds(test_loc)) {
                    return Some(test_loc);
                }
            }
            for _ in 0..=s.distance + 1 {
                test_loc.0 -= 1;
                test_loc.1 -= 1;
                if within(test_loc, BOUNDS) && sensors.iter().all(|s| !s.within_bounds(test_loc)) {
                    return Some(test_loc);
                }
            }
            for _ in 0..=s.distance + 1 {
                test_loc.0 += 1;
                test_loc.1 -= 1;
                if within(test_loc, BOUNDS) && sensors.iter().all(|s| !s.within_bounds(test_loc)) {
                    return Some(test_loc);
                }
            }
            for _ in 0..=s.distance + 1 {
                test_loc.0 += 1;
                test_loc.1 += 1;
                if within(test_loc, BOUNDS) && sensors.iter().all(|s| !s.within_bounds(test_loc)) {
                    return Some(test_loc);
                }
            }
            None
        })
        .next()
        .unwrap();

    let frequency = location.0 * 4000000 + location.1;

    let end = Instant::now();
    println!("Total time: {:#?}", end - start);

    println!("Part 1: {count_covered}");
    println!("Part 2: {frequency}");
}

fn abs_diff(a: i64, b: i64) -> i64 {
    a.abs_diff(b) as i64
}

fn within(point: (i64, i64), bounds: ((i64, i64), (i64, i64))) -> bool {
    point.0 >= bounds.0 .0
        && point.0 <= bounds.0 .1
        && point.1 >= bounds.1 .0
        && point.1 <= bounds.1 .1
}
