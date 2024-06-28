package com.mycompany.app;

import com.mycompany.binarysearch.BinarySearch;

/**
 * Hello world!
 *
 */
public class App {

    public static void main(String[] args) {
        System.out.println("Hello Davirain!");
        // test gcd
        System.out.println(gcd(10, 5));
        // test rank
        int[] a = { 1, 2, 3, 4, 5, 6, 7, 8, 9 };
        System.out.println(BinarySearch.rank(5, a));
    }

    /// gcd algorithm
    public static int gcd(int p, int q) {
        if (q == 0) {
            return p;
        } else {
            return gcd(q, p % q);
        }
    }
}
