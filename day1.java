import java.util.*;
import java.io.*;

class day1 {
  public static void main(String args[]) throws Exception {
    Scanner sc = new Scanner(new FileInputStream("day1.txt"));
    int sum = 0;
    int[] sMax = new int[3];
    ArrayList<Integer> l = new ArrayList<Integer>();
    while (sc.hasNext()) {
      String line = sc.nextLine();
      if (line.length() != 0) {
        int i = Integer.parseInt(line);
        sum += i;
      } else {
        if (sum > sMax[0]) {
          sMax[0] = sum;
        } else if (sum > sMax[1]) {
          sMax[1] = sum;
        } else if (sum > sMax[2]) {
          sMax[2] = sum;
        }
        l.add(sum);
        sum = 0;
      }
    }
    l.sort(new Comparator<Integer>() {
      public int compare(Integer a, Integer b) {
        return b - a;
      }
    });
    System.out.println(l.remove(0));
    System.out.println(l.remove(0));
    System.out.println(l.remove(0));
    System.out.println(sMax[0]);
    System.out.println(sMax[1]);
    System.out.println(sMax[2]);
    System.out.println(sMax[0] + sMax[1] + sMax[2]);
  }
}
