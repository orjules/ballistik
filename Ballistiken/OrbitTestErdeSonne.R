Jahr <- read.table("../TextOutputs/Jahr.txt",
                    header = TRUE,
                    sep = ",",
                    dec = ".")

plot(x = 1,                 
     xlab = "X in m", 
     ylab = "Y in m",
     xlim = c(-200000000000, 200000000000), 
     ylim = c(-200000000000, 200000000000),
     main = "Orbit von der Erde um die Sonne",
     type = "n")
abline(h=0)
abline(v=0)
points(Jahr$Erde_x, Jahr$Erde_y, col="blue")
points(Jahr$Sonne_x, Jahr$Sonne_y, col="red")

plot(x = 1,                 
     xlab = "X in m", 
     ylab = "Y in m",
     xlim = c(-10000000, 10000000), 
     ylim = c(-10000000, 10000000),
     main = "Nur die Sonne",
     type = "n")
abline(h=0)
abline(v=0)
points(Jahr$Sonne_x, Jahr$Sonne_y, col="red")

