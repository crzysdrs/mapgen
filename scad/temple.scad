$fn=50;

scale([2, 1, 1]) {
  cylinder(2,1,1);
  translate([0, 0, 2])
    cylinder(1, 1, 0.5);


};

translate([-0.5, 0, 3]) {
  cylinder(2, 0.4, 0.4);
  translate([0, 0, 2]) {
    cylinder(2, 0.4, 0);
  }
}
