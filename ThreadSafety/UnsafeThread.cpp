#include <iostream>
#include <thread>

int main() {
    int counter = 0;

    std::thread t1([&]() {
        for (int i = 0; i < 1000000; ++i)
            counter++;
    });

    std::thread t2([&]() {
        for (int i = 0; i < 1000000; ++i)
            counter++;
    });

    t1.join();
    t2.join();

    std::cout << "Counter: " << counter << std::endl;
}
