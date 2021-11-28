finalplot <- read.table("../TextOutputs/finalplot.txt",
                            header = TRUE,
                            sep = ",",
                            dec = ".")

plot(x = 1,                 
     xlab = "Weite in m", 
     ylab = "Höhe in m",
     xlim = c(0, 210000), 
     ylim = c(0, 55000),
     main = "Optimierte Version mit Reibung",
     sub = "Startwerte: v = 1000,1000 dt = 5",
     type = "n")
points(finalplot$Ort_x, finalplot$Ort_y, col="red")