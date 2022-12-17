use nalgebra::{Point2, Vector2};

fn cross_product(one: &Point2<f64>, two: &Point2<f64>, three: &Point2<f64>) -> f64 {
    let vector_first = Vector2::new(two.x - one.x, two.y - one.y);
    let vector_last = Vector2::new(three.x - two.x, three.y - two.y);
    vector_first.x * vector_last.y - vector_last.x * vector_first.y
}

pub fn scan_upper(points: &mut Vec<Point2<f64>>) -> Vec<Point2<f64>> {
    // Sort points lexicographically.
    points.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap());

    let mut upper_convec_hull: Vec<Point2<f64>> = vec![];

    // Insert the first two points.
    upper_convec_hull.push(points.remove(0));
    upper_convec_hull.push(points.remove(0));

    for _ in 0..points.len() {
        upper_convec_hull.push(points.remove(0));
        while upper_convec_hull.len() > 2
            && cross_product(
                &upper_convec_hull[upper_convec_hull.len() - 3],
                &upper_convec_hull[upper_convec_hull.len() - 2],
                &upper_convec_hull[upper_convec_hull.len() - 1],
            ) > 0.0
        {
            upper_convec_hull.remove(upper_convec_hull.len() - 2);
        }
    }

    upper_convec_hull
}

pub fn scan_lower(points: &mut Vec<Point2<f64>>) -> Vec<Point2<f64>> {
    // Sort points lexicographically.
    points.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap());

    let mut lower_convec_hull: Vec<Point2<f64>> = vec![];

    // Insert the first two points.
    lower_convec_hull.push(points.remove(0));
    lower_convec_hull.push(points.remove(0));

    for _ in 0..points.len() {
        lower_convec_hull.push(points.remove(0));
        while lower_convec_hull.len() > 2
            && cross_product(
                &lower_convec_hull[lower_convec_hull.len() - 3],
                &lower_convec_hull[lower_convec_hull.len() - 2],
                &lower_convec_hull[lower_convec_hull.len() - 1],
            ) < 0.0
        {
            lower_convec_hull.remove(lower_convec_hull.len() - 2);
        }
    }

    lower_convec_hull
}
