# 算法

## chapter1: basic

算法的定义：一种有限、确定、有效的并适合用计算机程序来实现的解决问题的方法。

算法是计算机科学的基础，是这个领域的核心。算法是一种解决问题的方法，是一种思维活动，是一种设计和分析的技术。

欧几里得算法

自然语言描述： 计算两个非负整数p和q的最大公约数：若q是0，则最大公约数为p。否则，将p除以q得到余数r，p和q的最大公约数即为q和r的最大公约数。

```java
public static int gcd(int p, int q) {
    if (q == 0) return p;
    int r = p % q;
    return gcd(q, r);
}
```

### 1.1 基础编程模型

#### 1.1.1 二分查找

二分查找是一种在有序数组中查找某一特定元素的搜索算法。搜索过程从数组的中间元素开始，如果中间元素正好是要查找的元素，则搜索过程结束；如果某一特定元素大于或者小于中间元素，则在数组大于或小于中间元素的那一半中查找，而且跟开始一样从中间元素开始比较。如果在某一步骤数组为空，则代表找不到。

```java
public class BinarySearch {

    public static void main(String[] args) {
        int[] whitelist = In.readInts(args[0]);

        Arrays.sort(whitelist);

        while (!StdIn.isEmpty()) {
            int key = StdIn.readInt();
            if (rank(key, whitelist) == -1) StdOut.println(key);
        }
    }

    public static int rank(int key, int[] a) {
        int lo = 0;
        int hi = a.length - 1;
        while (lo <= hi) {
            // Key is in a[lo..hi] or not present.
            int mid = lo + (hi - lo) / 2;
            if (key < a[mid]) {
                hi = mid - 1;
            } else if (key > a[mid]) {
                lo = mid + 1;
            } else {
                return mid;
            }
        }
        return -1;
    }
}
```

#### 1.1.2 原始数据类型与表达式

需要注意的是类型转换，例如：

```java
(int)3.7 == 3
(double)3 == 3.0
```

下面是Rust语言中的类型转换：

```rust
fn main() {
    let x = 3.0 as i32;
    println!("{}", x); // 3
}
```

#### 1.1.5 数组

数组的初始化

```java
double a[] = new double[N];

int[] a = {1, 1, 3, 4, 6, 9};
```

一些典型的数组处理的代码片段：

```java
// 计算数组元素的平均值
public static double average(double[] a) {
    int N = a.length;
    double sum = 0.0;
    for (int i = 0; i < N; i++) {
        sum += a[i];
    }
    return sum / N;
}

// 复制数组
public static double[] copy(double[] a) {
    int N = a.length;
    double[] b = new double[N];
    for (int i = 0; i < N; i++) {
        b[i] = a[i];
    }
    return b;
}

// 颠倒数组元素的顺序
public static void reverse(double[] a) {
    int N = a.length;
    for (int i = 0; i < N / 2; i++) {
        double temp = a[i];
        a[i] = a[N - 1 - i];
        a[N - 1 - i] = temp;
    }
}

// 找出数组中最大的元素
public static double max(double[] a) {
    int N = a.length;
    double max = a[0];
    for (int i = 1; i < N; i++) {
        if (a[i] > max) {
            max = a[i];
        }
    }
    return max;
}

// 矩阵相乘
// a[][] * b[][] = c[][]
public static double[][] matrixMult(double[][] a, double[][] b) {
    int N = a.length;
    double[][] c = new double[N][N];
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            for (int k = 0; k < N; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    return c;
}
```

#### 1.5.4 java中的数组别名

```java
int[] a = new int[10];

a[i] = 1234;

int[] b = a;

b[i] = 5678; // a[i] == 5678
```

#### 1.1.6 静态方法

静态方法是不需要创建对象就可以调用的方法。静态方法在声明时使用关键字static，静态方法不能访问非静态的成员变量和方法。

```java
public static double sqrt(double c) {
    if (c < 0) return Double.NaN;
    double err = 1e-15;
    double t = c;
    while (Math.abs(t - c / t) > err * t) {
        t = (c / t + t) / 2.0;
    }
    return t;
}
```

典型的静态方法的实现

```java
// 计算一个整数的绝对值
public static int abs(int x) {
    if (x < 0) return -x;
    else return x;
}

// 计算一个浮点数的绝对值
public static double abs(double x) {
    if (x < 0.0) return -x;
    else return x;
}

// 判断一个数是否是素数
public static boolean isPrime(int N) {
    if (N < 2) return false;
    for (int i = 2; i * i <= N; i++) {
        if (N % i == 0) return false;
    }
    return true;
}

// 计算平方根
public static double sqrt(double c) {
    if (c < 0) return Double.NaN;
    double err = 1e-15;
    double t = c;
    while (Math.abs(t - c / t) > err * t) {
        t = (c / t + t) / 2.0;
    }
    return t;
}

// 计算直角三角形的斜边
public static double hypotenuse(double a, double b) {
    return Math.sqrt(a * a + b * b);
}

// 计算调和级数
public static double H(int N) {
    double sum = 0.0;
    for (int i = 1; i <= N; i++) {
        sum += 1.0 / i;
    }
    return sum;
}
```

#### 1.1.7 递归

递归是一种解决问题的方法，它把一个问题分解为越来越小的子问题。递归的三个重要特征：

1. 递归调用是方法调用自身。
2. 递归调用必须有一个最简单的情况作为方法的返回条件。
3. 递归调用的过程中，每次调用之间的参数都应有所变化，以缩小问题的规模。

```java
public static int rank(int key, int[] a) {
    return rank(key, a, 0, a.length - 1);
}

public static int rank(int key, int[] a, int lo, int hi) {
    // 如果key存在于a[]中，它的索引不会小于lo且不会大于hi
    if (lo > hi) return -1;
    int mid = lo + (hi - lo) / 2;
    if (key < a[mid]) return rank(key, a, lo, mid - 1);
    else if (key > a[mid]) return rank(key, a, mid + 1, hi);
    else return mid;
}
```
