use nalgebra::{Point2, Vector2};

pub struct GrahamScan {
    points: Vec<Point2<f64>>,
}

impl GrahamScan {
    fn cross_product(one: &Point2<f64>, two: &Point2<f64>, three: &Point2<f64>) -> f64 {
        let vector_first = Vector2::new(two.x - one.x, two.y - one.y);
        let vector_last = Vector2::new(three.x - two.x, three.y - two.y);
        vector_first.x * vector_last.y - vector_last.x * vector_first.y
    }

    pub fn scan_upper(points: &mut Vec<Point2<f64>>) -> Vec<Point2<f64>> {
        // Sort points lexicographically.
        points.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap());

        let mut upper_convex_hull: Vec<Point2<f64>> = vec![];

        // Insert the first two points.
        upper_convex_hull.push(points.remove(0));
        upper_convex_hull.push(points.remove(0));

        for _ in 0..points.len() {
            upper_convex_hull.push(points.remove(0));
            while upper_convex_hull.len() > 2
                && GrahamScan::cross_product(
                    &upper_convex_hull[upper_convex_hull.len() - 3],
                    &upper_convex_hull[upper_convex_hull.len() - 2],
                    &upper_convex_hull[upper_convex_hull.len() - 1],
                ) > 0.0
            {
                upper_convex_hull.remove(upper_convex_hull.len() - 2);
            }
        }

        upper_convex_hull
    }

    pub fn scan_lower(points: &mut Vec<Point2<f64>>) -> Vec<Point2<f64>> {
        // Sort points lexicographically.
        points.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap());

        let mut lower_convex_hull: Vec<Point2<f64>> = vec![];

        // Insert the first two points.
        lower_convex_hull.push(points.remove(0));
        lower_convex_hull.push(points.remove(0));

        for _ in 0..points.len() {
            lower_convex_hull.push(points.remove(0));
            while lower_convex_hull.len() > 2
                && GrahamScan::cross_product(
                    &lower_convex_hull[lower_convex_hull.len() - 3],
                    &lower_convex_hull[lower_convex_hull.len() - 2],
                    &lower_convex_hull[lower_convex_hull.len() - 1],
                ) < 0.0
            {
                lower_convex_hull.remove(lower_convex_hull.len() - 2);
            }
        }

        lower_convex_hull
    }
}
