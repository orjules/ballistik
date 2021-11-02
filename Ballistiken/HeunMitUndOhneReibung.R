heunUndStokes <- read.table("../TextOutputs/heunUndStokes.txt",
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
     main = "Heun mit und ohne Reibung",
     type = "n")
legend("topright", 
       legend=c("Heun ohne Reibung", "Heun mit Reibung"), 
       col=c("red", "blue"), pch=c(1,1), cex=0.8)
points(nurHeun$Ort_x, nurHeun$Ort_y, col="red")
points(heunUndStokes$Ort_x, heunUndStokes$Ort_y, col="blue")
