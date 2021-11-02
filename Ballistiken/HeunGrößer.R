heunstokes01 <- read.table("../TextOutputs/heunstokes01.txt",
                          header = TRUE,
                          sep = ",",
                          dec = ".")
heunstokes1 <- read.table("../TextOutputs/heunstokes1.txt",
                           header = TRUE,
                           sep = ",",
                           dec = ".")
heunstokes10 <- read.table("../TextOutputs/heunstokes10.txt",
                          header = TRUE,
                          sep = ",",
                          dec = ".")
heunstokes100 <- read.table("../TextOutputs/heunstokes100.txt",
                          header = TRUE,
                          sep = ",",
                          dec = ".")

plot(x = 1,                 
     xlab = "Weite", 
     ylab = "Höhe",
     xlim = c(0, 420000), 
     ylim = c(0, 102000),
     main = "Heun mit Reibung",
     type = "n")
legend("topright", 
       legend=c("HeunStokes mit 0.1s", "HeunStokes mit 1s", "HeunStokes mit 10s", "HeunStokes mit 100s"), 
       col=c("red", "blue", "pink", "green"), pch=c(1,5,3), cex=0.8)
points(heunstokes01$Ort_x, heunstokes01$Ort_y, col="red")
points(heunstokes1$Ort_x, heunstokes1$Ort_y, pch=5, col="blue")
points(heunstokes10$Ort_x, heunstokes10$Ort_y, pch=3 , col="pink")
points(heunstokes100$Ort_x, heunstokes100$Ort_y, pch=3 , col="green")
