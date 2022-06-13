$fn=30;

lower_height = 0.25;
cylinder(lower_height, 1, 1);

translate([0,0,lower_height])
cylinder(1, 0.2, 0.2);

translate([0, 0, lower_height])
difference() {
  cylinder(0.3, 1, 1);
  translate([0, 0, -1])
    cylinder(3, 0.8, 0.8);
}
;
