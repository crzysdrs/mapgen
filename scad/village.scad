$fn=20;
seed=50;

count=20;
place = [
         rands(-1,1,count,seed),
         rands(-1,1,count,seed+50),
         rands(0.5,1.5,count,seed + 20),
         rands(0,360,count,seed + 20),
         ];

scale([1,1,1.5])
for (i = [0:count-1]) {
  x = place[0][i];
  y = place[1][i];
  z = place[2][i];
  a = place[3][i];

  rotate(a)
    translate([x, y, 0])
    scale(0.2)
    scale([1,1,z]) {
    if (i % 2 == 0) {
      translate([0, 0, 1])
        scale([1, 2, 1])
        rotate(45)
        cylinder(1, 1, 0, $fn=4);
      
      translate([0, 0, 0.5])
        cube([1,2,1], center=true);
    } else {
      translate([0, 0, 1])
        cylinder(1,0.5,0);
    
      cylinder(1,0.5,0.5);
    }
  }
 }
