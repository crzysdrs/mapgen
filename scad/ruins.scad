$fn=20;
seed=50;

count=20;
place = [
         rands(-1,1,count,seed),
         rands(-1,1,count,seed+50),
         rands(0.5,0.8,count,seed + 20),
         rands(0,360,count,seed + 20),
         ];

for (i = [0:count-1]) {
  x = place[0][i];
  y = place[1][i];
  z = place[2][i];
  a = place[3][i];
  
  scale(1.4)
    translate([x,y,0])
    if (i < 10) {
      cylinder(z, 0.05,0.05);
    } else {
      rotate(a) {
        difference() {
          cylinder(0.075, 0.3, 0.3);
          translate([0, 0, -0.1])
            cylinder(0.3, 0.25, 0.25);
          translate([0, -0.5, 0])
            cube(1);
        }
      }
    }
 }


