algorithms
==========

Algorithmen verschiedenster Art zum Training in verschiedenen Programmiersprachen.

* Rust
* C
* Python

bubble_sort
-----------

Der wohl berühmteste Algorithmus: Bubble Sort. 

i32_to_bcd
----------

i32_to_bcd ermöglicht es eine Dezimalzahl beliebiger Größe (in den Grenzen eines signed integers) in BCD-Code zu konvertieren. 

* TODO: Prüfen ob i32_to_bcd auch negative Zahlen umwandelt

Der Algorithmus berechnet als erstes die Stellenanzahl der Zahl. Dazu wird die Zahl so oft durch ein 
Vielfaches von 10 (10, 100, 1000, ...) geteilt, bis der Quotient kleiner 1 ist. Dann wird von dem Vielfachen der
aktuellen Iteration noch einmal durch 10 geteilt, damit mit dem Vielfachen der vorherigen Iteration weiter gerechnet wird. 

Als nächstes werden die Stellen der Reihe nach bis zur ersten ausgewertet und das korrespondierende BCD-Nibble geschrieben.

Ein Beispiel mit der Zahl 538:

```
538 / 10 >= 1
538 / 100 >= 1
538 / 1000 < 1

=> 100

538 / 100 = 5,38 -> 5
5 => 0101

BCD: 0101 0000 0000

538 - 500 = 38
38 / 10 = 3,8 -> 3
3 => 0011

BCD: 0101 0011 0000

38 - 30 = 8
8 => 1000

BCD: 0101 0011 1000

Der dezimale BCD-Wert für 538 ist:
1024 + 256 + 32 + 16 + 8 = 1336
```
