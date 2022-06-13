$fn=50;

translate([-0.75, -0.75, 0])
scale([1.5, 1.25, 1]) {
  translate([0.5, 0.3, 0])
    cube([0.75,0.75,1]);
  
  for(j = [0:1]) {
    translate([0, j *1.3, 0])
        for(i = [0:2]) {
          translate([i/2, 0, 0])
            cylinder(0.5+i/5, 0.25, 0.25);
          
        };
  };
  
  translate([0, 0.05, 0])
    for (i = [1:3]) {
      width=0.3;
      translate([0, i * width, 0])
        cylinder(0.5, width/2, width/2);
    }
}
