int main() {
    int x = 0;
    int r = 0;

    while (x < 8) {
        x = x + 1;

        if (x % 2 == 0)
            r += 7;
    }

    return r;
}
