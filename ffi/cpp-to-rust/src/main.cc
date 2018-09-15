#include <iostream>

extern "C"{
int add_one(int input);
}

using namespace std;

int main() {
  int input = 2;
  int output = add_one(input);
  cout << input << " + 1 = "<< output << endl;

  return 0;
}
