$fn=50;

translate([-0.5,-0.5, 0]) {
translate([0.5,0.4,0.5])
difference() {  
  scale(1)
    cube(1,center=true);
  scale([1,1,2])
    scale(0.7)
    cube(1, center=true);
};
      
for (i = [[0,0,0],[0,0.75,0], [1,0,0], [1,0.75,0]]) {
  translate(i) {
    translate([0,0,1])
      difference() {  
      cylinder(0.1, 0.3, 0.3);
      translate([0,0,.01])
        cylinder(1.0, 0.25, 0.25);
    };
  
    cylinder(1,0.25,0.25);
  }
  
 }
}
