# Daten für die folgenden Plots einholen
v100100s1 <- read.table("../TextOutputs/v100100s1.txt",
                     header = TRUE,
                     sep = ",",
                     dec = ".")
v100100s06 <- read.table("../TextOutputs/v100100s06.txt",
                     header = TRUE,
                     sep = ",",
                     dec = ".")

v80100s06 <- read.table("../TextOutputs/v80100s06.txt",
                     header = TRUE,
                     sep = ",",
                     dec = ".")

# Plot des Ortes in X und Y
plot(x = 1,                 
     xlab = "Weite", 
     ylab = "Höhe",
     xlim = c(0, 2500), 
     ylim = c(0, 600),
     main = "Ortsänderung",
     type = "n")

points(v100100s1$Ort_x, v100100s1$Ort_y, col="blue")
points(v100100s06$Ort_x, v100100s06$Ort_y, col="red")
points(v80100s06$Ort_x, v80100s06$Ort_y, col="purple")

# Plot der Geschwindigkeiten relativ zur Zeit
plot(x = 1,                 
     xlab = "Zeit", 
     ylab = "Geschwindigkeit X bzw Y",
     xlim = c(0, 25), 
     ylim = c(-110, 110),
     main = "Geschwindigkeiten über Zeit",
     type = "n")
abline(h=0)
points(v100100s1$Zeit, v100100s1$Geschw_x, col="blue")
points(v100100s1$Zeit, v100100s1$Geschw_y, col="blue")
points(v100100s06$Zeit, v100100s06$Geschw_x, col="red")
points(v100100s06$Zeit, v100100s06$Geschw_y, col="red")
points(v80100s06$Zeit, v80100s06$Geschw_x, col="purple")
points(v80100s06$Zeit, v80100s06$Geschw_y, col="purple")

