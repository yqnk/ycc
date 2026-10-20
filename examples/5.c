int main() {
  int a = 8;
  int b = 0;

  if (a == 8) {
    b = 16;
    b += a;
  } else {
    b = 67;
    b += 2;
  }

  return b;
}
