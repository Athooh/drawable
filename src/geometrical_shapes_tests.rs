#[cfg(test)]
mod tests {
    use raster::Image;
    use crate::geometrical_shapes::{Point, Line, Triangle, Rectangle, Circle};
    use crate::geometrical_shapes::Drawable;

    fn create_test_image() -> Image {
        Image::blank(100, 100)
    }

    #[test]
    fn test_point_creation() {
        let p = Point::new(10, 20);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
    }

    #[test]
    fn test_point_draw() {
        let mut image = create_test_image();
        let point = Point::new(5, 5);
        point.draw(&mut image);
        let pixel = image.get_pixel(point.x, point.y).unwrap();
        assert_eq!(pixel.r, point.color().r);
    }

    #[test]
    fn test_line_creation() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 10);
        let line = Line::new(&p1, &p2);
        assert_eq!(line.start.x, p1.x);
        assert_eq!(line.end.y, p2.y);
    }

    #[test]
    fn test_triangle_creation() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 0);
        let p3 = Point::new(5, 10);
        let triangle = Triangle::new(&p1, &p2, &p3);
        assert!(triangle.color().a == 255);
    }

    #[test]
    fn test_rectangle_creation() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 20);
        let rect = Rectangle::new(&p1, &p2);
        assert!(rect.color().a == 255);
    }

    #[test]
    fn test_circle_creation() {
        let center = Point::new(50, 50);
        let circle = Circle::new(&center, 10);
        assert_eq!(circle.radius, 10);
    }

    #[test]
    fn test_circle_draw() {
        let mut image = create_test_image();
        let center = Point::new(50, 50);
        let circle = Circle::new(&center, 10);
        circle.draw(&mut image);
        let sample_pixel = image.get_pixel(center.x + 10, center.y).unwrap();
        assert_eq!(sample_pixel.r, circle.color().r);
    }
}