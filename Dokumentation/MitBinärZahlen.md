# Übertragen in Binärdarstellung

Mein nächster Schritt ist nun den Algorithmus zu übertragen, in eine Version die aus den zur Verfügung stehenden 
Bauteilen besteht und mit 32 Bit Fixed Point Zahlen funktioniert.


## Definition des Punktes

Meine kleinste Zahl ist die dynamische Viskosität von Luft bei 20 Grad Celsius. 
Diese ist 18,21548 * 10^-6 also 0,00001821548 was in binär 0,0000_0000_0000_0001_0011_00011_0011_0101 ist.
Dabei sind schon alle 32 Bit als Nachkommastellen verwendet. Wenn ich das Komma in die Mitte tun würde also 
16 Bit,16 Bit wäre das 0000_0000_0000_0000,0000_0000_0000_0001. In Dezimal sind das 0,000015258789062.
Das ist ein Fehler von 0.0000029121094.

Die Zahlen sollten bis in die ca 100.0000 gehen können, weil ich in Metern arbeite und das wären 100km.
100.000 in Binär sind 1_1000_0110_1010_0000, was 17 Bit sind.

Problem ist nun, dass beides gleichzeitig unmöglich ist.

Da ich die Vsikosität immer mit 6 und pi multipliziere kann ich es gleich als 0.0003430619178 binär sind das
0,0000_0000_0001_0110_0111 (0,000342369079589) bzw nur 0,0000_0000_0001_0110 (0,000335693359375)

Die größte Zahl mit 16 Bit vor dem Komma ist 65.535. Die größte Zahl mit 12 Bit vor dem Komma ist 4.095. Erstere ist 
ein akzeptables Maximum. Da die großen Zahlen wichtiger sind als die Genauigkeit der Nachkommestellen, 
will ich es als 16 Bit, 16 Bit aufteilen. Ein zusätzlicher Vorteil ist die Einfachheit bei der weiteren Planung, weil
die 32 Bit einfach halbiert werden müssen


## Nutzung von Minus

Die 32 Bit fixed Point Zahlen sollen kein einziges Bit für sign hergeben, da es nur an wenigen stellen wichtig wird.
Bei der Addition gibt es drei Fälle:
- Beide Zahlen sind positiv => sign_flag = 0
- Eine Zahl ist negativ => sign_flag = 1 (aufpassen, erst positiv, dann negativ)
- Beide Zahlen sind negativ => sign_flag = 0 aber das sign_flag übernehmen

Diese Fallunterscheidung ist potentiell ein Problem, weil es Komplexität mit sich bringt.
Die Frage ist nun ob diese Komplexität weniger ist als das ganze mit Floating Point zahlen zu machen, was seine eigene
Komplexität bringt.