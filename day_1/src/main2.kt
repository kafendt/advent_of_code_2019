import java.io.File

fun calculateFuel(mass: Int, accumulated: Int): Int {
    var fuel = (mass / 3).toInt() - 2
    if (fuel < 0) {
        fuel = 0
    }

    val combinedFuel = fuel + accumulated
    return if (fuel > 0){
        calculateFuel(fuel, combinedFuel)
    }
    else {
        combinedFuel
    }
}

fun main() {
   // Read mass from text file and calculate fuel
   val fuelList = mutableListOf<Int>()
   File("../fuel.txt").inputStream().use { it ->
      it.bufferedReader().useLines {
         lines -> lines.forEach{
            fuelList.add(calculateFuel(it.toInt(), 0))
         }
      }
   }

    // Sum up and print
    val sum = fuelList.sum()
    println(sum)
}
