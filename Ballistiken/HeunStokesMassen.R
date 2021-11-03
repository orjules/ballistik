heunStokes1kg <- read.table("../TextOutputs/heunStokes1kg.txt",
                            header = TRUE,
                            sep = ",",
                            dec = ".")
heunStokes10kg <- read.table("../TextOutputs/heunStokes10kg.txt",
                      header = TRUE,
                      sep = ",",
                      dec = ".")
nurHeun <- read.table("../TextOutputs/nurHeun.txt",
                      header = TRUE,
                      sep = ",",
                      dec = ".")

plot(x = 1,                 
     xlab = "Weite", 
     ylab = "Höhe",
     xlim = c(0, 210000), 
     ylim = c(0, 55000),
     main = "Heun-Stokes mit verschiedenen Massen",
     type = "n")
legend("topright", 
       legend=c("Mit 1kg", "Mit 10kg", "Ohne Reibung"), 
       col=c("red", "blue", "pink"), pch=c(1,1, 1), cex=0.8)
points(heunStokes1kg$Ort_x, heunStokes1kg$Ort_y, col="red")
points(heunStokes10kg$Ort_x, heunStokes10kg$Ort_y, col="blue")
points(nurHeun$Ort_x, nurHeun$Ort_y, col="pink")