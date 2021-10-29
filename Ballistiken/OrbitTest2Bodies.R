# 2974947.988,4000.853,-241376171.140,-277097.231,0.000
# 65237235.273,88728.640,-5301150727.212,-6188575.362,0.000
Monat <- read.table("../TextOutputs/Monat.txt",
                        header = TRUE,
                        sep = ",",
                        dec = ".")

plot(x = 1,                 
     xlab = "X in m", 
     ylab = "Y in m",
     xlim = c(-500000000, 500000000), 
     ylim = c(-500000000, 500000000),
     main = "Orbit vom Mond um die Erde",
     type = "n")
abline(h=0)
abline(v=0)
points(Monat$Erde_x, Monat$Erde_y, col="blue")
points(Monat$Mond_x, Monat$Mond_y, col="red")


plot(x = 1,                 
     xlab = "X in m", 
     ylab = "Y in m",
     xlim = c(-50000000, 50000000), 
     ylim = c(-50000000, 50000000),
     main = "Orbit vom Mond um die Erde",
     type = "n")
abline(h=0)
abline(v=0)
points(Monat$Erde_x, Monat$Erde_y, col="blue")
