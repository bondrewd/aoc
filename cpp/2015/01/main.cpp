#include <iostream>
#include <fstream>
#include <streambuf>

template<typename T = char>
int last_floor(std::ifstream& file) {
  int floor{};
  for (std::istreambuf_iterator<T> it{file}, end; it != end; it++) {
    switch (*it) {
      case '(':
        ++floor;
        break;
      case ')':
        --floor;
        break;
    }
  }
  return floor;
}

template<typename T = char>
std::optional<int> first_cross(std::ifstream& file) {
  file.seekg(0);
  int count{};
  int floor{};
  for (std::istreambuf_iterator<T> it{file}, end; it != end; it++) {
    switch (*it) {
      case '(':
        ++floor;
        break;
      case ')':
        --floor;
        break;
    }
    ++count;
    if (floor < 0) return count;
  }
  return std::nullopt;
}

int main() {
  std::ifstream file("data.txt");

  const int ans1 = last_floor<char>(file);
  std::cout << "Answer 1: " << ans1 << std::endl;

  const int ans2 = first_cross(file).value_or(-1);
  std::cout << "Answer 2: " << ans2 << std::endl;

  return 0;
}
