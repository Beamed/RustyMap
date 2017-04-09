pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

pub fn get_distance(c1: &Coordinate, c2: &Coordinate) -> i32  {
    //pretty sloppy but eh, you know the ole distance formula
    (((c2.x - c1.x).pow(2)+(c2.y - c1.y).pow(2)) as f32).sqrt() as i32
}

impl Coordinate {
    pub fn get_distance(&self, c2: &Coordinate) ->i32 {
        get_distance(self, c2)
    }

    pub fn new(x_: i32, y_: i32) -> Self {
        Coordinate{
            x: x_,
            y: y_
        }
    }
}

mod test {
    use map::coordinate::Coordinate;
    #[test]
    fn coordinate_tests() {
        let c1: Coordinate = Coordinate{
            x: 16, y: 4
        };
        let c2: Coordinate = Coordinate {
            x: 25, y: 16
        };
        let c3: Coordinate = Coordinate {
            x: 22, y: 48
        };
        let c4: Coordinate = Coordinate::new(93, 234);

        assert_eq!(c1.get_distance(&c2), 15);
        assert_eq!(c2.get_distance(&c1), 15);
        let second_dist = c3.get_distance(&c4);
        assert_eq!(second_dist, 199);
        assert_eq!(second_dist, c4.get_distance(&c3));
    }
}
