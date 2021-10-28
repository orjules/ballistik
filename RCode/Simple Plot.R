# Daten für die folgenden Plots einholen
here("TextOutputs", "v100100s1.txt")
points <- read.table(here("TextOutputs", "v100100s1.txt"),
                     header = TRUE,
                     sep = ",",
                     dec = ".")

# v100100s05
# /Users/julius/SynologyDrive/Uni/Semester 5/SSE/ballistik

# Plot des Ortes in X und Y
plot(x = 1,                 
     xlab = "Weite", 
     ylab = "Höhe",
     xlim = c(0, 2500), 
     ylim = c(0, 600),
     main = "Simpler Plott",
     type = "n")

points(points$Ort_x, points$Ort_y)

# Plot der Geschwindigkeit relativ zueinander und relativ zur Zeit
plot(x = 1,                 
     xlab = "Geschwindigkeit X", 
     ylab = "Geschwindigkeit Y",
     xlim = c(0, 150), 
     ylim = c(0, 150),
     main = "Simpler Plott",
     type = "n")
points(points$Geschw_x, points$Geschw_y)

plot(x = 1,                 
     xlab = "Zeit", 
     ylab = "Geschwindigkeit X bzw Y",
     xlim = c(0, 25), 
     ylim = c(-110, 110),
     main = "Simpler Plott",
     type = "n")
abline(h=0)
points(points$Zeit, points$Geschw_x)
points(points$Zeit, points$Geschw_y)
