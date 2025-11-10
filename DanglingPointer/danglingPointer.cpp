#include <iostream>
using namespace std;

int* createNumber() {
    int x = 42;
    return &x; //gibt Pointer auf lokale Variable zurück (dangling pointer)
}

int main() {
    int* ptr = createNumber();
    cout << "Hello World!" << endl;
    cout << *ptr << endl; //Undefiniertes Verhalten
}
