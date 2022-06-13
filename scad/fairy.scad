$fn=50;

offsets = [
       [[0,0,0],1, 0.6],
       [[1,2,0], 1.25, 0,2],
       [[2,1,0], 2, 0.3],
       [[-1,0,0], 2, 0.1],
       ];

translate([-0.5, -0.5, 0])
for (off = offsets) {
  translate(off[0])
    scale([1, 1+ off[2], off[1]]) {
    for (fall = [-40, 30, 60]) {
      rotate(fall, [0, 0, 1])
        translate([1.01, 0, 0]){
        scale([0.75, 0.75, 1]) {
          translate([0, 0, 0.75])
            scale([1, 1, 1])
            cylinder(0.25, 0.25, 0.1);
          cylinder(0.75, 0.25, 0.25);
        }
      }
    }
    difference() {
      cylinder(1, 1, 1);
      translate([0, 0, 0.6])
        cylinder(0.5, 0.1, 1);
    }
  }
 }
