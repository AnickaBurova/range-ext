# Intersection between two ranges

``` rust
    use range_ext::intersect::*;
    if (3..10).intersect(&(8..33)).is_any() {
        ...
    }


    match (10..22).intersect(&(0..11)) {
        IntersectionExt::Bellow => (), // no intersection
        IntersectionExt::BellowOverlap => (),
        IntersectionExt::Within => (),
        IntersectionExt::Same => (),
        IntersectionExt::Over => (),
        IntersectionExt::AboveOverlap => (),
        IntersectionExt::Above => (), // no intersection
    }
```
