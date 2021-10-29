v1T1Ts5 <- read.table("../TextOutputs/v1T1Ts5.txt",
                        header = TRUE,
                        sep = ",",
                        dec = ".")

v1T1Tr1s5 <- read.table("../TextOutputs/v1T1Tr1s5.txt",
                          header = TRUE,
                          sep = ",",
                          dec = ".")
v100100s05 <- read.table("../TextOutputs/v100100s05.txt",
                       header = TRUE,
                       sep = ",",
                       dec = ".")

v100100r1s05 <- read.table("../TextOutputs/v100100r1s05.txt",
                         header = TRUE,
                         sep = ",",
                         dec = ".")

# Für 1000 

plot(x = 1,                 
     xlab = "Weite in m", 
     ylab = "Höhe in m",
     xlim = c(0, 210000), 
     ylim = c(0, 60000),
     main = "Ortsänderung",
     sub = "Startwerte: v = 1000,1000 dt = 5",
     type = "n")
legend("topleft", legend=c("Mit Reibung", "Ohne Reibung"), 
       col=c("red", "blue"), pch=c(1,1), cex=0.8)

points(v1T1Ts5$Ort_x, v1T1Ts5$Ort_y, col="blue")
points(v1T1Tr1s5$Ort_x, v1T1Tr1s5$Ort_y, col="red")


# Für 100
plot(x = 1,                 
     xlab = "Weite in m", 
     ylab = "Höhe in m",
     xlim = c(0, 2100), 
     ylim = c(0, 600),
     main = "Ortsänderung",
     sub = "Startwerte: v = 100,100 dt = 0.5",
     type = "n")
legend("topleft", legend=c("Mit Reibung", "Ohne Reibung"),
       col=c("red", "blue"), pch=c(1,1), cex=0.8)

points(v100100s05$Ort_x, v100100s05$Ort_y, col="blue")
points(v100100r1s05$Ort_x, v100100r1s05$Ort_y, col="red")

# Für ZeitxGeschw für 1000
plot(x = 1,                 
     xlab = "Zeit in s", 
     ylab = "Geschwindigkeit X,Y in m/s",
     xlim = c(0, 250), 
     ylim = c(-1100, 1100),
     main = "Geschwindigkeiten über Zeit",
     sub = "Startwerte: v = 1000,1000 dt = 5",
     type = "n")
abline(h=0)
legend("bottomleft", legend=c("x mit Reibung", "x ohne Reibung", 
                              "y mit Reibung", "y ohne Reibung"),
       col=c("red", "blue", "red", "blue"), pch=c(1,1,2,2), cex=0.8)

points(v1T1Ts5$Zeit, v1T1Ts5$Geschw_x, col="blue")
points(v1T1Ts5$Zeit, v1T1Ts5$Geschw_y, pch=2, col="blue")
points(v1T1Tr1s5$Zeit, v1T1Tr1s5$Geschw_x, col="red")
points(v1T1Tr1s5$Zeit, v1T1Tr1s5$Geschw_y, pch=2, col="red")


# Für ZeitxGeschw für 100
plot(x = 1,                 
     xlab = "Zeit in s", 
     ylab = "Geschwindigkeit X,Y in m/s",
     xlim = c(0, 25), 
     ylim = c(-110, 110),
     main = "Geschwindigkeiten über Zeit",
     sub = "Startwerte: v = 100,100 dt = 0.5",
     type = "n")
abline(h=0)
legend("bottomleft", legend=c("x mit Reibung", "x ohne Reibung", 
                              "y mit Reibung", "y ohne Reibung"),
       col=c("red", "blue", "red", "blue"), pch=c(1,1,2,2), cex=0.8)

points(v100100s05$Zeit, v100100s05$Geschw_x, col="blue")
points(v100100s05$Zeit, v100100s05$Geschw_y, pch=2, col="blue")
points(v100100r1s05$Zeit, v100100r1s05$Geschw_x, col="red")
points(v100100r1s05$Zeit, v100100r1s05$Geschw_y, pch=2, col="red")
