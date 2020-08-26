import java.io.File
import java.io.InputStream

fun calculateFuel(mass: Int): Int {
   return (mass / 3).toInt() - 2
}

fun main() {
   // Read mass from text file and calculate fuel
   val fuelList = mutableListOf<Int>()
   File("../fuel.txt").inputStream().use { it ->
      it.bufferedReader().useLines {
         lines -> lines.forEach{
            fuelList.add(calculateFuel(it.toInt()))
         }
      }
   }

    // Sum up and print
    val sum = fuelList.sum()
    println(sum)
}
