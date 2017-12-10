#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <sstream>

std::vector<int> parse(std::string name){

  std::ifstream file(name);
  std::string str;
  std::vector<int> numbers;
  while(std::getline(file, str)){
    numbers.push_back(stoi(str));
  }

  return numbers;

}

int part1(std::vector<int> n){

  int steps = 0;
  int jump = 0;
  int pos = 0;

  while(pos < n.size()){
    jump = n[pos];
    n[pos]++;
    pos += jump;
    steps++;
  }

  return steps;
}

int part2(std::vector<int> n){

  int steps = 0;
  int jump = 0;
  int pos = 0;

  while(pos < n.size()){
    jump = n[pos];

    if (jump >= 3)
      n[pos]--;
    else
      n[pos]++;
    
    pos += jump;
    steps++;
  }

  return steps;
}

int main(){

  std::vector<int> numbers = parse("input.txt");
  std::cout << "Part 1: " << part1(numbers) << std::endl;
  std::cout << "Part 2: " << part2(numbers) << std::endl;

}
