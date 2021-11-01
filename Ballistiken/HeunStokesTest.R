heunstokes1 <- read.table("../TextOutputs/heunstokes1.txt",
                   header = TRUE,
                   sep = ",",
                   dec = ".")
heunstokes05 <- read.table("../TextOutputs/heunstokes05.txt",
                    header = TRUE,
                    sep = ",",
                    dec = ".")
heunstokes2 <- read.table("../TextOutputs/heunstokes2.txt",
                     header = TRUE,
                     sep = ",",
                     dec = ".")

plot(x = 1,                 
     xlab = "Weite", 
     ylab = "Höhe",
     xlim = c(0, 2500), 
     ylim = c(0, 600),
     main = "Heun mit Reibung",
     type = "n")
legend("topright", 
       legend=c("HeunStokes mit 2s", "HeunStokes mit 1s", "HeunStokes mit 0.5s"), 
       col=c("red", "blue", "pink"), pch=c(1,5,3), cex=0.8)
points(heunstokes2$Ort_x, heunstokes2$Ort_y, col="red")
points(heunstokes1$Ort_x, heunstokes1$Ort_y, pch=5, col="blue")
points(heunstokes05$Ort_x, heunstokes05$Ort_y, pch=3 , col="pink")
