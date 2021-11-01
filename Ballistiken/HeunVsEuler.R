heun <- read.table("../TextOutputs/heun.txt",
                        header = TRUE,
                        sep = ",",
                        dec = ".")
euler <- read.table("../TextOutputs/euler.txt",
                   header = TRUE,
                   sep = ",",
                   dec = ".")
heun05 <- read.table("../TextOutputs/heun05.txt",
                   header = TRUE,
                   sep = ",",
                   dec = ".")
euler05 <- read.table("../TextOutputs/euler05.txt",
                    header = TRUE,
                    sep = ",",
                    dec = ".")


plot(x = 1,                 
     xlab = "Weite", 
     ylab = "Höhe",
     xlim = c(0, 2500), 
     ylim = c(0, 600),
     main = "Heun vs Euler",
     type = "n")
legend("topright", 
       legend=c("Heun mit 1s", "Heun mit 0.5s", 
                "Euler mit 1s", "Euler mit 0.5s"), 
       col=c("red", "red", "blue", "blue"), pch=c(1,5,1,5), cex=0.8)
points(heun$Ort_x, heun$Ort_y, col="red")
points(euler$Ort_x, euler$Ort_y, col="blue")
points(heun05$Ort_x, heun05$Ort_y, pch=5 , col="red")
points(euler05$Ort_x, euler05$Ort_y, pch=5, col="blue")
