fn main() {
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let n = buffer.trim().parse::<i64>().unwrap();

        if n == 0 { return; }

        let mut pos: Vec<i64> = vec![0; 2020];
        let mut points: Vec<Point> = vec![Point::new(0, 0); 2020];

        for i in 1..=n {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let p = buffer.split_ascii_whitespace().map(|x|x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            points[i as usize] = Point::new(p[0], p[1]);
        }

        for i in 1..=n {
            pos[i as usize] = i;
        }

        points[1..(n as usize + 1)].sort_by(|a, b| if a.x != b.x { a.x.cmp(&b.x) } else { a.y.cmp(&b.y) });

        let mut v: Vec<Line> = vec![];

        for i in 1..=n {
            for j in i+1..=n {
                v.push(Line::new(i, j, points[i as usize], points[j as usize]));
            } 
        }

        v.sort_by(
            |a, b|
            {
                let le = a.dy * b.dx;
                let ri = b.dy * a.dx;
                if le != ri {
                    return le.cmp(&ri);
                } else {
                    if a.i != b.i {
                        return a.i.cmp(&b.i);
                    } else {
                        return a.j.cmp(&b.j);
                    }
                }
            }
        );

        let mut min: i64 = 4557430888798830399;
        let mut max: i64 = -4557430888798830400;
        let mut i = 0;
        let mut j = 0;
        while i < v.len() {
            while j < v.len() && v[i].equal(&v[j]) { j += 1; }

            for k in i..j {
                let mut u = v[k].i as usize;
                let mut m = v[k].j as usize;
                pos.swap(u, m);
                points.swap(pos[u] as usize, pos[m] as usize);
                if pos[u] > pos[m] { let temp = u; u = m; m = temp; }
                if pos[u] > 1 {
                    min = min.min(area(&points[pos[u] as usize], &points[pos[m] as usize], &points[(pos[u] - 1) as usize]));
                    max = max.max(area(&points[pos[u] as usize], &points[pos[m] as usize], &points[1]));
                }
                if pos[m] < n {
                    min = min.min(area(&points[pos[u] as usize], &points[pos[m] as usize], &points[(pos[m] + 1) as usize]));
                    max = max.max(area(&points[pos[u] as usize], &points[pos[m] as usize], &points[n as usize]));
                }
            }

            i = j;
        }

        println!("{}.{} {}.{}", min / 2, min%2*5, max / 2, max%2*5);
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

struct Line {
    i: i64,
    j: i64,
    dx: i64,
    dy: i64,
}

impl Line {
    fn new(i: i64, j: i64, pi: Point, pj: Point) -> Self {
        Self {
            i: i,
            j: j,
            dx: (pj.x - pi.x),
            dy: (pj.y - pi.y),
        }
    }

    fn equal(&self, other: &Self) -> bool {
        (self.dy * other.dx) == (self.dx * other.dy)
    }
}

fn area(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    ((p2.x - p1.x) * (p3.y - p2.y) - (p3.x - p2.x) * (p2.y - p1.y)).abs()
}