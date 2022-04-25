/*
goal: generate simple physics based animations use the ray tracing to generate graphics

things to figure out:
how to detect when one thing collides with another
what to do when collision is detected
how to represent the shape of an object's collision
how to detect when a force is applied


ideas:

struct Collision
    keeps track of the shape of an object's collision, and detects when it collides with something
    detecing sphere collision -
        sphere s1 with radius r1 collides with sphere s2 with radius r2 is if the closest point on s1 to the center of s2 is less than r2 from the center of s2
            how do we figure out the closest point
        if the distance from the center of s1 to the center of s2 is less than r1 + r2 <<<< seems pretty good
    when collision is detected -
        the two objects should exert a force on each other
        force should be scaled surface normal of object at the point of collision
    https://developer.mozilla.org/en-US/docs/Games/Techniques/3D_collision_detection
    this should be a trait i think

struct Physics
    contains physical data: mass (f64), velocity (vec3), total forces acting upon object (vec3), air resistance (f64), friction (f64), spring constant (f64) etc
    should have update function that runs each frame, updating object based on the forces acting upon it: eg. gravity, collision, friction, air resistance
    actually I think this should be a trait
    updates based on real time, not limited by frame rate

struct Object
    has collision and physics
    appearance determined by stuff from graphics package
    maybe should have Option<Physics> so we can make static objects

Gravity:
    source could be a point with a defined mass
    then use the newton gravity force equation on all objects

struct Surface
    maybe have a surface struct to separate surfaces from objects
    the physical impact would be negligable but it would be easier to think about and maybe improve rendering times



Example scene: ball slides down ramp
    initialize with ball placed up a slope, gravity source placed somewhere far below so the ball will move down
    ball experiences gravity scaled by sin of angle of ramp, and slides down ramp accordingly
    ramp should be static and not experience physics, e.g. won't move in the gravitational field and wont be affected by ball pushing back on it, so should use collidable but not physical

*/
use vek::vec::Vec3;

pub type Point3 = Vec3<f64>;
