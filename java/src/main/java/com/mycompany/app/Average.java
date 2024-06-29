package com.mycompany.app;

import com.mycompany.standrard.StdIn;
import com.mycompany.standrard.StdOut;

public class Average {
    public static void main(String[] args) {
        //
        double sum = 0.0;
        int cnt = 0;
        while (!StdIn.isEmpty()) {
            sum += StdIn.readDouble();
            cnt += 1;
        }
        double avg = sum / cnt;
        StdOut.printf("Average is %.5f\n", avg);
    }
}
