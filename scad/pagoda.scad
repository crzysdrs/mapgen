$fn=20;

for(i = [0:4]) {
  scale(1 - i * 0.05) {
    translate([0, 0, 1 * i])
      cylinder(1, 0.75, 0.75);
    
    translate([0, 0, 1 * i + 1])
      if (i == 4) {
        cylinder(0.5, 1.6  - i * 0.1, 0);
      } else {
        cylinder(0.2, 1.2 - i * 0.05, 0.75);
      }
  }
}
