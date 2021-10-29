v111s1 <- read.table("../TextOutputs/v111s1.txt",
                        header = TRUE,
                        sep = ",",
                        dec = ".")

scatterplot3d(v111s1$Ort_x, v111s1$Ort_y, v111s1$Ort_z,
              xlab = "X", ylab = "Y", zlab = "Z",
              main = "3D Wurf")

plot3d(v111s1$Ort_x,v111s1$Ort_y, v111s1$Ort_z, 
       col = "blue",
       xlab = "X",
       ylab = "Y",
       zlab = "Z"
       )
rglwidget() 

librascatter3D(v111s1$Ort_x, v111s1$Ort_y, v111s1$Ort_z, labels("X", "Y", "Z"))

# Erkenntnis: 3d ist interaktiv gut aber im PDF schwierig