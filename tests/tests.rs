use delaunator::{Point, Triangulation};

#[test]
fn basic() {
    validate(&load_fixture(include_str!("fixtures/ukraine.json")));
}

#[test]
fn js_issues() {
    validate(&load_fixture(include_str!("fixtures/issue5.json")));
    validate(&load_fixture(include_str!("fixtures/issue11.json")));
    validate(&load_fixture(include_str!("fixtures/issue13.json")));
    validate(&load_fixture(include_str!("fixtures/issue24.json")));
    validate(&load_fixture(include_str!("fixtures/issue43.json")));
    validate(&load_fixture(include_str!("fixtures/issue44.json")));
}

#[test]
fn robustness() {
    let robustness1 = load_fixture(include_str!("fixtures/robustness1.json"));

    validate(&robustness1);
    validate(&(scale_points(&robustness1, 1e-9)));
    validate(&(scale_points(&robustness1, 1e-2)));
    validate(&(scale_points(&robustness1, 100.0)));
    validate(&(scale_points(&robustness1, 1e9)));

    let robustness2 = load_fixture(include_str!("fixtures/robustness2.json"));
    validate(&robustness2[0..100]);
    validate(&robustness2);
}

#[test]
fn bad_input() {
    let mut points = vec![Point { x: 0., y: 0. }];
    assert!(
        Triangulation::new(&points).is_none(),
        "Expected empty triangulation (1 point)"
    );

    points.push(Point { x: 1., y: 0. });
    assert!(
        Triangulation::new(&points).is_none(),
        "Expected empty triangulation (2 point)"
    );

    points.push(Point { x: 2., y: 0. });
    assert!(
        Triangulation::new(&points).is_none(),
        "Expected empty triangulation (collinear points)"
    );

    points.push(Point { x: 1., y: 1. });
    validate(&points);
}

fn scale_points(points: &[Point<f64>], scale: f64) -> Vec<Point<f64>> {
    let scaled: Vec<Point<f64>> = points
        .iter()
        .map(|p| Point {
            x: p.x * scale,
            y: p.y * scale,
        })
        .collect();
    scaled
}

fn load_fixture(json: &str) -> Vec<Point<f64>> {
    let u: Vec<(f64, f64)> = serde_json::from_str(json).unwrap();
    u.iter().map(|p| Point { x: p.0, y: p.1 }).collect()
}

fn validate(points: &[Point<f64>]) {
    let Triangulation {
        triangles,
        halfedges,
        hull,
    } = Triangulation::new(&points).expect("No triangulation exists for this input");

    // validate halfedges
    for (i, &h) in halfedges.iter().enumerate() {
        if h.get().map(|h| halfedges[h] != i.into()).unwrap_or(false) {
            panic!("Invalid halfedge connection");
        }
    }

    // validate triangulation
    let hull_area = {
        let mut hull_areas = Vec::new();
        let mut i = 0;
        let mut j = hull.len() - 1;
        while i < hull.len() {
            let p0 = &points[hull[j]];
            let p = &points[hull[i]];
            hull_areas.push((p.x + p0.x) * (p.y - p0.y));
            j = i;
            i += 1;
        }
        sum(&hull_areas)
    };
    let triangles_area = {
        let mut triangle_areas = Vec::new();
        let mut i = 0;
        while i < triangles.len() {
            let a = &points[triangles[i]];
            let b = &points[triangles[i + 1]];
            let c = &points[triangles[i + 2]];
            triangle_areas.push(((b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y)).abs());
            i += 3;
        }
        sum(&triangle_areas)
    };

    let err = ((hull_area - triangles_area) / hull_area).abs();
    // const EPSILON: f32 = f32::EPSILON * 4.0;
    const EPSILON: f64 = f64::EPSILON * 2.0;
    assert!(
        err <= EPSILON,
        "Triangulation is broken: {} error, epsilon: {}",
        err,
        EPSILON
    );
}

// Kahan and Babuska summation, Neumaier variant; accumulates less FP error
fn sum(x: &[f64]) -> f64 {
    let mut sum = x[0];
    let mut err = 0.0;
    for &k in x.iter().skip(1) {
        let m = sum + k;
        err += if sum.abs() >= k.abs() {
            sum - m + k
        } else {
            k - m + sum
        };
        sum = m;
    }
    sum + err
}
