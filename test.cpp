#include <vector>
#include <stdio.h>

void print(std::vector<int>& a) {
    for (const auto& item : a) {
        printf("d", item);
    }
    printf("\n");
}

int main() {
    std::vector<int> a;
    std::vector<int> b = a;

    b.push_back(1);

    printf("a");
    print(a);
    printf("b");
    print(b);
}