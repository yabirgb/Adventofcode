#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <sstream>

int maxmin(std::string line){

  std::stringstream ss(line);
  std::string number;

  std::vector<int> row;


  int max, min;
  
  while (getline(ss, number, '\t')){
    row.push_back(stoi(number));
  }

  
  if (row.size() == 0)
    return 0;

  max = row[0];
  min = row[0];

  for(std::vector<int>::const_iterator it=row.cbegin(); it != row.cend(); it++){

    if(*it > max)
      max = *it;
    else if(*it < min)
      min = *it;
  }

  return max - min;
}

int divides(std::string line){

  std::stringstream ss(line);
  std::string number;

  std::vector<int> row;
  
  while (getline(ss, number, '\t')){
    row.push_back(stoi(number));
  }

  
  if (row.size() == 0)
    return 0;

  std::vector<int>::const_iterator start = row.cbegin();
  
  for (std::vector<int>::const_iterator pos = row.cbegin(); pos != row.cend(); pos++){
    for(std::vector<int>::const_iterator it=row.cbegin(); it != row.cend(); it++){
      if(*pos % *it == 0 && it - pos != 0)
	return *pos / *it;
    }
  }
}

int readAndCount(std::string name, int f(std::string)){

  std::ifstream file(name);
  std::string str;

  int counter = 0;
  
  while (std::getline(file, str)){
    counter += f(str);
  }

  return counter;

}

int main(){

  int result1 = readAndCount("input.txt", maxmin);
  int result2 = readAndCount("input.txt", divides);

  std::cout << "Solution to part 1: " << result1 << std::endl;
  std::cout << "Solution to part 1: " << result2 << std::endl;
}
