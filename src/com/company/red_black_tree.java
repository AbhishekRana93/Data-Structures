package com.company;

import java.util.LinkedList;
import java.util.Queue;

enum Color {
    Red, Black;
}

class RedBlackNode {
    int key;
    RedBlackNode left;
    RedBlackNode right;
    RedBlackNode p;
    Color color;

    RedBlackNode(int value) {
        key = value;
        left = null;
        right = null;
        p = null;
        color = Color.Black;
    }
}

public class red_black_tree {

    RedBlackNode root;
    RedBlackNode Nil;

    red_black_tree() {
        root = null;
        Nil = new RedBlackNode(0);
        Nil.color = Color.Black;
    }

    private void levelOrderTraversal(RedBlackNode root) {
        Queue<RedBlackNode> q = new LinkedList<RedBlackNode>();
        q.add(root);
        while (!q.isEmpty()) {
            int size = q.size();

            while (size > 0) {
                RedBlackNode a = q.remove();
                System.out.print(a.key + " ");
                if (a.left != null)
                    q.add(a.left);
                if (a.right != null)
                    q.add(a.right);

                size--;
            }

            System.out.println();
        }
    }

    void bfs() {
        levelOrderTraversal(root);
    }


    //insert a node
    void insert(int value) {
//        root = insertRec(root, null, value);
//        root = insertRec(root, null, 15);
//        root = insertRec(root, null, 6);
//        root = insertRec(root, null, 5);
//        root = insertRec(root, null, 3);
//        root = insertRec(root, null, 4);
        RedBlackNode prev = null;
        RedBlackNode current = root;
        RedBlackNode newNode = new RedBlackNode(value);

        while(current != null) {
            prev = current;
            if(current.key < value) current = current.right;
            else current = current.left;
        }

        if(prev == null) {
            root = newNode;
            newNode.p = Nil;
        }
        else if(prev.key < value) {
            prev.right = newNode;
            newNode.p = prev.right;
        }
        else {
            prev.left = newNode;
            newNode.p = prev.left;
        }

        newNode.left = Nil;
        newNode.right = Nil;
        newNode.color = Color.Red;
        insertFixup(newNode);
    }

    void insertFixup(RedBlackNode z) {
        while (z.p.color == Color.Red) {
            if(z.p == z.p.p.left) {
                RedBlackNode y = z.p.p.right;
                if(y.color == Color.Red) {
                    y.color = Color.Black;
                    z.p.p.color = Color.Red;
                    z.p.color = Color.Black;
                }
                else if(z == z.p.right) {
                    z = z.p;
                    //Left-Rotate(root, z)
                }

                z.p.p.color = Color.Red;
                z.p.color = Color.Black;
            }
        }
    }

    /*
    RedBlackNode insertRec(RedBlackNode current, RedBlackNode prev, int value) {
        if(current == null) {
            current = new RedBlackNode(value);
            current.p = prev;
        }
        else if(current.key < value) {
            current.right = insertRec(current.right, current, value);
        }
        else {
            current.left = insertRec(current.left, current, value);
        }
        return current;
    }
    */

    public static void main(String[] Args) {
        red_black_tree rbtree = new red_black_tree();
        rbtree.insert(7);
//        System.out.println(rbtree.root.key);
        rbtree.bfs();
    }
}


