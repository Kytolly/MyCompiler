int F(int n) {
    if (n <= 0) {
        return 1;
    } else {
        return n * F(n - 1);
    }
}

void main() {
    int k;
    read(m);
    k = F(m);
    write(k);
}