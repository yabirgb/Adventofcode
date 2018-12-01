
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <sstream>
#include <algorithm>
#include <iterator>
void spin(std::vector<char> &input, int amount){

  int i = 0;
  char to_insert;
  for(i; i < amount; i++){
    to_insert = input.back();
    input.pop_back();
    input.insert(input.begin(), to_insert);
  }
  
}

void exchange(std::vector<char> &input, int posA, int posB){
  char atposA = input.at(posA);
  char atposB = input.at(posB);

  input[posB] = atposA;
  input[posA] = atposB;
}

void partner(std::vector<char> &input, char proA, char proB){
  std::vector<char>::iterator a = std::find(input.begin(), input.end(), proA);
  std::vector<char>::iterator b = std::find(input.begin(), input.end(), proB);

  std::iter_swap(a,b);
}


int main(){

  std::vector<char> letters;

  //Fill the vector 113
  for (int i=97; i<113; i++){
    char temp = i;
    letters.push_back(temp);
  }
  
  //Manage the file

  std::ifstream file("input.txt");
  std::string str;


  while (std::getline(file, str, ',')){
    //std::cout << str[0] <<std::endl;
    if(str[0] == 's'){
      //std::cout << "s" << std::endl;
      std::string num;
      int amount;
      if (str.length() == 3){
	num = str.substr(1,2);
	amount = std::stoi(num);
      }
      else{
	num = str.substr (1,1);
	amount = std::stoi(num);
      }

      spin(letters, amount);
    }else if(str[0] == 'x'){
      //std::cout << "x" << std::endl;
      std::string num1, num2;
      int first, last, pos;
      
      auto fpos = std::find(str.begin(), str.end(), '/');
      pos = std::distance(str.begin(), fpos);
      
      first = std::stoi(str.substr(1,pos -1));
      last = std::stoi(str.substr(pos+1));

      exchange(letters, first, last);
    }else if(str[0] == 'p'){
      //std::cout << "p" << std::endl;
      partner(letters, str[1], str[3]);
    }

    
  }

  //Show the result
  std::cout << "myvector contains: ";
  for (std::vector<char>::iterator it=letters.begin(); it!=letters.end(); ++it)
    std::cout << *it;

  std::cout << '\n';

  return 0;
  
}
