$fn=50;

scale([1.5, 1.5, 2])
linear_extrude(height=0.3, convexity=10, twist=20, scale=0.6)
square();

translate([0.5, 0.25, 0.3])
scale([0.75, 0.75, 2]) {
  rad =0.4;
  height=1;
  translate([0, 0, height])
    scale([1, 1, 3])
    cylinder(0.3, rad, 0.0);
  cylinder(height, rad, rad);
}
