type Line = u64;

pub struct Map {
    data: [Line; 12],
}

pub type Coordinate = (u8, u8);

pub const EMPTY: u8 = 0x00;
pub const PADDING: u8 = 0x08;

pub const RB: u8 = 0x01;
pub const RP: u8 = 0x02;
pub const RC: u8 = 0x03;
pub const RM: u8 = 0x04;
pub const RX: u8 = 0x05;
pub const RS: u8 = 0x06;
pub const RJ: u8 = 0x07;

pub const BB: u8 = 0x09;
pub const BP: u8 = 0x0a;
pub const BC: u8 = 0x0b;
pub const BM: u8 = 0x0c;
pub const BX: u8 = 0x0d;
pub const BS: u8 = 0x0e;
pub const BJ: u8 = 0x0f;

fn overridable(p0: u8, p1: u8) -> bool {
    p1 != PADDING && ((p0 & 8u8) ^ (p1 & 8u8) != 0 || p1 == EMPTY)
}

impl Map {
    pub fn new() -> Map {
        Map {
            data: [
                0x88888888888,
                0x8bcdefedcb8,
                0x80000000008,
                0x80a00000a08,
                0x89090909098,
                0x80000000008,
                0x80000000008,
                0x81010101018,
                0x80200000208,
                0x80000000008,
                0x83456765438,
                0x88888888888,
            ],
        }
    }

    fn get(&self, t: &Coordinate) -> u8 {
        ((self.data[t.0 as usize] >> (t.1 * 4)) & 0x0f) as u8
    }

    pub fn mv(&mut self, from: &Coordinate, to: &Coordinate) {
        let u = self.get(&from);
        self.data[from.0 as usize] &= !(0x0fu64 << (from.1 * 4));
        self.data[to.0 as usize] &= !(0x0fu64 << (to.1 * 4));
        self.data[to.0 as usize] |= (u as u64) << (to.1 * 4);
    }

    pub fn get_candidates(&self, t: &Coordinate) -> Vec<Coordinate> {
        let u = self.get(t);
        match u {
            RB => {
                if t.0 >= 6 && overridable(u, self.get(&(t.0 - 1, t.1))) {
                    vec![(t.0 - 1, t.1)]
                } else {
                    [(0i16, 1i16), (0i16, -1i16), (-1i16, 0i16)]
                        .into_iter()
                        .map(|(x, y)| ((t.0 as i16 + x) as u8, (t.1 as i16 + y) as u8))
                        .filter(|m| overridable(u, self.get(&m)))
                        .collect::<Vec<Coordinate>>()
                }
            }
            BB => {
                if t.0 <= 5 && overridable(u, self.get(&(t.0 - 1, t.1))) {
                    vec![(t.0 + 1, t.1)]
                } else {
                    [(0i16, 1i16), (0i16, -1i16), (1i16, 0i16)]
                        .into_iter()
                        .map(|(x, y)| ((t.0 as i16 + x) as u8, (t.1 as i16 + y) as u8))
                        .filter(|m| overridable(u, self.get(&m)))
                        .collect::<Vec<Coordinate>>()
                }
            }
            RP | BP => Vec::new(),
            RC | BC => Vec::new(),
            RM | BM => Vec::new(),
            RX | BX => match t {
                (10, 7) => vec![(8, 9), (8, 5)],
                (10, 3) => vec![(8, 1), (8, 5)],
                (8, 9) => vec![(6, 7), (10, 7)],
                (8, 1) => vec![(6, 3), (10, 3)],
                (6, 7) => vec![(8, 9), (8, 5)],
                (6, 3) => vec![(8, 1), (8, 5)],
                (8, 5) => vec![(10, 7), (10, 3), (6, 7), (6, 3)],
                (1, 7) => vec![(3, 9), (3, 5)],
                (1, 3) => vec![(3, 1), (3, 5)],
                (3, 1) => vec![(5, 3), (1, 3)],
                (3, 9) => vec![(1, 7), (5, 7)],
                (5, 3) => vec![(3, 1), (3, 5)],
                (5, 7) => vec![(3, 9), (3, 5)],
                (3, 5) => vec![(1, 7), (1, 3), (5, 3), (5, 7)],
                _ => vec![],
            }
            .into_iter()
            .filter(|d| overridable(u, self.get(&d)))
            .collect::<Vec<Coordinate>>(),
            RS | BS => match t {
                (10, 6) | (10, 4) | (8, 6) | (8, 4) => vec![(9, 5)],
                (9, 5) => vec![(10, 6), (10, 4), (8, 6), (8, 4)],
                (1, 6) | (1, 4) | (3, 6) | (3, 4) => vec![(2, 5)],
                (2, 5) => vec![(1, 6), (1, 4), (3, 6), (3, 4)],
                _ => vec![],
            }
            .into_iter()
            .filter(|d| overridable(u, self.get(&d)))
            .collect::<Vec<Coordinate>>(),
            RJ | BJ => [(0i16, 1i16), (0i16, -1i16), (1i16, 0i16), (-1i16, 0i16)]
                .into_iter()
                .map(|(x, y)| ((t.0 as i16 + x) as u8, (t.1 as i16 + y) as u8))
                .filter(|(x, y)| {
                    (*x >= 1u8 && *x <= 3u8 && *y >= 4u8 && *y <= 6u8)
                        || (*x >= 8u8 && *x <= 10u8 && *y >= 4u8 && *y <= 6u8)
                })
                .filter(|m| overridable(u, self.get(&m)))
                .collect::<Vec<Coordinate>>(),
            _ => vec![],
        }
    }
}

#[test]
pub fn test_get() {
    let map = Map::new();
    assert_eq!(map.get(&(10, 1)), RC);
    assert_eq!(map.get(&(10, 9)), RC);
    assert_eq!(map.get(&(10, 2)), RM);
    assert_eq!(map.get(&(10, 8)), RM);
    assert_eq!(map.get(&(10, 3)), RX);
    assert_eq!(map.get(&(10, 7)), RX);
    assert_eq!(map.get(&(10, 4)), RS);
    assert_eq!(map.get(&(10, 6)), RS);
    assert_eq!(map.get(&(10, 5)), RJ);
    assert_eq!(map.get(&(8, 2)), RP);
    assert_eq!(map.get(&(8, 8)), RP);
    assert_eq!(map.get(&(7, 1)), RB);
    assert_eq!(map.get(&(7, 3)), RB);
    assert_eq!(map.get(&(7, 5)), RB);
    assert_eq!(map.get(&(7, 7)), RB);
    assert_eq!(map.get(&(7, 9)), RB);

    assert_eq!(map.get(&(1, 1)), BC);
}

#[test]
pub fn test_move() {
    assert_eq!(overridable(RB, RC), false);
    assert_eq!(overridable(RB, BC), true);
    assert_eq!(overridable(RB, EMPTY), true);
    let mut map = Map::new();
    // 当头炮
    map.mv(&(8, 2), &(8, 5));
    assert_eq!(map.data[8], 0x80200200008);
    map.mv(&(8, 5), &(8, 2));
    assert_eq!(map.data[8], 0x80200000208);
    // 飞象
    map.mv(&(10, 7), &(8, 5));
    assert_eq!(map.data[8], 0x80200500208);
    assert_eq!(map.data[10], 0x83406765438);
}

#[test]
pub fn test_candidates() {
    // 红兵黑卒交替前进
    let mut map = Map::new();
    assert_eq!(&map.get_candidates(&(7, 3)), &[(6, 3)]);
    map.mv(&(7, 3), &(6, 3));
    assert_eq!(&map.get_candidates(&(6, 3)), &[(5, 3)]);
    assert_eq!(&map.get_candidates(&(4, 3)), &[(5, 3)]);
    map.mv(&(4, 3), &(5, 3));
    map.mv(&(6, 3), &(5, 3));
    assert_eq!(map.data[5], 0x80000001008);
    assert_eq!(&map.get_candidates(&(5, 3)), &[(5, 4), (5, 2), (4, 3)]);
    // 红帅黑将
    assert_eq!(&map.get_candidates(&(10, 5)), &[(9, 5)]);
    assert_eq!(&map.get_candidates(&(1, 5)), &[(2, 5)]);
}
