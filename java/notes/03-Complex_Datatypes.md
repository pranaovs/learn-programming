# Complex Datatypes

## Lists

`java.util.ArrayList` is a tool used for storing lists.

```java
import java.util.ArrayList;
```

A new list can be created by `ArrayList<Type> list = new ArrayList<>()`
where `Type` is the datatype of the value to be stored in the list.

```java
import java.util.ArrayList;

public class Program {

  public static void main(String[] args) {

    ArrayList<Integer> list = new ArrayList<>();
    list.add(1);
    System.out.println(list.get(0)); // 1

    ArrayList<Double> list = new ArrayList<>();
    list.add(4.2);
    System.out.println(list.get(0)); // 4.2

    ArrayList<Boolean> list = new ArrayList<>();
    list.add(true);
    System.out.println(list.get(0)); // true

    ArrayList<String> list = new ArrayList<>();
    list.add("String is a reference-type variable")

  }
}
```

### Invalid retrieving

```java
publc
```
