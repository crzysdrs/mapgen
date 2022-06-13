
$fn=50;

import("/home/crzysdrs/proj/icos/test.stl");

translate([75, -27, 0]) {

  translate([25, 2, 0])
    scale([28, 6, 4])
    difference() {
    cylinder(0.5, 1, 1);
    translate([0, 0, 0.4])
      cylinder(0.6, 0.98, 0.98);
  }

  linear_extrude(2)
    text("Jade Moon Apocalypse", size=5, font="Kingthings Calligraphica:style=Regular");
    
}
  

translate([115, -85.8, -0.25])
rotate(a=[90,0,0])
linear_extrude(3)
text("crzysdrs", size=2, font="Ubuntu Mono:style=Regular");
