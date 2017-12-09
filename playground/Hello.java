package org.typeserve.hello;

public class Hello {

    public int add(int x, int y) {
        return x + y;
    }

    public static void main(String[] args) {
        Hello hello = new Hello();

        int a = hello.add(1, 2);
    }

}