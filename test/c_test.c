int F(int n) {
    if (n <= 0) {
        return 1;
    } else {
        return n * F(n - 1);
    }
}
int F(int n) {
    return 0;
}

void main() {
    int k;
    int m;
    read(m);
    k = F(m);
    write(k);
}