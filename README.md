# raytracer-challenge-rs
This is an implementation of the raytracer from the book The Ray Tracer 
Challenge by Jamis Buck using the Rust programming language

#Issues/TODOs
Problems with adding vectors and points, should a point + vector be a point or a vector.

Should we add the w component in Add/Sub for vectors? This may be good to do since that would mean
a vector plus point would equal to point, a vector + vector would equal vector, and a point + point would give
a result of a w = 2.0. Which we could then detect as an error?

So far it looks like adding w component is the right call. 


#FAQ

Q: How do I actually render an image?
A: There is currenly no code in src/main.rs. If you want to see an image, look
   in the tests directory for tests that are marked as disabled (#[ignore="render"]).
   Re-enable the tests and run 
```
cargo test --release
```
    The rendering will output a .ppm file which you can view in Preview.app

