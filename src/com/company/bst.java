package com.company;


import java.util.LinkedList;
import java.util.Queue;


class Node {
    int key;
    Node left;
    Node right;
    Node p;

    Node(int value) {
        key = value;
        left = null;
        right = null;
    }
}


public class bst {

    private Node root;

    bst() {
        root = null;
    }

    private void printTreeRec(Node root, int depth) {
        if(root != null) {
            System.out.println(root.key);
            printTreeRec(root.left, depth + 1);
            printTreeRec(root.right, depth + 1);
        }
    }

    void bfs() {
        levelOrderTraversal(root);
    }

    void insert(int value) {
        root = insertRec(root, null, value);
        System.out.println("Debug0..." + root.key);

    }

    private Node insertRec(Node current, Node prev, int value) {
        if(current == null) {
            current = new Node(value);
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

    void printTree() {
        printTreeRec(root, 0);
    }

    private void levelOrderTraversal(Node root) {
        Queue<Node> q = new LinkedList<Node>();
        q.add(root);
        while (!q.isEmpty()) {
            int size = q.size();

            while (size > 0) {
                Node a = q.remove();
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

    void transplant (Node u, Node v) {
        if(u.p == root) root = v;
        else if (u == u.p.left) {
            u.p.left = v;
        }
        else if(u == u.p.right) {
            u.p.right = v;
        }

        if(v != null) {
            v.p = u.p;
        }
    }

    Node Search(int key) {
        return SearchRec(root, key);
    }

    Node SearchRec(Node root, int key) {
        if(root != null) {
            if(root.key == key) return root;
            else if(root.key < key) return SearchRec(root.right, key);
            else return SearchRec(root.left, key);
        }
        return null;
    }

    void delete(Node x) {
        if(x.right == null) {
            transplant(x, x.left);
        }

        else if(x.left == null){
            transplant(x, x.right);
        }

        else {
            Node x_successor = minimum(x.right);
            if(x_successor.p != x) {
                transplant(x_successor, x_successor.right);
                x_successor.right = x.right;
                x_successor.right.p = x_successor;
            }
            transplant(x, x_successor);
            x_successor.left = x.left;
            x.left.p = x_successor;
        }

    }

    Node minimum(Node root) {
        Node prev = null;
        while (root != null) {
            prev = root;
            root = root.left;
        }
        return prev;
    }

    Node maximum(Node root) {
        Node prev = null;
        while (root != null) {
            prev = root;
            root = root.right;
        }
        return prev;
    }

    Node successor(Node x) {
        if(x == null) return null;
        if(x.right != null) {
            return minimum(x.right);
        }
        else {
            Node y = x.p;
            while(y!= null && x == y.right) {
                x = y;
                y = y.p;
            }
            return y;
        }
    }


    public static void main(String[] Args) {
        bst tree = new bst();

        tree.insert(25);
        tree.insert(15);
        tree.insert(35);
        tree.insert(7);
        tree.insert(18);
        tree.insert(33);
        tree.insert(36);

        tree.bfs();
        System.out.println(tree.successor(tree.Search(36)).key);
        tree.bfs();
    }
}
